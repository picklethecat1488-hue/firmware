"""Pytest configuration, top-to-bottom source ordering, project/board filtering, and async fixtures."""

import asyncio
import datetime
import os
from pathlib import Path
from typing import Any, AsyncIterator, Dict, List, Optional, Tuple

import pytest
import pytest_asyncio

from device_tests.client import AsyncHostSessionClient, DEFAULT_DAEMON_HOST
from device_tests.manual import ManualTestHelper
from device_tests.model import (
    DetectedBoard,
    TargetBinarySpec,
    get_device_test_metadata,
    matches_board_patterns,
    resolve_target_binary,
)

REPO_ROOT: Path = Path(__file__).resolve().parents[2]


def pytest_addoption(parser: pytest.Parser) -> None:
    """Register CLI flags for on-device test execution, project/board filtering, and reporting."""
    group = parser.getgroup("device_tests", "On-device hardware-in-the-loop test options")
    group.addoption(
        "--project",
        action="store",
        default="all",
        help="Project identifier filter matching @device_test(project=...) (e.g. cat_detector, carrier_board).",
    )
    group.addoption(
        "--board",
        action="store",
        default="auto",
        help="Target board_type or glob filter (e.g. auto, rp2040*, rp2040_flash_2mb, carrier_board).",
    )
    group.addoption(
        "--host-addr",
        action="store",
        default=DEFAULT_DAEMON_HOST,
        help="Hostname/IP of the host.py daemon (default: 127.0.0.1).",
    )
    group.addoption(
        "--host-port",
        action="store",
        type=int,
        default=None,
        help="Explicit TCP port of a running or to-be-spawned host.py daemon.",
    )
    group.addoption(
        "--manual",
        action="store_true",
        default=False,
        help="Enable operator prompts for @device_test(manual=True) benchtop hardware steps.",
    )
    group.addoption(
        "--report-md",
        action="store",
        default=None,
        help="Optional path to export a Markdown verification report upon completion.",
    )
    group.addoption(
        "--debug-server",
        dest="enable_debug_server",
        action="store_true",
        default=None,
        help="Enable symbolic target GDB server (host_debug + :50000 RTT bridge) on the session daemon.",
    )
    group.addoption(
        "--no-debug-server",
        dest="enable_debug_server",
        action="store_false",
        help="Disable symbolic target GDB server and use direct probe-rs RTT streaming on the session daemon.",
    )


def pytest_configure(config: pytest.Config) -> None:
    """Initialize session report accumulator on `config`."""
    setattr(config, "_device_report_entries", [])


def pytest_collection_modifyitems(
    session: pytest.Session,
    config: pytest.Config,
    items: List[pytest.Item],
) -> None:
    """Enforce deterministic top-to-bottom source-line execution order and `--project` filtering."""
    del session
    module_appearance_order: Dict[str, int] = {}
    for item in items:
        mod_path = str(item.location[0])
        if mod_path not in module_appearance_order:
            module_appearance_order[mod_path] = len(module_appearance_order)

    def _sort_key(item: pytest.Item) -> Tuple[int, int]:
        mod_idx = module_appearance_order.get(str(item.location[0]), 0)
        meta = get_device_test_metadata(item)
        lineno = meta.definition_lineno if meta is not None else int(item.location[1] or 0)
        return (mod_idx, lineno)

    items.sort(key=_sort_key)

    project_filter = str(config.getoption("--project") or "all").strip().lower()
    if project_filter and project_filter not in ("all", "*"):
        kept: List[pytest.Item] = []
        deselected: List[pytest.Item] = []
        for item in items:
            meta = get_device_test_metadata(item)
            proj_mark = item.get_closest_marker("project")
            item_project = (
                meta.project
                if meta is not None
                else (str(proj_mark.args[0]).strip().lower() if proj_mark and proj_mark.args else "")
            )
            if item_project == project_filter:
                kept.append(item)
            else:
                deselected.append(item)
        if deselected:
            config.hook.pytest_deselected(items=deselected)
        items[:] = kept

    manual_enabled = bool(config.getoption("--manual"))
    for order_idx, item in enumerate(items, start=1):
        meta = get_device_test_metadata(item)
        item.user_properties.append(("order", str(order_idx)))
        if meta is not None:
            item.user_properties.append(("project", meta.project))
            item.user_properties.append(("description", meta.description))
            item.user_properties.append(("boards", ",".join(meta.boards)))
            item.user_properties.append(("scenarios", ",".join(s.value for s in meta.scenarios)))
            if meta.target_binary is not None:
                item.user_properties.append(("target_binary", meta.target_binary.bin_name))
        else:
            desc_mark = item.get_closest_marker("description")
            if desc_mark and desc_mark.args:
                item.user_properties.append(("description", str(desc_mark.args[0])))
            bin_mark = item.get_closest_marker("target_binary")
            if bin_mark and bin_mark.args:
                resolved = resolve_target_binary(bin_mark.args[0], repo_root=REPO_ROOT)
                if resolved is not None:
                    item.user_properties.append(("target_binary", resolved.bin_name))

        if not manual_enabled and item.get_closest_marker("manual") is not None:
            item.add_marker(pytest.mark.skip(reason="Manual benchtop step skipped (pass --manual to prompt operator)"))


