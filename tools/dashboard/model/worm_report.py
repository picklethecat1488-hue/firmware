"""Domain data models for Worm Report CLI and Markdown Tracker.

Provides structured schemas for worm reports, severities, statuses, categories,
attachments (logs, screenshots, references), and worm collection management.
"""

from collections.abc import Collection
from enum import Enum
from typing import Dict, List, Optional
import uuid as uuid_pkg
from pydantic import BaseModel, Field

from model.id_generator import generate_docker_pattern_id


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
        """Find a worm by its unique ID (exact, case-insensitive, or prefix-normalized)."""
        clean_target = worm_id.strip()
        for w in self.worms:
            if w.id == clean_target:
                return w
        for w in self.worms:
            if w.id.upper() == clean_target.upper():
                return w
        for w in self.worms:
            norm_w = w.id.removeprefix("WORM-").removeprefix("BUG-").upper()
            norm_target = clean_target.removeprefix("WORM-").removeprefix("BUG-").upper()
            if norm_w == norm_target:
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

    def generate_worm_id(self, existing_ids: Optional[Collection[str]] = None) -> str:
        """Generate a random Docker-pattern worm ID using a CRNG (secrets/os.urandom).

        Format: WORM-[ADJECTIVE]-[ANIMAL/NOUN]-[3 digits]
        Examples:
            - WORM-BURROWING-ANNELID-42
            - WORM-WRIGGLY-EARTHWORM-809
            - WORM-SLIMY-NIGHTCRAWLER-17
        """
        known = set(existing_ids or [])
        for w in self.worms:
            known.add(w.id)
        return generate_docker_pattern_id(prefix="WORM", existing_ids=known)
