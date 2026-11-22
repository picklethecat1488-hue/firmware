"""Data models, Cargo binary metadata parser, glob board matchers, and decorators for device suites."""

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

if sys.version_info >= (3, 11):
    import tomllib
else:
    try:
        import tomli as tomllib
    except ImportError:  # pragma: no cover
        tomllib = None  # type: ignore[assignment]

DEFAULT_PACKAGE_NAME: Optional[str] = None
DEFAULT_BIN_NAME: Optional[str] = None
DEFAULT_ARM_TARGET: Optional[str] = None
DEFAULT_CHIP: Optional[str] = None

REPO_ROOT_DEFAULT: Path = Path(__file__).resolve().parents[2]

F = TypeVar("F", bound=Callable[..., Any])


def _is_unset_target_field(value: Optional[str]) -> bool:
    """Return True if a target/chip/package/binary identifier is None, empty, or 'none'."""
    cleaned = str(value or "").strip()
    return not cleaned or cleaned.lower() == "none"


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

    package: Optional[str] = DEFAULT_PACKAGE_NAME
    bin_name: Optional[str] = DEFAULT_BIN_NAME
    arm_target: Optional[str] = DEFAULT_ARM_TARGET
    chip: Optional[str] = DEFAULT_CHIP
    release: bool = False
    board_type: Optional[str] = None

    def __post_init__(self) -> None:
        """Fail fast if package, binary name, ARM target, or chip cannot be inferred."""
        for attr_name in ("package", "bin_name", "arm_target", "chip"):
            raw = getattr(self, attr_name)
            if _is_unset_target_field(raw):
                raise ValueError(
                    f"TargetBinarySpec.{attr_name} is unset ('none'); target discovery requires "
                    "explicit binary metadata in app/Cargo.toml without fallbacks."
                )
            object.__setattr__(self, attr_name, str(raw).strip())
        if self.board_type is not None:
            cleaned_bt = str(self.board_type).strip()
            object.__setattr__(self, "board_type", None if _is_unset_target_field(cleaned_bt) else cleaned_bt)

    def to_dict(self) -> Dict[str, Any]:
        """Serialize target binary specification to a JSON-compatible dictionary."""
        return {
            "package": self.package,
            "bin": self.bin_name,
            "target": self.arm_target,
            "chip": self.chip,
            "release": self.release,
            "profile": "release" if self.release else "debug",
            "board_type": self.board_type,
        }


def discover_app_binaries(repo_root: Optional[Path] = None) -> Dict[str, TargetBinarySpec]:
    """Parse target binary specifications directly from `app/Cargo.toml` and `board/board.toml` without fallbacks."""
    root = (repo_root or REPO_ROOT_DEFAULT).resolve()
    cargo_toml_path = root / "app" / "Cargo.toml"
    if not cargo_toml_path.is_file():
        raise FileNotFoundError(f"Application manifest not found at {cargo_toml_path}")

    board_type_by_board_key: Dict[str, str] = {}
    board_toml_path = root / "board" / "board.toml"
    if board_toml_path.is_file():
        boards_manifest = tomllib.loads(board_toml_path.read_text(encoding="utf-8"))
        for board_key, board_cfg in (boards_manifest.get("boards") or {}).items():
            if isinstance(board_cfg, dict) and board_cfg.get("board_type"):
                board_type_by_board_key[str(board_key).strip()] = str(board_cfg["board_type"]).strip()

    manifest = tomllib.loads(cargo_toml_path.read_text(encoding="utf-8"))
    pkg_table = manifest.get("package") or {}
    package_name = str(pkg_table.get("name") or "").strip()
    if _is_unset_target_field(package_name):
        raise ValueError(f"Missing [package].name in {cargo_toml_path}")

    metadata_bins = ((pkg_table.get("metadata") or {}).get("binaries")) or {}
    bin_entries = manifest.get("bin") or []
    if not bin_entries:
        raise ValueError(f"No [[bin]] targets declared in {cargo_toml_path}")

    specs: Dict[str, TargetBinarySpec] = {}
    for entry in bin_entries:
        bin_name = str(entry.get("name") or "").strip()
        if _is_unset_target_field(bin_name):
            raise ValueError(f"Unnamed [[bin]] entry in {cargo_toml_path}")
        bin_meta = metadata_bins.get(bin_name)
        if not isinstance(bin_meta, dict):
            raise ValueError(
                f"Binary '{bin_name}' in {cargo_toml_path} is missing "
                f"[package.metadata.binaries.{bin_name}] target/chip metadata"
            )
        board_key = str(bin_meta.get("board") or "").strip()
        inferred_board_type = board_type_by_board_key.get(board_key) or (board_key or None)
        specs[bin_name] = TargetBinarySpec(
            package=package_name,
            bin_name=bin_name,
            arm_target=bin_meta.get("target"),
            chip=bin_meta.get("chip"),
            release=bool(bin_meta.get("release", False)),
            board_type=inferred_board_type,
        )
    return specs


