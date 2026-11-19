"""Data models, glob board matchers, project discovery, and decorators for on-device pytest suites."""

from dataclasses import dataclass
from enum import Enum
import fnmatch
import importlib.util
import inspect
import json
from pathlib import Path
import re
import sys
from typing import Any, Callable, Dict, List, Optional, Sequence, Tuple, TypeVar, Union

import pytest

DEFAULT_PACKAGE_NAME: str = "cat_detector"
RP2040_ARM_TARGET: str = "thumbv6m-none-eabi"
MCXN947_ARM_TARGET: str = "thumbv8m.main-none-eabihf"
DEFAULT_RP2040_CHIP: str = "rp2040"
DEFAULT_MCXN947_CHIP: str = "MCXN947"

F = TypeVar("F", bound=Callable[..., Any])


class BoardType(str, Enum):
    """Canonical board type identifiers stored in `.program_metadata` and `metadata` partitions."""

    RP2040_FLASH_2MB = "rp2040_flash_2mb"
    RP2040_FLASH_4MB = "rp2040_flash_4mb"
    RP2040_FLASH_8MB = "rp2040_flash_8mb"
    RP2040_FLASH_16MB = "rp2040_flash_16mb"
    CARRIER_BOARD = "carrier_board"


class ScenarioTier(str, Enum):
    """Supported on-device verification scenario tiers."""

    SMOKE = "smoke"
    INTEGRATION = "integration"
    BRINGUP = "bringup"


@dataclass(frozen=True)
class TargetBinarySpec:
    """Specification of the firmware binary required on target for a device test."""

    package: str
    bin_name: str
    arm_target: str = RP2040_ARM_TARGET
    chip: str = DEFAULT_RP2040_CHIP
    release: bool = False

    def to_dict(self) -> Dict[str, Any]:
        """Serialize target binary specification to a JSON-compatible dictionary."""
        return {
            "package": self.package,
            "bin": self.bin_name,
            "target": self.arm_target,
            "chip": self.chip,
            "release": self.release,
            "profile": "release" if self.release else "debug",
        }


KNOWN_TARGET_BINARIES: Dict[str, TargetBinarySpec] = {
    "cat_detector_shell": TargetBinarySpec(
        package=DEFAULT_PACKAGE_NAME,
        bin_name="cat_detector_shell",
        arm_target=RP2040_ARM_TARGET,
        chip=DEFAULT_RP2040_CHIP,
        release=False,
    ),
    "cat_detector_app": TargetBinarySpec(
        package=DEFAULT_PACKAGE_NAME,
        bin_name="cat_detector_app",
        arm_target=RP2040_ARM_TARGET,
        chip=DEFAULT_RP2040_CHIP,
        release=False,
    ),
    "carrier_board_shell": TargetBinarySpec(
        package=DEFAULT_PACKAGE_NAME,
        bin_name="carrier_board_shell",
        arm_target=RP2040_ARM_TARGET,
        chip=DEFAULT_MCXN947_CHIP,
        release=False,
    ),
}


@dataclass(frozen=True)
class DetectedBoard:
    """Board identity and 16-bit metadata checksum validation returned by `host.py` / `query_metadata`."""

    board_type: str
    chip: str
    source: str = "program_metadata"
    checksum: Optional[str] = None
    elf_checksum: Optional[str] = None
    matches_target: bool = True

    @classmethod
    def from_metadata_response(cls, payload: Dict[str, Any]) -> Optional["DetectedBoard"]:
        """Construct `DetectedBoard` from a `/api/metadata` or `query_metadata` dictionary."""
        bt = str(payload.get("board_type") or "").strip()
        if not bt:
            return None
        return cls(
            board_type=bt,
            chip=str(payload.get("chip") or DEFAULT_RP2040_CHIP).strip(),
            source=str(payload.get("source") or "program_metadata").strip(),
            checksum=payload.get("checksum"),
            elf_checksum=payload.get("elf_checksum"),
            matches_target=bool(payload.get("matches_target", True)),
        )

    def to_dict(self) -> Dict[str, Any]:
        """Serialize detected board metadata to a dictionary."""
        return {
            "board_type": self.board_type,
            "chip": self.chip,
            "source": self.source,
            "checksum": self.checksum,
            "elf_checksum": self.elf_checksum,
            "matches_target": self.matches_target,
        }


