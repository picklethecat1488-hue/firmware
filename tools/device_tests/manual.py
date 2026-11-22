"""Pytest-native manual benchtop verification helper with `host.py` heartbeats and UI response sync."""

import asyncio
from dataclasses import dataclass
from typing import Any, Callable, Dict, NoReturn, Optional
import sys
import uuid

from device_tests.client import AsyncHostSessionClient
from host_daemon.model.manual_prompt import (
    ManualPromptAction,
    ManualPromptKind,
    parse_measurement_string,
)

DEFAULT_PROMPT_HEARTBEAT_INTERVAL_SEC: float = 0.25


@dataclass(frozen=True)
class ManualPromptResult:
    """Outcome of an operator prompt (`ask_yes_no` or `read_measurement`) inside a manual device test."""

    action: ManualPromptAction
    passed: bool
    measurement: Optional[float] = None
    raw_value: str = ""
    comment: str = ""


def _parse_cancel_input(raw_text: str) -> Optional[str]:
    """Return the cancellation comment (possibly empty if omitted) when `raw_text` requests cancel, else None."""
    stripped = raw_text.strip()
    lowered = stripped.lower()
    if lowered in ("c", "cancel"):
        return ""
    for prefix in ("cancel:", "cancel ", "c:", "c "):
        if lowered.startswith(prefix):
            return stripped[len(prefix) :].strip()
    return None