@pytest_asyncio.fixture(scope="session")
async def host(pytestconfig: pytest.Config) -> AsyncIterator[AsyncHostSessionClient]:
    """Provide a session-scoped `AsyncHostSessionClient` reusing any running host.py daemon."""
    host_addr = str(pytestconfig.getoption("--host-addr"))
    host_port = pytestconfig.getoption("--host-port")
    manual_enabled = bool(pytestconfig.getoption("--manual"))
    enable_debug_server = pytestconfig.getoption("enable_debug_server")

    from device_tests.client import query_metadata_standalone
    from host_daemon.cli import get_lock_data

    spawn_needed = True
    if host_port is None and get_lock_data() is None:
        probe_client = await AsyncHostSessionClient.connect(
            REPO_ROOT,
            host=host_addr,
            port=None,
            manual_enabled=manual_enabled,
            spawn_if_needed=False,
        )
        try:
            status = await probe_client.get_status()
            daemon_listening = status.get("status") == "running"
        except (OSError, RuntimeError):
            daemon_listening = False

        if not daemon_listening and os.environ.get("XERXES_REQUIRE_CONNECTED_TARGET") != "1":
            standalone_board = query_metadata_standalone(REPO_ROOT)
            if standalone_board is None:
                spawn_needed = False

    client = await AsyncHostSessionClient.connect(
        REPO_ROOT,
        host=host_addr,
        port=host_port,
        manual_enabled=manual_enabled,
        spawn_if_needed=spawn_needed,
        enable_debug_server=enable_debug_server,
    )
    try:
        yield client
    finally:
        await client.close()


@pytest_asyncio.fixture(scope="session")
async def detected_board(
    pytestconfig: pytest.Config,
    host: AsyncHostSessionClient,
) -> Optional[DetectedBoard]:
    """Query `board_type` once per session via `host.py` (`/api/metadata` -> `host_fs query_metadata`)."""
    board = await host.query_metadata()
    if board is None and os.environ.get("XERXES_REQUIRE_CONNECTED_TARGET") == "1":
        await asyncio.sleep(0.5)
        board = await host.query_metadata(force_refresh=True)
    cli_board_filter = str(pytestconfig.getoption("--board") or "auto").strip()
    if board is not None and cli_board_filter.lower() != "auto":
        if not matches_board_patterns(board.board_type, [cli_board_filter]):
            return None
    return board


@pytest_asyncio.fixture(autouse=True)
async def _prepare_device_test(
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
    host: AsyncHostSessionClient,
    detected_board: Optional[DetectedBoard],
) -> AsyncIterator[None]:
    """Enforce board-glob auto-skip and idempotent `target_binary` flashing before each test."""
    meta = get_device_test_metadata(request.node)
    boards_marker = request.node.get_closest_marker("boards")
    patterns: Tuple[str, ...] = ()
    if meta is not None:
        patterns = meta.boards
    elif boards_marker is not None:
        patterns = tuple(str(arg) for arg in boards_marker.args)

    if not patterns and meta is None and request.node.get_closest_marker("target_binary") is None:
        yield
        return

    cli_board_filter = str(pytestconfig.getoption("--board") or "auto").strip()
    if detected_board is None:
        if os.environ.get("XERXES_REQUIRE_CONNECTED_TARGET") == "1":
            pytest.fail("Target hardware disconnected after discovery (host_fs query_metadata returned None)")
        if cli_board_filter.lower() != "auto":
            pytest.skip(f"No connected target matching --board='{cli_board_filter}'")
        pytest.skip("No physical target device detected via host_fs query_metadata")

    assert detected_board is not None
    if patterns and not matches_board_patterns(detected_board.board_type, patterns):
        pytest.skip(
            f"Target board '{detected_board.board_type}' does not match required board pattern(s) {list(patterns)}"
        )

    bin_spec: Optional[TargetBinarySpec] = meta.target_binary if meta is not None else None
    if bin_spec is None:
        bin_marker = request.node.get_closest_marker("target_binary")
        if bin_marker and bin_marker.args:
            bin_spec = resolve_target_binary(bin_marker.args[0], repo_root=REPO_ROOT)

    if bin_spec is not None:
        await host.ensure_target_binary(bin_spec)

    yield


