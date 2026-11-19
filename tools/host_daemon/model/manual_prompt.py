"""Data model for active manual test prompts and operator responses between `device_tests` and `host.py`."""

from dataclasses import dataclass
from datetime import datetime, timezone
from enum import Enum
from typing import Any, Dict, Optional


class ManualPromptKind(str, Enum):
    """Supported interactive manual test prompt types."""

    YES_NO = "yes_no"
    MEASUREMENT = "measurement"


class ManualPromptAction(str, Enum):
    """Operator actions supported for an active manual test prompt."""

    YES = "yes"
    NO = "no"
    MEASUREMENT = "measurement"
    CANCEL = "cancel"


def parse_measurement_string(raw_value: str, unit: str = "") -> float:
    """Parse a numeric float from user input, stripping an optional trailing unit suffix."""
    cleaned = raw_value.strip()
    if not cleaned:
        raise ValueError("Measured value cannot be empty")
    clean_unit = unit.strip()
    if clean_unit and cleaned.lower().endswith(clean_unit.lower()):
        cleaned = cleaned[: -len(clean_unit)].strip()
    return float(cleaned)


@dataclass
class ManualPromptState:
    """Represents the active manual test procedure, expected prompt, and operator response on `host.py`."""

    active: bool = False
    prompt_id: str = ""
    kind: str = ManualPromptKind.YES_NO.value
    unit: str = ""
    test_name: str = ""
    project: str = ""
    procedure: str = ""
    prompt: str = ""
    heartbeat_count: int = 0
    updated_at: str = ""
    response: Optional[Dict[str, Any]] = None

    def to_dict(self) -> Dict[str, Any]:
        """Serialize manual prompt state to a JSON-compatible dictionary."""
        return {
            "active": self.active,
            "prompt_id": self.prompt_id,
            "kind": self.kind,
            "unit": self.unit,
            "test_name": self.test_name,
            "project": self.project,
            "procedure": self.procedure,
            "prompt": self.prompt,
            "heartbeat_count": self.heartbeat_count,
            "updated_at": self.updated_at,
            "response": dict(self.response) if self.response is not None else None,
        }

    def apply_update(self, payload: Dict[str, Any]) -> bool:
        """Apply an incoming prompt heartbeat payload; return True if this is a newly activated prompt."""
        new_active = bool(payload.get("active", True))
        incoming_id = str(payload.get("prompt_id") or "").strip()
        raw_kind = str(payload.get("kind") or ManualPromptKind.YES_NO.value).strip().lower()
        new_kind = (
            raw_kind
            if raw_kind in (ManualPromptKind.YES_NO.value, ManualPromptKind.MEASUREMENT.value)
            else ManualPromptKind.YES_NO.value
        )
        new_unit = str(payload.get("unit") or "").strip()
        new_test = str(payload.get("test_name") or "").strip()
        new_project = str(payload.get("project") or "").strip()
        new_proc = str(payload.get("procedure") or "").strip()
        new_prompt = str(payload.get("prompt") or "").strip()
        now_iso = datetime.now(timezone.utc).isoformat()

        if not new_active:
            self.active = False
            self.prompt_id = ""
            self.kind = ManualPromptKind.YES_NO.value
            self.unit = ""
            self.test_name = ""
            self.project = ""
            self.procedure = ""
            self.prompt = ""
            self.heartbeat_count = 0
            self.response = None
            self.updated_at = now_iso
            return False

        if incoming_id and incoming_id == self.prompt_id and self.response is not None:
            self.heartbeat_count += 1
            self.updated_at = now_iso
            return False

        newly_activated = (
            not self.active
            or (incoming_id != "" and self.prompt_id != incoming_id)
            or self.test_name != new_test
            or self.prompt != new_prompt
        )
        if newly_activated:
            self.response = None
            self.heartbeat_count = 1
        else:
            self.heartbeat_count += 1

        self.active = True
        self.prompt_id = incoming_id
        self.kind = new_kind
        self.unit = new_unit
        self.test_name = new_test
        self.project = new_project
        self.procedure = new_proc
        self.prompt = new_prompt
        self.updated_at = now_iso
        return newly_activated

    def submit_response(self, payload: Dict[str, Any]) -> Dict[str, Any]:
        """Validate and record an operator response (`yes`, `no`, `measurement`, or `cancel` with comment)."""
        if not self.active:
            raise ValueError("No active manual test prompt is awaiting a response")

        requested_id = str(payload.get("prompt_id") or "").strip()
        if requested_id and self.prompt_id and requested_id != self.prompt_id:
            raise ValueError(f"Stale manual prompt ID '{requested_id}' (expected '{self.prompt_id}')")

        raw_action = str(payload.get("action") or "").strip().lower()
        try:
            action = ManualPromptAction(raw_action)
        except ValueError as exc:
            raise ValueError(
                f"Invalid manual prompt action '{raw_action}'. Expected: yes, no, measurement, cancel"
            ) from exc

        comment = str(payload.get("comment") or "").strip()
        raw_value = ""
        measurement: Optional[float] = None

        if action == ManualPromptAction.MEASUREMENT:
            raw_value = str(payload.get("value") if payload.get("value") is not None else "").strip()
            measurement = parse_measurement_string(raw_value, self.unit)
        elif action == ManualPromptAction.CANCEL:
            if not comment:
                raise ValueError("Cancelling a manual test requires a non-empty comment")
        elif self.kind == ManualPromptKind.MEASUREMENT:
            raise ValueError("Active prompt requires a numeric measurement or cancellation with comment")

        now_iso = datetime.now(timezone.utc).isoformat()
        self.response = {
            "prompt_id": self.prompt_id,
            "action": action.value,
            "measurement": measurement,
            "raw_value": raw_value,
            "comment": comment,
            "responded_at": now_iso,
        }
        self.active = False
        self.updated_at = now_iso
        return self.to_dict()