class ManualTestHelper:
    """Executes manual benchtop checks inside `pytest` while streaming prompts & syncing UI responses on `host.py`.

    Supports three operator interactions in both terminal (`pytest --manual`) and `host.py` Test Mode UI:
      1. `read_measurement(...)` — read a numeric measured value (e.g. DMM rail voltage or current) to assert in Python.
      2. `ask_yes_no(...)` — answer `YES` or `NO` to a physical inspection step.
      3. `cancel(comment)` — cancel the manual test with a mandatory explanatory comment.
    """

    def __init__(
        self,
        *,
        host: AsyncHostSessionClient,
        test_name: str,
        project: str,
        manual_enabled: bool,
        heartbeat_interval: float = DEFAULT_PROMPT_HEARTBEAT_INTERVAL_SEC,
        prompt_reader: Optional[Callable[[str], str]] = None,
    ) -> None:
        """Initialize manual helper for the active pytest test item."""
        self.host = host
        self.test_name = test_name
        self.project = project
        self.manual_enabled = manual_enabled
        self.heartbeat_interval = max(heartbeat_interval, 0.02)
        self._prompt_reader = prompt_reader

    def _has_terminal_input(self) -> bool:
        """Return True if an explicit reader callback or interactive TTY stdin is available."""
        if self._prompt_reader is not None:
            return True
        return bool(sys.stdin is not None and not sys.stdin.closed and sys.stdin.isatty())

    async def _read_terminal_line(self, prompt_text: str) -> str:
        """Read a single line from the configured `prompt_reader` or interactive `input()`."""
        reader = self._prompt_reader if self._prompt_reader is not None else input
        return await asyncio.to_thread(reader, prompt_text)

    def cancel(self, comment: str) -> NoReturn:
        """Cancel the current manual test with a required operator comment."""
        import pytest

        cleaned = comment.strip()
        if not cleaned:
            raise ValueError("Manual test cancellation requires a non-empty comment")
        pytest.skip(f"Manual test '{self.test_name}' cancelled: {cleaned}")

    async def _await_prompt_resolution(
        self,
        *,
        kind: ManualPromptKind,
        procedure: str,
        prompt: str,
        unit: str,
        terminal_coro_factory: Optional[Callable[[], Any]],
    ) -> Dict[str, Any]:
        """Broadcast heartbeats to `host.py` and resolve as soon as either Web UI or terminal responds."""
        prompt_id = uuid.uuid4().hex[:12]
        stop_event = asyncio.Event()

        async def _ui_heartbeat_waiter() -> Dict[str, Any]:
            while not stop_event.is_set():
                state = await self.host.publish_manual_prompt(
                    prompt_id=prompt_id,
                    kind=kind.value,
                    unit=unit,
                    test_name=self.test_name,
                    project=self.project,
                    procedure=procedure,
                    prompt=prompt,
                    active=True,
                )
                response = state.get("response") if isinstance(state, dict) else None
                if isinstance(response, dict) and response.get("action"):
                    if not response.get("prompt_id") or response.get("prompt_id") == prompt_id:
                        return dict(response)
                try:
                    await asyncio.wait_for(stop_event.wait(), timeout=self.heartbeat_interval)
                except asyncio.TimeoutError:
                    continue
            return {}

        ui_task: asyncio.Task[Dict[str, Any]] = asyncio.create_task(_ui_heartbeat_waiter())
        term_task: Optional[asyncio.Task[Dict[str, Any]]] = (
            asyncio.create_task(terminal_coro_factory()) if terminal_coro_factory is not None else None
        )

        try:
            if term_task is None:
                return await ui_task

            done, _ = await asyncio.wait(
                {ui_task, term_task},
                return_when=asyncio.FIRST_COMPLETED,
            )
            if term_task in done:
                return await term_task
            return await ui_task
        finally:
            stop_event.set()
            if not ui_task.done():
                ui_task.cancel()
            try:
                await ui_task
            except asyncio.CancelledError:
                pass
            if term_task is not None and not term_task.done():
                term_task.cancel()
            await self.host.publish_manual_prompt(
                prompt_id=prompt_id,
                kind=kind.value,
                unit=unit,
                test_name=self.test_name,
                project=self.project,
                procedure=procedure,
                prompt=prompt,
                active=False,
            )

    async def ask_yes_no(
        self,
        *,
        procedure: str,
        prompt: str,
        fail_on_no: bool = True,
    ) -> ManualPromptResult:
        """Prompt the operator to answer `YES`, `NO`, or `CANCEL` with a comment."""
        import pytest

        if not self.manual_enabled:
            pytest.skip(f"Manual step '{self.test_name}' skipped (pass --manual): {prompt}")

        async def _read_yes_no_from_terminal() -> Dict[str, Any]:
            from host_daemon.cli import format_clickable_choice

            yes_choice = format_clickable_choice("Yes", "Y", "yes", "\033[1;32m")
            no_choice = format_clickable_choice("No", "N", "no", "\033[1;31m")
            cancel_choice = format_clickable_choice("Cancel", "C", "cancel", "\033[1;33m")
            rendered = (
                f"\n[MANUAL TEST: {self.test_name} ({self.project})]\n"
                f"  Procedure: {procedure}\n"
                f"  Question:  {prompt}\n"
                f"Answer {yes_choice}, {no_choice}, or {cancel_choice} <comment>: "
            )
            while True:
                raw = (await self._read_terminal_line(rendered)).strip()
                cancel_comment = _parse_cancel_input(raw)
                if cancel_comment is not None:
                    while not cancel_comment:
                        cancel_comment = (await self._read_terminal_line("Cancellation comment (required): ")).strip()
                    return {"action": ManualPromptAction.CANCEL.value, "comment": cancel_comment}

                lowered = raw.lower()
                if lowered in ("y", "yes"):
                    return {"action": ManualPromptAction.YES.value, "comment": ""}
                if lowered in ("n", "no"):
                    return {"action": ManualPromptAction.NO.value, "comment": ""}

        terminal_factory = _read_yes_no_from_terminal if self._has_terminal_input() else None
        resolved = await self._await_prompt_resolution(
            kind=ManualPromptKind.YES_NO,
            procedure=procedure,
            prompt=prompt,
            unit="",
            terminal_coro_factory=terminal_factory,
        )

        action = ManualPromptAction(str(resolved.get("action", ManualPromptAction.NO.value)).lower())
        comment = str(resolved.get("comment") or "").strip()
        if action == ManualPromptAction.CANCEL:
            self.cancel(comment)
        if action == ManualPromptAction.NO:
            if fail_on_no:
                pytest.fail(f"Operator answered NO for manual test '{self.test_name}': {prompt}")
            return ManualPromptResult(action=ManualPromptAction.NO, passed=False, comment=comment)
        return ManualPromptResult(action=ManualPromptAction.YES, passed=True, comment=comment)

    async def read_measurement(
        self,
        *,
        procedure: str,
        prompt: str,
        unit: str = "",
    ) -> ManualPromptResult:
        """Prompt the operator to enter a numeric measured value (or cancel with a comment)."""
        import pytest

        if not self.manual_enabled:
            pytest.skip(f"Manual step '{self.test_name}' skipped (pass --manual): {prompt}")

        unit_label = f" ({unit})" if unit else ""

        async def _read_measurement_from_terminal() -> Dict[str, Any]:
            rendered = (
                f"\n[MANUAL MEASUREMENT: {self.test_name} ({self.project})]\n"
                f"  Procedure: {procedure}\n"
                f"  Measure:   {prompt}{unit_label}\n"
                f"Enter measured value{unit_label} (or 'cancel <comment>'): "
            )
            while True:
                raw = (await self._read_terminal_line(rendered)).strip()
                cancel_comment = _parse_cancel_input(raw)
                if cancel_comment is not None:
                    while not cancel_comment:
                        cancel_comment = (await self._read_terminal_line("Cancellation comment (required): ")).strip()
                    return {"action": ManualPromptAction.CANCEL.value, "comment": cancel_comment}
                try:
                    numeric = parse_measurement_string(raw, unit)
                except ValueError:
                    continue
                return {
                    "action": ManualPromptAction.MEASUREMENT.value,
                    "measurement": numeric,
                    "raw_value": raw,
                    "comment": "",
                }

        terminal_factory = _read_measurement_from_terminal if self._has_terminal_input() else None
        resolved = await self._await_prompt_resolution(
            kind=ManualPromptKind.MEASUREMENT,
            procedure=procedure,
            prompt=prompt,
            unit=unit,
            terminal_coro_factory=terminal_factory,
        )

        action = ManualPromptAction(str(resolved.get("action", ManualPromptAction.MEASUREMENT.value)).lower())
        comment = str(resolved.get("comment") or "").strip()
        if action == ManualPromptAction.CANCEL:
            self.cancel(comment)

        raw_val = str(resolved.get("raw_value") or resolved.get("measurement") or "").strip()
        measurement_val = resolved.get("measurement")
        numeric_measurement = (
            float(measurement_val) if measurement_val is not None else parse_measurement_string(raw_val, unit)
        )
        return ManualPromptResult(
            action=ManualPromptAction.MEASUREMENT,
            passed=True,
            measurement=numeric_measurement,
            raw_value=raw_val,
            comment=comment,
        )