def resolve_binary_board_type(bin_name: Optional[str], repo_root: Optional[Path] = None) -> Optional[str]:
    """Return the canonical `board_type` associated with `bin_name` in `app/Cargo.toml` & `board/board.toml`."""
    if _is_unset_target_field(bin_name):
        return None
    assert bin_name is not None
    try:
        known = discover_app_binaries(repo_root)
    except (OSError, ValueError):
        return None
    spec = known.get(bin_name.strip())
    return spec.board_type if spec is not None else None


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
        """Construct `DetectedBoard` from a `/api/metadata` or `query_metadata` dictionary without fallbacks."""
        bt = str(payload.get("board_type") or "").strip()
        chip = str(payload.get("chip") or "").strip()
        if _is_unset_target_field(bt) or _is_unset_target_field(chip):
            return None
        return cls(
            board_type=bt,
            chip=chip,
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
    trace_mode: Optional[str] = None

    def to_dict(self, order_index: int = 1) -> Dict[str, Any]:
        """Serialize test metadata with its 1-based listing order index."""
        return {
            "order": order_index,
            "name": self.name,
            "project": self.project,
            "boards": list(self.boards),
            "scenarios": [s.value for s in self.scenarios],
            "target_binary": self.target_binary.bin_name if self.target_binary else None,
            "trace_mode": self.trace_mode,
            "manual": self.manual,
            "description": self.description,
            "lineno": self.definition_lineno,
            "file_path": self.file_path,
        }


def resolve_target_binary(
    binary: Union[None, str, TargetBinarySpec],
    *,
    repo_root: Optional[Path] = None,
) -> Optional[TargetBinarySpec]:
    """Resolve a binary name or `TargetBinarySpec` from `app/Cargo.toml` without fallbacks."""
    if binary is None:
        return None
    if isinstance(binary, TargetBinarySpec):
        return binary
    cleaned = binary.strip()
    if _is_unset_target_field(cleaned):
        raise ValueError("Target binary identifier cannot be empty or 'none'")
    known_binaries = discover_app_binaries(repo_root)
    if cleaned not in known_binaries:
        raise ValueError(
            f"Unknown target binary '{cleaned}': not declared in app/Cargo.toml "
            f"(available binaries: {sorted(known_binaries.keys())})"
        )
    return known_binaries[cleaned]


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
            if isinstance(payload, dict):
                parsed = DetectedBoard.from_metadata_response(payload)
                if parsed is not None:
                    return parsed

    kv: Dict[str, str] = {}
    for line in cleaned.splitlines():
        if ":" in line and not line.strip().startswith("partition["):
            key, val = line.split(":", 1)
            kv[key.strip().lower()] = val.strip()
    board_type = kv.get("board_type", "")
    chip = kv.get("chip", "")
    if not _is_unset_target_field(board_type) and not _is_unset_target_field(chip):
        matches_str = kv.get("matches_target", "true").lower()
        return DetectedBoard(
            board_type=board_type,
            chip=chip,
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
    trace_mode: Optional[str] = None,
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
        normalized_trace = str(trace_mode).strip().upper() if trace_mode else None
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
            trace_mode=normalized_trace,
        )
        setattr(fn, "__device_test_meta__", metadata)

        try:
            import pytest as _pytest
        except ImportError:
            return fn

        marked: Any = _pytest.mark.asyncio(fn)
        marked = _pytest.mark.project(normalized_project)(marked)
        marked = _pytest.mark.boards(*board_patterns)(marked)
        if resolved_bin is not None:
            marked = _pytest.mark.target_binary(resolved_bin.bin_name)(marked)
        if doc_summary:
            marked = _pytest.mark.description(doc_summary)(marked)
        for tier in scenario_tiers:
            marked = getattr(_pytest.mark, tier.value)(marked)
        if manual:
            marked = _pytest.mark.manual(marked)
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


def _board_filter_matches_test(board_filter: str, test_board_patterns: Sequence[str]) -> bool:
    """Return True if a user-specified board filter matches a test's `@device_test(boards=...)` patterns."""
    filt = board_filter.strip().lower()
    if not filt or filt in ("*", "all", "any"):
        return True
    if matches_board_patterns(filt, test_board_patterns):
        return True
    return any(fnmatch.fnmatchcase(pat.strip().lower(), filt) for pat in test_board_patterns)


KEYWORD_TOKEN_RE = re.compile(r"\(|\)|'[^']*'|\"[^\"]*\"|\S+")


def matches_keyword_expression(target_text: str, expression: str) -> bool:
    """Evaluate a `pytest -k` style boolean substring expression (`and`, `or`, `not`, parentheses) against `target_text`."""
    cleaned_expr = expression.strip()
    if not cleaned_expr:
        return True

    haystack = target_text.lower()
    raw_tokens = KEYWORD_TOKEN_RE.findall(cleaned_expr)
    if not raw_tokens:
        return True

    idx = 0

    def _parse_or() -> bool:
        nonlocal idx
        result = _parse_and()
        while idx < len(raw_tokens) and raw_tokens[idx].lower() == "or":
            idx += 1
            rhs = _parse_and()
            result = result or rhs
        return result

    def _parse_and() -> bool:
        nonlocal idx
        result = _parse_not()
        while idx < len(raw_tokens) and raw_tokens[idx].lower() == "and":
            idx += 1
            rhs = _parse_not()
            result = result and rhs
        return result

    def _parse_not() -> bool:
        nonlocal idx
        if idx < len(raw_tokens) and raw_tokens[idx].lower() == "not":
            idx += 1
            return not _parse_not()
        return _parse_atom()

    def _parse_atom() -> bool:
        nonlocal idx
        if idx >= len(raw_tokens):
            return True
        tok = raw_tokens[idx]
        if tok == "(":
            idx += 1
            val = _parse_or()
            if idx < len(raw_tokens) and raw_tokens[idx] == ")":
                idx += 1
            return val
        idx += 1
        if len(tok) >= 2 and ((tok[0] == "'" and tok[-1] == "'") or (tok[0] == '"' and tok[-1] == '"')):
            tok = tok[1:-1]
        return tok.lower() in haystack

    result = _parse_or()
    while idx < len(raw_tokens):
        extra = raw_tokens[idx].strip("'\"()").lower()
        idx += 1
        if extra and extra not in ("and", "or", "not"):
            result = result and (extra in haystack)
    return result


def discover_device_tests(
    repo_root: Path,
    *,
    project: Optional[str] = None,
    script: Optional[Union[str, Path]] = None,
    scenarios: Optional[Sequence[Union[str, ScenarioTier]]] = None,
    board: Optional[str] = None,
    target_binary: Optional[str] = None,
    manual: Optional[bool] = None,
    query: Optional[str] = None,
    keyword: Optional[str] = None,
    detected_board_type: Optional[str] = None,
) -> List[Dict[str, Any]]:
    """Discover `@device_test` functions across `tools/device_tests/suites/` with attribute filtering."""
    project_norm = project.strip().lower() if project and project.strip().lower() not in ("all", "*") else None
    scenario_filter = (
        {s if isinstance(s, ScenarioTier) else ScenarioTier(str(s).strip().lower()) for s in scenarios}
        if scenarios
        else None
    )
    board_norm = board.strip() if board and board.strip().lower() not in ("all", "*") else None
    if board_norm and board_norm.lower() == "auto":
        board_norm = detected_board_type
    binary_norm = target_binary.strip() if target_binary and target_binary.strip().lower() not in ("all", "*") else None
    query_norm = query.strip() if query and query.strip() else None
    keyword_norm = keyword.strip() if keyword and keyword.strip() else None

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
            if board_norm and not _board_filter_matches_test(board_norm, meta.boards):
                continue
            if binary_norm is not None:
                actual_bin = meta.target_binary.bin_name if meta.target_binary else "none"
                if actual_bin.lower() != binary_norm.lower():
                    continue
            if manual is not None and bool(meta.manual) != bool(manual):
                continue
            node_id = f"{rel_path}::{meta.name}"
            if keyword_norm and not matches_keyword_expression(node_id, keyword_norm):
                continue
            if query_norm:
                haystack = f"{meta.name} {meta.project} {meta.description} {rel_path}"
                if not matches_keyword_expression(haystack, query_norm):
                    continue

            order_counter += 1
            entry = meta.to_dict(order_index=order_counter)
            entry["file_path"] = rel_path
            entry["node_id"] = node_id
            entry["board_compatible"] = (
                matches_board_patterns(detected_board_type, meta.boards) if detected_board_type else None
            )
            results.append(entry)
    return results


def infer_scenario_from_script(
    script_spec: Union[str, Path],
    repo_root: Optional[Path] = None,
) -> str:
    """Infer the pytest scenario marker expression from a `@device_test` script or raise `ValueError`."""
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

    raise ValueError(
        f"Could not infer scenario tier from script '{script_spec}'; "
        "decorate tests with @device_test(scenarios=...) or pass --scenario explicitly."
    )