@dataclass(frozen=True)
class DeviceTestMetadata:
    """Metadata attached to each `@device_test` coroutine (ordered by `definition_lineno`)."""

    name: str
    project: str
    boards: Tuple[str, ...]
    scenarios: Tuple[ScenarioTier, ...]
    target_binary: Optional[TargetBinarySpec]
    manual: bool = False
    description: str = ""
    definition_lineno: int = 0
    file_path: str = ""

    def to_dict(self, order_index: int = 1) -> Dict[str, Any]:
        """Serialize test metadata with its 1-based listing order index."""
        return {
            "order": order_index,
            "name": self.name,
            "project": self.project,
            "boards": list(self.boards),
            "scenarios": [s.value for s in self.scenarios],
            "target_binary": self.target_binary.bin_name if self.target_binary else None,
            "manual": self.manual,
            "description": self.description,
            "lineno": self.definition_lineno,
            "file_path": self.file_path,
        }


def resolve_target_binary(
    binary: Union[None, str, TargetBinarySpec],
    *,
    default_package: str = DEFAULT_PACKAGE_NAME,
) -> Optional[TargetBinarySpec]:
    """Resolve a binary name or `TargetBinarySpec` into a canonical `TargetBinarySpec`."""
    if binary is None:
        return None
    if isinstance(binary, TargetBinarySpec):
        return binary
    cleaned = binary.strip()
    if not cleaned:
        return None
    if cleaned in KNOWN_TARGET_BINARIES:
        return KNOWN_TARGET_BINARIES[cleaned]
    chip = DEFAULT_MCXN947_CHIP if "carrier_board" in cleaned else DEFAULT_RP2040_CHIP
    return TargetBinarySpec(
        package=default_package,
        bin_name=cleaned,
        arm_target=RP2040_ARM_TARGET,
        chip=chip,
        release=False,
    )


def matches_board_patterns(board_type: str, patterns: Sequence[str]) -> bool:
    """Return True if `board_type` matches any glob pattern in `patterns` (case-insensitive)."""
    if not patterns:
        return True
    normalized = board_type.strip().lower()
    if not normalized:
        return False
    for pattern in patterns:
        pat = pattern.strip().lower()
        if pat in ("*", "any", "all") or fnmatch.fnmatchcase(normalized, pat):
            return True
    return False


def check_regex(output: str, pattern: str) -> Tuple[bool, str]:
    """Search `output` for `pattern` line-by-line or multiline, returning `(matched, matched_line)`."""
    if not pattern:
        return True, ""
    compiled = re.compile(pattern, re.MULTILINE)
    for raw_line in output.splitlines():
        if compiled.search(raw_line):
            return True, raw_line.strip()
    match = compiled.search(output)
    if match:
        return True, match.group(0).strip()
    return False, ""


def parse_query_metadata_output(raw_output: str) -> Optional[DetectedBoard]:
    """Parse JSON or key-value stdout emitted by `host_fs query_metadata` into `DetectedBoard`."""
    cleaned = raw_output.strip()
    if not cleaned or "error:" in cleaned.lower():
        return None

    for line in reversed(cleaned.splitlines()):
        candidate = line.strip()
        if candidate.startswith("{") and candidate.endswith("}"):
            try:
                payload = json.loads(candidate)
            except json.JSONDecodeError:
                payload = None
            if isinstance(payload, dict) and payload.get("board_type"):
                return DetectedBoard.from_metadata_response(payload)

    kv: Dict[str, str] = {}
    for line in cleaned.splitlines():
        if ":" in line and not line.strip().startswith("partition["):
            key, val = line.split(":", 1)
            kv[key.strip().lower()] = val.strip()
    if "board_type" in kv and kv["board_type"]:
        matches_str = kv.get("matches_target", "true").lower()
        return DetectedBoard(
            board_type=kv["board_type"],
            chip=kv.get("chip", DEFAULT_RP2040_CHIP),
            source=kv.get("source", "program_metadata"),
            checksum=kv.get("checksum"),
            elf_checksum=kv.get("elf_checksum"),
            matches_target=matches_str not in ("false", "0", "no"),
        )
    return None


