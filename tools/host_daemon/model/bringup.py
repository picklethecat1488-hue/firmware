"""Bringup checklist data models and YAML schema definitions for Xerxes Host Workstation."""

from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
import re
from typing import Any, Dict, List, Optional, Tuple, Union
import yaml

DEFAULT_DEFAULT_BIN: str = "cat_detector_shell"
DEFAULT_SERIAL_TIMEOUT_SEC: float = 5.0
DEFAULT_HOST_COMMAND_TIMEOUT_SEC: float = 30.0


class BringupStepType(str, Enum):
    """Supported Bringup step types."""

    SHELL_COMMAND = "shell_command"
    HOST_COMMAND = "host_command"
    HARDWARE_CHECK = "hardware_check"
    INTERACTIVE = "interactive"
    MANUAL = "manual"


class BringupStepStatus(str, Enum):
    """Execution status for Bringup checklist steps."""

    PENDING = "pending"
    RUNNING = "running"
    PASSED = "passed"
    FAILED = "failed"
    SKIPPED = "skipped"


@dataclass
class BringupStep:
    """Individual bringup checklist step definition and execution result."""

    id: str
    name: str
    type: BringupStepType
    command: Optional[str] = None
    expected_regex: Optional[str] = None
    description: Optional[str] = None
    procedure: Optional[str] = None
    expected: Optional[str] = None
    reboot: bool = False
    flash_before: Optional[str] = None
    test_points: Dict[str, str] = field(default_factory=dict)
    control_pins: Dict[str, str] = field(default_factory=dict)
    status: BringupStepStatus = BringupStepStatus.PENDING
    output_log: str = ""
    matched_line: str = ""
    notes: str = ""
    timestamp: Optional[str] = None

    def to_dict(self) -> Dict[str, Any]:
        """Convert step instance to JSON-serializable dictionary."""
        return {
            "id": self.id,
            "name": self.name,
            "type": self.type.value,
            "command": self.command,
            "expected_regex": self.expected_regex,
            "description": self.description,
            "procedure": self.procedure,
            "expected": self.expected,
            "reboot": self.reboot,
            "flash_before": self.flash_before,
            "test_points": dict(self.test_points),
            "control_pins": dict(self.control_pins),
            "status": self.status.value,
            "output_log": self.output_log,
            "matched_line": self.matched_line,
            "notes": self.notes,
            "timestamp": self.timestamp,
        }


@dataclass
class BringupConfig:
    """Parsed bringup configuration file containing project metadata and steps."""

    project_name: str
    device_chip: str
    flash_addr: str
    flash_size: int
    cargo_target: Optional[str] = None
    app_cargo_target: Optional[str] = None
    file_path: Optional[str] = None
    steps: List[BringupStep] = field(default_factory=list)

    def to_dict(self) -> Dict[str, Any]:
        """Convert configuration instance to JSON-serializable dictionary."""
        return {
            "project_name": self.project_name,
            "device_chip": self.device_chip,
            "flash_addr": self.flash_addr,
            "flash_size": self.flash_size,
            "cargo_target": self.cargo_target,
            "app_cargo_target": self.app_cargo_target,
            "file_path": self.file_path,
            "steps": [step.to_dict() for step in self.steps],
        }


def check_regex(output_log: str, regex_str: str) -> Tuple[bool, str]:
    """Search for regular expression pattern line-by-line within output log."""
    if not regex_str or not output_log:
        return False, ""
    try:
        pattern = re.compile(regex_str)
        for line in output_log.splitlines():
            if pattern.search(line):
                return True, line.strip()
    except re.error:
        return False, ""
    return False, ""


def parse_cargo_target(cargo_target_str: str, default_bin: str = DEFAULT_DEFAULT_BIN) -> str:
    """Parse cargo invocation string and return path to the target ELF binary."""
    args = cargo_target_str.split()
    target = None
    bin_name = None
    is_release = False

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "--target" and i + 1 < len(args):
            target = args[i + 1]
            i += 2
        elif arg == "--bin" and i + 1 < len(args):
            bin_name = args[i + 1]
            i += 2
        elif arg == "--release":
            is_release = True
            i += 1
        else:
            i += 1

    if not bin_name:
        bin_name = default_bin

    profile = "release" if is_release else "debug"
    if target:
        return f"target/{target}/{profile}/{bin_name}"
    return f"target/{profile}/{bin_name}"


def load_bringup_yaml(file_path: Union[str, Path]) -> BringupConfig:
    """Load and parse YAML bringup checklist file into validated BringupConfig."""
    path = Path(file_path).resolve()
    if not path.exists():
        raise FileNotFoundError(f"Bringup configuration file not found at: {path}")

    content = path.read_text(encoding="utf-8")
    data = yaml.safe_load(content)
    if not isinstance(data, dict):
        raise ValueError(f"Invalid bringup configuration format in {path}: expected mapping root")

    project_name = str(data.get("project_name") or "Unknown Project")
    device_chip = str(data.get("device_chip") or "RP2040")
    flash_addr = str(data.get("flash_addr") or "0x101C0000")
    flash_size = int(data.get("flash_size") or 262144)
    cargo_target = data.get("cargo_target")
    app_cargo_target = data.get("app_cargo_target")

    raw_steps = data.get("steps") or []
    steps: List[BringupStep] = []
    for item in raw_steps:
        if not isinstance(item, dict):
            continue
        step_id = str(item.get("id") or str(len(steps) + 1))
        step_name = str(item.get("name") or f"Step {step_id}")
        raw_type = str(item.get("type") or "manual").lower()

        try:
            step_type = BringupStepType(raw_type)
        except ValueError:
            step_type = BringupStepType.MANUAL

        steps.append(
            BringupStep(
                id=step_id,
                name=step_name,
                type=step_type,
                command=item.get("command"),
                expected_regex=item.get("expected_regex"),
                description=item.get("description"),
                procedure=item.get("procedure"),
                expected=item.get("expected"),
                reboot=bool(item.get("reboot", False)),
                flash_before=item.get("flash_before"),
                test_points=dict(item.get("test_points") or {}),
                control_pins=dict(item.get("control_pins") or {}),
                status=BringupStepStatus.PENDING,
            )
        )

    return BringupConfig(
        project_name=project_name,
        device_chip=device_chip,
        flash_addr=flash_addr,
        flash_size=flash_size,
        cargo_target=cargo_target,
        app_cargo_target=app_cargo_target,
        file_path=str(path),
        steps=steps,
    )


def discover_bringup_scripts(repo_root: Path) -> List[Dict[str, Any]]:
    """Scan workspace repository for valid bringup YAML checklist configurations."""
    scripts: List[Dict[str, Any]] = []
    search_dirs = [repo_root / "app", repo_root / "tools", repo_root / "projects"]

    for sdir in search_dirs:
        if not sdir.exists():
            continue
        for yaml_file in sorted(sdir.glob("**/*.yaml")):
            try:
                cfg = load_bringup_yaml(yaml_file)
                rel_path = yaml_file.relative_to(repo_root)
                scripts.append({
                    "name": cfg.project_name,
                    "path": str(rel_path),
                    "chip": cfg.device_chip,
                    "steps_count": len(cfg.steps),
                })
            except Exception:
                continue

    return scripts
