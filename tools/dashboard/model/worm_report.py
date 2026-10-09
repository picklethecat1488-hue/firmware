"""Domain data models for Worm Report CLI and Markdown Tracker.

Provides structured schemas for worm reports, severities, statuses, categories,
attachments (logs, screenshots, references), and worm collection management.
"""

from enum import Enum
from typing import Dict, List, Optional
import uuid as uuid_pkg
from pydantic import BaseModel, Field


class WormSeverity(str, Enum):
    """Severity ratings for engineering worm reports."""

    CRITICAL = "CRITICAL"  # Blocker / Severe defect preventing build or execution
    HIGH = "HIGH"  # Major functional defect or design flaw
    MEDIUM = "MEDIUM"  # Moderate issue with workaround available
    LOW = "LOW"  # Minor defect, cosmetic issue, or low-priority polish


class WormStatus(str, Enum):
    """Lifecycle status for a worm report."""

    OPEN = "OPEN"
    PLANNED = "PLANNED"
    IN_PROGRESS = "IN_PROGRESS"
    RESOLVED = "RESOLVED"
    CLOSED = "CLOSED"


class WormCategory(str, Enum):
    """Subsystem classification for worm reports."""

    FIRMWARE = "FIRMWARE"
    CONTROLLER = "CONTROLLER"
    DRIVER = "DRIVER"
    PLATFORM = "PLATFORM"
    BOARD = "BOARD"
    MODEL = "MODEL"
    SHELL = "SHELL"
    INFRASTRUCTURE = "INFRASTRUCTURE"
    UI = "UI"
    GENERAL = "GENERAL"


class WormAttachmentModel(BaseModel):
    """File attachment (log, screenshot, artifact, reference) associated with a worm."""

    id: str
    filename: str
    file_type: str = "reference"  # log, screenshot, reference, artifact
    file_path: str
    size_bytes: int = 0
    description: str = ""
    created_at: str = ""


class WormReportModel(BaseModel):
    """Complete specification of an engineering worm report."""

    id: str
    uuid: str = Field(default_factory=lambda: str(uuid_pkg.uuid4()))
    title: str
    status: WormStatus = WormStatus.OPEN
    severity: WormSeverity = WormSeverity.MEDIUM
    category: WormCategory = WormCategory.FIRMWARE
    component: str = ""
    description: str = ""
    reproduction_steps: List[str] = Field(default_factory=list)
    expected_behavior: str = ""
    actual_behavior: str = ""
    logs: str = ""
    attachments: List[WormAttachmentModel] = Field(default_factory=list)
    created_at: str = ""
    updated_at: str = ""
    resolved_at: Optional[str] = None
    resolution_notes: str = ""


class WormDatabaseModel(BaseModel):
    """Container and serialization model for tracking repository worms."""

    title: str = "Firmware Engineering Worm Tracker"
    repo_root: Optional[str] = ""
    summary: str = ""
    worms: List[WormReportModel] = Field(default_factory=list)
    updated_at: str = ""

    def get_worm(self, worm_id: str) -> Optional[WormReportModel]:
        """Find a worm by its unique ID."""
        for w in self.worms:
            if w.id == worm_id:
                return w
        return None

    def get_worm_by_uuid(self, worm_uuid: str) -> Optional[WormReportModel]:
        """Find a worm by its unique UUID."""
        for w in self.worms:
            if w.uuid == worm_uuid:
                return w
        return None

    def add_or_update(self, worm: WormReportModel) -> None:
        """Add a new worm or update an existing worm matching UUID or ID."""
        for idx, existing in enumerate(self.worms):
            if existing.uuid == worm.uuid or existing.id == worm.id:
                self.worms[idx] = worm
                return
        self.worms.append(worm)

    def count_by_status(self) -> Dict[str, int]:
        """Tally worms grouped by lifecycle status."""
        counts = {st.value: 0 for st in WormStatus}
        for w in self.worms:
            counts[w.status.value] = counts.get(w.status.value, 0) + 1
        return counts

    def count_by_severity(self) -> Dict[str, int]:
        """Tally worms grouped by severity level."""
        counts = {sev.value: 0 for sev in WormSeverity}
        for w in self.worms:
            counts[w.severity.value] = counts.get(w.severity.value, 0) + 1
        return counts

    def count_by_category(self) -> Dict[str, int]:
        """Tally worms grouped by subsystem category."""
        counts = {cat.value: 0 for cat in WormCategory}
        for w in self.worms:
            counts[w.category.value] = counts.get(w.category.value, 0) + 1
        return counts

    def generate_worm_id(self) -> str:
        """Generate next sequential worm ID (e.g. WORM-001, WORM-002)."""
        max_idx = 0
        for w in self.worms:
            id_str = w.id
            if id_str.startswith("WORM-"):
                num_str = id_str[5:]
            elif id_str.startswith("BUG-"):
                num_str = id_str[4:]
            else:
                num_str = ""
            if num_str.isdigit():
                idx = int(num_str)
                if idx > max_idx:
                    max_idx = idx
        return f"WORM-{max_idx + 1:03d}"