def device_test(
    *,
    project: str,
    boards: Union[str, Sequence[Union[str, BoardType]]] = ("*",),
    scenarios: Sequence[Union[str, ScenarioTier]] = (ScenarioTier.SMOKE,),
    target_binary: Union[None, str, TargetBinarySpec] = None,
    manual: bool = False,
) -> Callable[[F], F]:
    """Attach project, board-glob, scenario-tier, and `target_binary` attributes to an async pytest test.

    Execution order is inferred automatically from top-to-bottom source-line placement in each
    suite listing (`definition_lineno`), and all test assertions/manual prompts reside inside
    the test body itself.
    """
    normalized_project = project.strip().lower()
    if not normalized_project:
        raise ValueError("@device_test requires a non-empty `project` identifier")

    raw_boards = [boards] if isinstance(boards, (str, BoardType)) else list(boards)
    board_patterns = tuple(b.value if isinstance(b, BoardType) else str(b) for b in raw_boards)
    scenario_tiers = tuple(s if isinstance(s, ScenarioTier) else ScenarioTier(str(s).lower()) for s in scenarios)
    resolved_bin = resolve_target_binary(target_binary)

    def _decorator(fn: F) -> F:
        try:
            _, lineno = inspect.getsourcelines(fn)
        except OSError:
            lineno = 0
        try:
            src_file = inspect.getsourcefile(fn) or ""
        except TypeError:
            src_file = ""

        doc_summary = inspect.getdoc(fn) or ""
        metadata = DeviceTestMetadata(
            name=fn.__name__,
            project=normalized_project,
            boards=board_patterns,
            scenarios=scenario_tiers,
            target_binary=resolved_bin,
            manual=manual,
            description=doc_summary,
            definition_lineno=lineno,
            file_path=src_file,
        )
        setattr(fn, "__device_test_meta__", metadata)

        marked: Any = pytest.mark.asyncio(fn)
        marked = pytest.mark.project(normalized_project)(marked)
        marked = pytest.mark.boards(*board_patterns)(marked)
        if resolved_bin is not None:
            marked = pytest.mark.target_binary(resolved_bin.bin_name)(marked)
        if doc_summary:
            marked = pytest.mark.description(doc_summary)(marked)
        for tier in scenario_tiers:
            marked = getattr(pytest.mark, tier.value)(marked)
        if manual:
            marked = pytest.mark.manual(marked)
            marked = pytest.mark.interactive(marked)
        setattr(marked, "__device_test_meta__", metadata)
        return marked

    return _decorator


def get_device_test_metadata(obj: Any) -> Optional[DeviceTestMetadata]:
    """Extract `DeviceTestMetadata` from a decorated function or `pytest.Item`, if present."""
    direct = getattr(obj, "__device_test_meta__", None)
    if isinstance(direct, DeviceTestMetadata):
        return direct
    underlying = getattr(obj, "obj", None)
    if underlying is not None:
        meta = getattr(underlying, "__device_test_meta__", None)
        if isinstance(meta, DeviceTestMetadata):
            return meta
    return None