@pytest_asyncio.fixture
async def manual(
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
    host: AsyncHostSessionClient,
) -> ManualTestHelper:
    """Provide a standard pytest manual verification helper with periodic `host.py` heartbeats."""
    meta = get_device_test_metadata(request.node)
    project = meta.project if meta is not None else "unknown"
    return ManualTestHelper(
        host=host,
        test_name=request.node.name,
        project=project,
        manual_enabled=bool(pytestconfig.getoption("--manual")),
    )


@pytest.hookimpl(hookwrapper=True)
def pytest_runtest_makereport(item: pytest.Item, call: pytest.CallInfo[None]) -> Any:
    """Record per-test outcomes for optional `--report-md` Markdown export."""
    outcome = yield
    report = outcome.get_result()
    entries: Optional[List[Dict[str, Any]]] = getattr(item.config, "_device_report_entries", None)
    if entries is None:
        return

    if report.when == "call" or (report.when == "setup" and report.skipped):
        meta = get_device_test_metadata(item)
        status_label = "PASSED" if report.passed else ("SKIPPED" if report.skipped else "FAILED")
        detail = ""
        if report.skipped and isinstance(report.longrepr, tuple) and len(report.longrepr) >= 3:
            detail = str(report.longrepr[2])
        elif report.failed:
            detail = str(report.longreprtext)[:300]

        entries.append(
            {
                "order": len(entries) + 1,
                "name": item.name,
                "project": meta.project if meta else "unknown",
                "boards": ",".join(meta.boards) if meta else "*",
                "scenarios": ",".join(s.value for s in meta.scenarios) if meta else "",
                "target_binary": (meta.target_binary.bin_name if (meta and meta.target_binary) else "none"),
                "description": meta.description if meta else (item.obj.__doc__ or "").strip(),
                "status": status_label,
                "duration_s": round(float(report.duration), 3),
                "detail": detail,
            }
        )


def pytest_sessionfinish(session: pytest.Session, exitstatus: int) -> None:
    """Write Markdown verification report when `--report-md <path>` is passed."""
    del exitstatus
    report_path_raw = session.config.getoption("--report-md")
    if not report_path_raw:
        return

    entries: List[Dict[str, Any]] = getattr(session.config, "_device_report_entries", [])
    out_path = Path(str(report_path_raw))
    if not out_path.is_absolute():
        out_path = REPO_ROOT / out_path
    out_path.parent.mkdir(parents=True, exist_ok=True)

    now_iso = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
    passed = sum(1 for e in entries if e["status"] == "PASSED")
    failed = sum(1 for e in entries if e["status"] == "FAILED")
    skipped = sum(1 for e in entries if e["status"] == "SKIPPED")

    lines = [
        "# On-Device Verification Report",
        "",
        f"- **Generated**: {now_iso}",
        f"- **Summary**: {passed} Passed, {failed} Failed, {skipped} Skipped ({len(entries)} Total)",
        "",
        "| # | Project | Test | Scenarios | Boards | Binary | Status | Duration |",
        "| --- | --- | --- | --- | --- | --- | --- | --- |",
    ]
    for e in entries:
        lines.append(
            f"| {e['order']} | `{e['project']}` | `{e['name']}` | `{e['scenarios']}` | "
            f"`{e['boards']}` | `{e['target_binary']}` | **{e['status']}** | {e['duration_s']:.2f}s |"
        )
    lines.append("")
    lines.append("## Test Descriptions & Notes")
    lines.append("")
    for e in entries:
        lines.append(f"### [{e['order']}] `{e['name']}` — {e['status']}")
        if e["description"]:
            lines.append(f"- **Description**: {e['description']}")
        lines.append(f"- **Project**: `{e['project']}` | **Target Binary**: `{e['target_binary']}`")
        if e["detail"]:
            lines.append(f"- **Detail**: `{e['detail']}`")
        lines.append("")

    out_path.write_text("\n".join(lines), encoding="utf-8")
