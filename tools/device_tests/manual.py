"""Pytest-native manual benchtop verification helper with periodic `host.py` prompt heartbeats."""

import asyncio
from dataclasses import dataclass
from typing import Callable, Optional

import pytest

from device_tests.client import AsyncHostSessionClient

DEFAULT_PROMPT_HEARTBEAT_INTERVAL_SEC: float = 2.0


@dataclass(frozen=True)
class ManualPromptResult:
    """Outcome of an interactive operator verification inside a manual pytest test."""

    action: str
    passed: bool
    notes: str = ""


class ManualTestHelper:
    """Executes manual benchtop checks inside `pytest` while streaming context to `host.py`.

    While waiting for operator input (`Pass` / `Fail` / `Skip`), a background asyncio task
    periodically sends `{test_name, project, procedure, prompt, active}` to `host.py`
    (`/api/manual_prompt`) so any developer viewing the host daemon or workstation UI has
    live context on what procedure and prompt is expected next.
    """

    def __init__(
        self,
        *,
        host: AsyncHostSessionClient,
        test_name: str,
        project: str,
        interactive: bool,
        heartbeat_interval: float = DEFAULT_PROMPT_HEARTBEAT_INTERVAL_SEC,
        prompt_reader: Optional[Callable[[str], str]] = None,
    ) -> None:
        """Initialize manual helper for the active pytest test item."""
        self.host = host
        self.test_name = test_name
        self.project = project
        self.interactive = interactive
        self.heartbeat_interval = max(heartbeat_interval, 0.02)
        self._prompt_reader = prompt_reader

    async def verify(
        self,
        *,
        procedure: str,
        prompt: str,
    ) -> ManualPromptResult:
        """Prompt the operator in `pytest` while periodically broadcasting context to `host.py`."""
        if not self.interactive:
            pytest.skip(f"Manual step '{self.test_name}' skipped (pass --interactive): {prompt}")

        stop_event = asyncio.Event()

        async def _heartbeat_loop() -> None:
            while not stop_event.is_set():
                await self.host.publish_manual_prompt(
                    test_name=self.test_name,
                    project=self.project,
                    procedure=procedure,
                    prompt=prompt,
                    active=True,
                )
                try:
                    await asyncio.wait_for(stop_event.wait(), timeout=self.heartbeat_interval)
                except asyncio.TimeoutError:
                    continue

        heartbeat_task = asyncio.create_task(_heartbeat_loop())
        try:
            from host_daemon.cli import format_clickable_choice

            pass_choice = format_clickable_choice("Pass", "P", "pass", "\033[1;32m")
            fail_choice = format_clickable_choice("Fail", "F", "fail", "\033[1;31m")
            skip_choice = format_clickable_choice("Skip", "S", "skip", "\033[1;33m")
            rendered_prompt = (
                f"\n[MANUAL TEST: {self.test_name} ({self.project})]\n"
                f"  Procedure: {procedure}\n"
                f"  Expected:  {prompt}\n"
                f"Outcome {pass_choice}, {fail_choice}, {skip_choice} (default: PASS): "
            )
            if self._prompt_reader is not None:
                raw_choice = await asyncio.to_thread(self._prompt_reader, rendered_prompt)
            else:
                raw_choice = await asyncio.to_thread(input, rendered_prompt)
            verdict = (raw_choice or "").strip().lower() or "pass"
        finally:
            stop_event.set()
            await heartbeat_task
            await self.host.publish_manual_prompt(
                test_name=self.test_name,
                project=self.project,
                procedure=procedure,
                prompt=prompt,
                active=False,
            )

        if verdict.startswith("s"):
            pytest.skip(f"Operator skipped manual test '{self.test_name}': {prompt}")
        if verdict.startswith("f"):
            pytest.fail(f"Operator failed manual test '{self.test_name}': {prompt}")

        return ManualPromptResult(action="pass", passed=True)