def load_device_test_file(file_path: Union[str, Path]) -> List[Tuple[Callable[..., Any], DeviceTestMetadata]]:
    """Import a Python suite listing and return `(fn, metadata)` sorted top-to-bottom by line number."""
    path = Path(file_path).resolve()
    if not path.exists() or path.suffix != ".py":
        raise FileNotFoundError(f"Device test Python file not found: {path}")

    for ancestor in path.parents:
        if (ancestor / "device_tests").is_dir():
            if str(ancestor) not in sys.path:
                sys.path.insert(0, str(ancestor))
            break

    mod_key = f"_xerxes_device_listing_{abs(hash(str(path)))}_{path.stem}"
    spec = importlib.util.spec_from_file_location(mod_key, path)
    if spec is None or spec.loader is None:
        raise ValueError(f"Could not create import spec for {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[mod_key] = module
    spec.loader.exec_module(module)

    collected: List[Tuple[int, Callable[..., Any], DeviceTestMetadata]] = []
    for _, member in inspect.getmembers(module, inspect.iscoroutinefunction):
        meta = get_device_test_metadata(member)
        if meta is not None:
            collected.append((meta.definition_lineno, member, meta))
    collected.sort(key=lambda item: item[0])
    return [(fn, meta) for _, fn, meta in collected]


def discover_device_tests(
    repo_root: Path,
    *,
    project: Optional[str] = None,
    script: Optional[Union[str, Path]] = None,
    scenarios: Optional[Sequence[ScenarioTier]] = None,
) -> List[Dict[str, Any]]:
    """Discover `@device_test` functions across `tools/device_tests/suites/` (or a specific script)."""
    project_norm = project.strip().lower() if project and project.strip().lower() not in ("all", "*") else None
    scenario_filter = set(scenarios) if scenarios else None

    if script is not None:
        file_part = str(script).partition("::")[0]
        candidate = Path(file_part) if Path(file_part).is_absolute() else (repo_root / file_part)
        files = [candidate] if candidate.is_file() else sorted(candidate.glob("test_*.py"))
    else:
        suites_dir = repo_root / "tools" / "device_tests" / "suites"
        files = sorted(suites_dir.glob("test_*.py")) if suites_dir.is_dir() else []

    results: List[Dict[str, Any]] = []
    order_counter = 0
    for suite_file in files:
        pairs = load_device_test_file(suite_file)
        rel_path = (
            str(suite_file.resolve().relative_to(repo_root.resolve()))
            if suite_file.resolve().is_relative_to(repo_root.resolve())
            else str(suite_file)
        )
        for _, meta in pairs:
            if project_norm and meta.project != project_norm:
                continue
            if scenario_filter and not (set(meta.scenarios) & scenario_filter):
                continue
            order_counter += 1
            entry = meta.to_dict(order_index=order_counter)
            entry["file_path"] = rel_path
            results.append(entry)
    return results


def infer_scenario_from_script(
    script_spec: Union[str, Path],
    repo_root: Optional[Path] = None,
) -> str:
    """Infer the pytest scenario marker expression from a `@device_test` script or node ID."""
    raw_spec = str(script_spec).strip()
    file_part, _, node_selector = raw_spec.partition("::")
    candidate = Path(file_part)
    if not candidate.is_absolute() and repo_root is not None:
        candidate = repo_root / candidate

    target_func_name = node_selector.split("::")[-1].strip() if node_selector else ""
    if candidate.exists() and candidate.is_file() and candidate.suffix == ".py":
        pairs = load_device_test_file(candidate)
        if target_func_name:
            for fn, meta in pairs:
                if fn.__name__ == target_func_name and meta.scenarios:
                    if len(meta.scenarios) == 1:
                        return meta.scenarios[0].value
                    return " or ".join(s.value for s in meta.scenarios)

        stem_tokens = {tok for tok in re.split(r"[^a-z0-9]+", candidate.stem.lower()) if tok}
        explicit_stem_tiers = [
            tier.value
            for tier in (ScenarioTier.SMOKE, ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP)
            if tier.value in stem_tokens
        ]
        if explicit_stem_tiers:
            return " or ".join(explicit_stem_tiers)

        if pairs:
            step_sets = [set(meta.scenarios) for _, meta in pairs if meta.scenarios]
            if step_sets:
                common = set.intersection(*step_sets)
                if len(common) == 1:
                    return next(iter(common)).value
                union_set = set.union(*step_sets)
                ordered_union = [
                    tier.value
                    for tier in (ScenarioTier.SMOKE, ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP)
                    if tier in union_set
                ]
                if ordered_union:
                    return " or ".join(ordered_union)

    stem_tokens = {tok for tok in re.split(r"[^a-z0-9]+", candidate.stem.lower()) if tok}
    for tier in (ScenarioTier.SMOKE, ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP):
        if tier.value in stem_tokens:
            return tier.value
    return "smoke or integration"
