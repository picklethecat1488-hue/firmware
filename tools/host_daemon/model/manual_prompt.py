"""Data model for active manual test prompts streamed from `device_tests` to `host.py`."""

from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Any, Dict


@dataclass
class ManualPromptState:
    """Represents the active manual test procedure and user prompt sent by `pytest`."""

    active: bool = False
    test_name: str = ""
    project: str = ""
    procedure: str = ""
    prompt: str = ""
    heartbeat_count: int = 0
    updated_at: str = ""

    def to_dict(self) -> Dict[str, Any]:
        """Serialize manual prompt state to a JSON-compatible dictionary."""
        return {
            "active": self.active,
            "test_name": self.test_name,
            "project": self.project,
            "procedure": self.procedure,
            "prompt": self.prompt,
            "heartbeat_count": self.heartbeat_count,
            "updated_at": self.updated_at,
        }

    def apply_update(self, payload: Dict[str, Any]) -> bool:
        """Apply an incoming prompt heartbeat payload; return True if this is a newly activated prompt."""
        new_active = bool(payload.get("active", True))
        new_test = str(payload.get("test_name") or "").strip()
        new_project = str(payload.get("project") or "").strip()
        new_proc = str(payload.get("procedure") or "").strip()
        new_prompt = str(payload.get("prompt") or "").strip()

        newly_activated = new_active and (not self.active or self.test_name != new_test or self.prompt != new_prompt)
        self.active = new_active
        self.test_name = new_test if new_active else ""
        self.project = new_project if new_active else ""
        self.procedure = new_proc if new_active else ""
        self.prompt = new_prompt if new_active else ""
        self.heartbeat_count = (self.heartbeat_count + 1) if new_active else 0
        self.updated_at = datetime.now(timezone.utc).isoformat()
        return newly_activated
