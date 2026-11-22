"""HTTP server and REST API for Xerxes Host Daemon workstation."""

from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
from pathlib import Path
import sys
import threading
import time
from typing import Any, Dict
import urllib.parse

import jinja2

dashboard_dir = Path(__file__).resolve().parents[3] / "dashboard"
if str(dashboard_dir) not in sys.path:
    sys.path.insert(0, str(dashboard_dir))

from host_daemon.model.trace_mode import (
    TRACE_MODE_DESCRIPTORS,
    TraceMode,
    build_perfetto_deep_link,
    get_external_trace_viewer,
    normalize_trace_mode,
)
from host_daemon.provider.daemon.bug_report_service import create_target_worm_report
from host_daemon.provider.daemon.rerun_bridge import open_trace_in_rerun
from host_daemon.provider.daemon.runner import (
    DEFAULT_CLI_OUTPUT_TIMEOUT_SEC,
    DaemonRunner,
    open_url_in_system_browser,
)
from host_daemon.provider.daemon.test_manager import DeviceTestRunnerManager
from host_daemon.provider.daemon.trace_store import TraceSessionStore


def _get_test_manager(runner: Any) -> DeviceTestRunnerManager:
    """Return `runner.test_manager`, lazily attaching a `DeviceTestRunnerManager` for mock/test runners."""
    existing = getattr(runner, "test_manager", None)
    if isinstance(existing, DeviceTestRunnerManager):
        return existing
    created = DeviceTestRunnerManager(runner)
    setattr(runner, "test_manager", created)
    return created


def _get_trace_store(runner: Any) -> TraceSessionStore:
    """Return `runner.trace_store`, lazily attaching a `TraceSessionStore` for mock/test runners."""
    existing = getattr(runner, "trace_store", None)
    if isinstance(existing, TraceSessionStore):
        return existing
    created = TraceSessionStore()
    setattr(runner, "trace_store", created)
    return created


def _get_runner_trace_mode(runner: Any) -> TraceMode:
    """Return the normalized `TraceMode` on `runner` (defaulting to `TraceMode.NONE`)."""
    raw = getattr(runner, "trace_mode", TraceMode.NONE)
    return raw if isinstance(raw, TraceMode) else normalize_trace_mode(str(raw))


def _resolve_runner_device_ts(runner: Any) -> str:
    """Return a target monotonic device timestamp string (`e.g. '0.412000s'`) for `runner`."""
    if hasattr(runner, "get_device_timestamp_str"):
        return str(runner.get_device_timestamp_str())
    anchor = getattr(runner, "_sim_boot_mono", None)
    now = time.monotonic()
    if anchor is None:
        anchor = now
        setattr(runner, "_sim_boot_mono", anchor)
    return f"{max(0.0, now - float(anchor)):.6f}s"


HIGH_FREQUENCY_POLL_PATHS: frozenset[str] = frozenset(
    {
        "/api/status",
        "/api/sync",
        "/api/sync/status",
        "/api/logs",
        "/api/cli/output",
        "/api/fs/output",
        "/api/manual_prompt",
        "/api/tests/status",
        "/api/trace/events",
    }
)


def _record_runner_api_call(
    server: Any,
    method: str,
    path: str,
    status: int,
    duration_ms: float,
    detail: str = "",
) -> None:
    """Log `/api/*` invocations to `runner.log_api_invocation`, rate-limiting high-frequency UI pollers."""
    if not path.startswith("/api/"):
        return
    runner = getattr(server, "runner", None)
    if runner is None or not hasattr(runner, "log_api_invocation"):
        return
    if method == "GET" and path in HIGH_FREQUENCY_POLL_PATHS and status < 400:
        seen = getattr(server, "_seen_poll_paths", None)
        if not isinstance(seen, set):
            seen = set()
            setattr(server, "_seen_poll_paths", seen)
        if path in seen:
            return
        seen.add(path)
    runner.log_api_invocation(method, path, status, duration_ms, detail)


class HostDaemonRequestHandler(BaseHTTPRequestHandler):
    """HTTP request handler for host daemon web workstation and IPC APIs."""

    server: "HostDaemonServer"

    def log_message(self, format: str, *args) -> None:  # noqa: A002
        """Suppress default HTTP logging to keep console clean."""
        return

    def send_response(self, code: int, message: Any = None) -> None:
        """Record HTTP status code for structured API telemetry before delegating to BaseHTTPRequestHandler."""
        self._last_http_status = int(code)
        super().send_response(code, message)

    def _flush_api_log_once(self) -> None:
        """Flush structured `/api/*` telemetry to `host_daemon.log` before socket write completes."""
        if getattr(self, "_api_logged", True):
            return
        self._api_logged = True
        start_t = getattr(self, "_req_start_mono", None)
        elapsed_ms = (time.monotonic() - float(start_t)) * 1000.0 if start_t is not None else 0.0
        _record_runner_api_call(
            self.server,
            str(getattr(self, "_req_method", "GET")),
            str(getattr(self, "_req_path", "")),
            int(getattr(self, "_last_http_status", 200)),
            elapsed_ms,
            str(getattr(self, "_req_detail", "")),
        )

    def _send_json(self, data: Any, status: int = 200) -> None:
        payload = json.dumps(data).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(payload)))
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        self.end_headers()
        self._flush_api_log_once()
        self.wfile.write(payload)

    def do_OPTIONS(self) -> None:  # noqa: N802
        """Respond to CORS and Private Network preflight requests (e.g. from `https://ui.perfetto.dev`)."""
        self.send_response(204)
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "*")
        self.send_header("Access-Control-Allow-Private-Network", "true")
        self.send_header("Cache-Control", "no-cache, no-store, must-revalidate")
        self.end_headers()

    def do_GET(self) -> None:  # noqa: N802
        """Handle GET requests for UI and telemetry APIs."""
        self._req_start_mono = time.monotonic()
        self._req_method = "GET"
        self._req_path = self.path.split("?")[0]
        self._req_detail = ""
        self._last_http_status = 200
        self._api_logged = False
        try:
            self._handle_get(self._req_path)
        finally:
            self._flush_api_log_once()

    def _handle_get(self, path: str) -> None:
        """Dispatch GET requests across UI, static, and telemetry endpoints."""
        runner = self.server.runner

        if path in ("/", "/index.html"):
            try:
                from host_daemon.provider.daemon.config import discover_target_packages, load_last_session

                template = self.server.jinja_env.get_template("host_daemon.html.j2")
                active_tmode = _get_runner_trace_mode(runner)
                html = template.render(
                    runner=runner,
                    package_selection_mode=getattr(runner, "package_selection_mode", False),
                    target_packages=discover_target_packages(runner.repo_root),
                    last_session=load_last_session(runner.repo_root),
                    trace_modes=list(TRACE_MODE_DESCRIPTORS),
                    active_trace_mode=active_tmode.value,
                    active_external_viewer=get_external_trace_viewer(active_tmode).value,
                )
                body = html.encode("utf-8")
                self.send_response(200)
                self.send_header("Content-Type", "text/html; charset=utf-8")
                self.send_header("Content-Length", str(len(body)))
                self.send_header("Cache-Control", "no-cache, no-store, must-revalidate")
                self.send_header("Pragma", "no-cache")
                self.send_header("Expires", "0")
                self.end_headers()
                self.wfile.write(body)
            except Exception as e:
                self.send_error(500, f"Template rendering error: {e}")
        elif path == "/api/status":
            self._send_json(runner.get_status())
        elif path == "/api/boot_status":
            if hasattr(runner, "query_boot_status"):
                self._send_json(runner.query_boot_status(refresh_probe=True))
            else:
                self._send_json({"state": "unknown", "boot_ready": False})
        elif path == "/api/logs/files":
            if hasattr(runner, "get_log_files"):
                self._send_json({"log_dir": str(getattr(runner, "log_dir", "")), "files": runner.get_log_files()})
            else:
                self._send_json({"log_dir": "", "files": {}})
        elif path == "/api/targets":
            from host_daemon.provider.daemon.config import discover_target_packages, load_last_session

            dbg_status = runner.get_debug_status() if hasattr(runner, "get_debug_status") else {}
            cur_tmode = _get_runner_trace_mode(runner)
            self._send_json(
                {
                    "packages": discover_target_packages(runner.repo_root),
                    "trace_modes": list(TRACE_MODE_DESCRIPTORS),
                    "last_session": load_last_session(runner.repo_root),
                    "current": {
                        "package": runner.package,
                        "bin": runner.bin_name,
                        "release": runner.release,
                        "target": runner.target,
                        "speed": runner.speed,
                        "chip": runner.chip,
                        "trace_mode": cur_tmode.value,
                        "external_viewer": get_external_trace_viewer(cur_tmode).value,
                        "show_system_logs": getattr(runner, "show_system_logs", False),
                        "enable_debug_server": getattr(runner, "enable_debug_server", False),
                        "halt_on_reset": getattr(runner, "halt_on_reset", False),
                        "gdb_core0_port": getattr(runner, "gdb_core0_port", 3333),
                        "gdb_core1_port": getattr(runner, "gdb_core1_port", 3334),
                        "rtt_bridge_port": getattr(runner, "rtt_bridge_port", 50000),
                        "debug_server": dbg_status,
                    },
                    "package_selection_mode": getattr(runner, "package_selection_mode", False),
                }
            )
        elif path == "/api/debug/status":
            if hasattr(runner, "get_debug_status"):
                self._send_json(runner.get_debug_status())
            else:
                self._send_json(
                    {
                        "enabled": bool(getattr(runner, "enable_debug_server", False)),
                        "active": bool(getattr(runner, "debug_server_active", False)),
                        "halt_on_reset": bool(getattr(runner, "halt_on_reset", False)),
                        "gdb_core0_port": int(getattr(runner, "gdb_core0_port", 3333)),
                        "gdb_core1_port": int(getattr(runner, "gdb_core1_port", 3334)),
                        "rtt_bridge_port": int(getattr(runner, "rtt_bridge_port", 50000)),
                    }
                )
        elif path in ("/api/sync", "/api/sync/status"):
            with runner._lock:
                self._send_json(dict(runner.sync_status))
        elif path == "/api/logs":
            with runner._lock:
                self._send_json(list(runner.logs))
        elif path == "/api/cli/output":
            with runner._lock:
                self._send_json(list(runner.cli_outputs))
        elif path == "/api/cli/history":
            with runner._lock:
                self._send_json({"history": list(getattr(runner, "command_history", []))})
        elif path == "/api/fs/output":
            with runner._lock:
                self._send_json(list(runner.fs_outputs))
        elif path == "/api/manual_prompt":
            if hasattr(runner, "manual_prompt"):
                with runner._lock:
                    self._send_json(runner.manual_prompt.to_dict())
            else:
                self._send_json({"active": False})
        elif path == "/api/metadata":
            if hasattr(runner, "query_target_metadata"):
                self._send_json(runner.query_target_metadata())
            else:
                self._send_json({"connected": False, "matches_target": False})
        elif path == "/api/tests":
            raw_qs = self.path.partition("?")[2]
            qs = urllib.parse.parse_qs(raw_qs)
            filters: Dict[str, Any] = {
                "project": qs.get("project", [None])[0],
                "scenario": qs.get("scenario", [None])[0] or qs.get("scenarios", [None])[0],
                "board": qs.get("board", [None])[0],
                "binary": qs.get("binary", [None])[0] or qs.get("target_binary", [None])[0],
                "manual": qs.get("manual", [None])[0],
                "query": qs.get("q", [None])[0] or qs.get("query", [None])[0],
            }
            probe_flag = str(qs.get("probe", ["0"])[0]).lower() in ("1", "true", "yes")
            self._send_json(_get_test_manager(runner).scan_tests(filters, probe_board=probe_flag))
        elif path == "/api/tests/status":
            self._send_json(_get_test_manager(runner).get_status())
        elif path == "/api/trace/events":
            raw_qs = self.path.partition("?")[2]
            qs = urllib.parse.parse_qs(raw_qs)
            raw_after = qs.get("after_seq", qs.get("after_id", ["0"]))[0]
            raw_limit = qs.get("limit", ["10000"])[0]
            raw_epoch = qs.get("session_epoch", [None])[0]
            after_seq = int(raw_after) if str(raw_after).isdigit() else 0
            limit_val = int(raw_limit) if str(raw_limit).isdigit() else 10_000
            session_epoch = int(raw_epoch) if raw_epoch is not None and str(raw_epoch).isdigit() else None
            if hasattr(runner, "get_trace_events"):
                self._send_json(
                    runner.get_trace_events(
                        after_seq=after_seq,
                        limit=limit_val,
                        session_epoch=session_epoch,
                    )
                )
            else:
                store = _get_trace_store(runner)
                cur_tmode = _get_runner_trace_mode(runner)
                data = store.get_events(
                    after_seq=after_seq,
                    limit=limit_val,
                    session_epoch=session_epoch,
                )
                data["trace_mode"] = cur_tmode.value
                data["external_viewer"] = get_external_trace_viewer(cur_tmode).value
                data["chip"] = getattr(runner, "chip", None)
                data["bin"] = getattr(runner, "bin_name", None)
                self._send_json(data)
        elif path in ("/api/trace/perfetto.json", "/api/trace/download"):
            raw_qs = self.path.partition("?")[2]
            qs = urllib.parse.parse_qs(raw_qs)
            fmt = "perfetto" if path == "/api/trace/perfetto.json" else str(qs.get("format", ["perfetto"])[0]).lower()
            store = _get_trace_store(runner)
            cur_tmode = _get_runner_trace_mode(runner)
            stem = f"{getattr(runner, 'bin_name', None) or 'target'}_{cur_tmode.value.lower()}"
            if fmt == "csv":
                body = store.format_csv().encode("utf-8")
                ctype = "text/csv; charset=utf-8"
                fname = f"{stem}.csv"
            else:
                body = store.format_perfetto_json(chip=getattr(runner, "chip", None)).encode("utf-8")
                ctype = "application/json; charset=utf-8"
                fname = f"{stem}.perfetto.json"
            self.send_response(200)
            self.send_header("Content-Type", ctype)
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Access-Control-Allow-Origin", "*")
            self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
            self.send_header("Access-Control-Allow-Headers", "*")
            self.send_header("Access-Control-Allow-Private-Network", "true")
            self.send_header("Cache-Control", "no-cache, no-store, must-revalidate")
            if path == "/api/trace/download":
                self.send_header("Content-Disposition", f'attachment; filename="{fname}"')
            self.end_headers()
            self.wfile.write(body)
        elif (
            path.startswith("/js/")
            or path.startswith("/static/")
            or path in ("/favicon.ico", "/favicon.svg", "/manifest.json")
        ):
            from provider.http_static import (
                get_firmware_daemon_static_dir,
                get_firmware_js_root,
                get_firmware_static_dir,
                serve_static_asset,
            )

            mapped_path = f"/static{path}" if path in ("/favicon.ico", "/favicon.svg", "/manifest.json") else path
            serve_static_asset(
                self,
                mapped_path,
                js_root=get_firmware_js_root(runner.repo_root),
                static_dirs=[
                    get_firmware_daemon_static_dir(runner.repo_root),
                    get_firmware_static_dir(runner.repo_root),
                ],
                cache_control="no-cache, no-store, must-revalidate",
            )
        elif path.startswith("/attachments/"):
            clean_path = path.lstrip("/")
            rel_name = clean_path[len("attachments/") :]
            repo_root = runner.repo_root
            file_path = repo_root / "attachments" / rel_name
            if not file_path.exists() or not file_path.is_file():
                fallback_att = repo_root / "feedback" / "attachments" / rel_name
                if fallback_att.exists() and fallback_att.is_file():
                    file_path = fallback_att
            if file_path.exists() and file_path.is_file():
                ext = file_path.suffix.lower()
                content_types = {
                    ".png": "image/png",
                    ".jpg": "image/jpeg",
                    ".jpeg": "image/jpeg",
                    ".gif": "image/gif",
                    ".svg": "image/svg+xml",
                    ".pdf": "application/pdf",
                    ".csv": "text/csv",
                    ".log": "text/plain",
                    ".txt": "text/plain",
                    ".json": "application/json",
                }
                ctype = content_types.get(ext, "application/octet-stream")
                data = file_path.read_bytes()
                self.send_response(200)
                self.send_header("Content-Type", ctype)
                self.send_header("Content-Length", str(len(data)))
                self.end_headers()
                self.wfile.write(data)
            else:
                self.send_error(404, "Attachment not found")
        else:
            self.send_error(404, "Endpoint not found")

    def do_POST(self) -> None:  # noqa: N802
        """Handle POST requests for command execution and controls."""
        self._req_start_mono = time.monotonic()
        self._req_method = "POST"
        self._req_path = self.path.split("?")[0]
        self._req_detail = ""
        self._last_http_status = 200
        self._api_logged = False
        try:
            self._req_detail = self._handle_post(self._req_path)
        finally:
            self._flush_api_log_once()

    def _handle_post(self, path: str) -> str:
        """Dispatch POST requests across target control, CLI/FS, debug, trace, and bug report endpoints."""
        runner = self.server.runner

        content_len = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(content_len).decode("utf-8") if content_len > 0 else "{}"
        try:
            payload = json.loads(body) if body else {}
        except Exception:
            payload = {}

        api_detail = ""
        if isinstance(payload, dict):
            if "mode" in payload and path == "/api/mode":
                api_detail = f"mode={payload.get('mode')}"
            elif "action" in payload and path == "/api/trace/control":
                api_detail = f"action={payload.get('action')}"
            elif "command" in payload and path in ("/api/cli/execute", "/api/fs/execute"):
                api_detail = f"cmd={str(payload.get('command'))[:64]!r}"
            elif path in ("/api/sync", "/api/target/configure") and (payload.get("bin") or payload.get("bin_name")):
                api_detail = f"bin={payload.get('bin') or payload.get('bin_name')}"
        self._req_detail = api_detail

        if path == "/api/sync":
            if payload:
                runner.update_config(
                    target=payload.get("target"),
                    package=payload.get("package"),
                    bin_name=payload.get("bin") or payload.get("bin_name"),
                    release=payload.get("release"),
                    speed=payload.get("speed"),
                    chip=payload.get("chip"),
                    enable_debug_server=payload.get("enable_debug_server"),
                    halt_on_reset=payload.get("halt_on_reset"),
                    gdb_core0_port=payload.get("gdb_core0_port"),
                    gdb_core1_port=payload.get("gdb_core1_port"),
                    rtt_bridge_port=payload.get("rtt_bridge_port"),
                    probe_selector=payload.get("probe"),
                    trace_mode=payload.get("trace_mode"),
                )
                try:
                    from host_daemon.cli import get_lock_data, write_lock_file

                    lock_data = get_lock_data() or {}
                    lock_data.update(
                        {
                            "target": runner.target,
                            "package": runner.package,
                            "bin": runner.bin_name,
                            "profile": "release" if runner.release else "debug",
                            "trace_mode": _get_runner_trace_mode(runner).value,
                            "speed": runner.speed,
                        }
                    )
                    write_lock_file(lock_data)
                except Exception:
                    pass
            runner.sync_async()
            self._send_json({"success": True, "sync": runner.sync_status})
        elif path == "/api/target/configure":
            from host_daemon.provider.daemon.config import infer_arm_target, save_last_session

            pkg = payload.get("package") or runner.package
            bin_name = payload.get("bin") or payload.get("bin_name") or runner.bin_name
            rel = payload.get("release") if "release" in payload else runner.release
            spd = payload.get("speed") or runner.speed
            chp = payload.get("chip") or runner.chip
            tgt = payload.get("target")
            show_logs = payload.get("show_system_logs")
            enable_dbg = (
                payload.get("enable_debug_server")
                if "enable_debug_server" in payload
                else getattr(runner, "enable_debug_server", False)
            )
            halt_rst = (
                payload.get("halt_on_reset") if "halt_on_reset" in payload else getattr(runner, "halt_on_reset", False)
            )
            c0_port = payload.get("gdb_core0_port") or getattr(runner, "gdb_core0_port", 3333)
            c1_port = payload.get("gdb_core1_port") or getattr(runner, "gdb_core1_port", 3334)
            rtt_port = payload.get("rtt_bridge_port") or getattr(runner, "rtt_bridge_port", 50000)
            probe_sel = payload.get("probe") if "probe" in payload else getattr(runner, "probe_selector", None)
            try:
                resolved_tm = (
                    normalize_trace_mode(payload.get("trace_mode"))
                    if "trace_mode" in payload and payload.get("trace_mode") is not None
                    else _get_runner_trace_mode(runner)
                )
            except ValueError as exc:
                self._send_json({"error": str(exc)}, status=400)
                return api_detail

            if not tgt:
                tgt = infer_arm_target(pkg, bin_name, chip=chp, repo_root=runner.repo_root)

            runner.update_config(
                target=tgt,
                package=pkg,
                bin_name=bin_name,
                release=rel,
                speed=spd,
                chip=chp,
                show_system_logs=show_logs,
                enable_debug_server=enable_dbg,
                halt_on_reset=halt_rst,
                gdb_core0_port=int(c0_port),
                gdb_core1_port=int(c1_port),
                rtt_bridge_port=int(rtt_port),
                probe_selector=probe_sel,
                trace_mode=resolved_tm.value,
            )
            runner.package_selection_mode = False
            save_last_session(
                runner.repo_root,
                {
                    "package": pkg,
                    "bin": bin_name,
                    "release": rel,
                    "target": tgt,
                    "speed": spd,
                    "chip": chp,
                    "trace_mode": resolved_tm.value,
                    "enable_debug_server": bool(enable_dbg),
                    "halt_on_reset": bool(halt_rst),
                    "gdb_core0_port": int(c0_port),
                    "gdb_core1_port": int(c1_port),
                },
            )
            try:
                from host_daemon.cli import get_lock_data, write_lock_file

                lock_data = get_lock_data() or {}
                lock_data.update(
                    {
                        "target": runner.target,
                        "package": runner.package,
                        "bin": runner.bin_name,
                        "profile": "release" if runner.release else "debug",
                        "trace_mode": resolved_tm.value,
                        "speed": runner.speed,
                    }
                )
                write_lock_file(lock_data)
            except Exception:
                pass

            runner.sync_async()
            self._send_json(
                {
                    "success": True,
                    "message": "Target configuration applied and synchronization started.",
                    "target": tgt,
                    "package": pkg,
                    "bin": bin_name,
                    "profile": "release" if rel else "debug",
                    "trace_mode": resolved_tm.value,
                    "external_viewer": get_external_trace_viewer(resolved_tm).value,
                    "enable_debug_server": bool(enable_dbg),
                    "halt_on_reset": bool(halt_rst),
                    "gdb_core0_port": int(c0_port),
                    "gdb_core1_port": int(c1_port),
                }
            )
        elif path == "/api/debug/start":
            try:
                raw_resolve = payload.get("resolve")
                resolve_specs = [str(s) for s in raw_resolve] if isinstance(raw_resolve, list) else None
                dbg = runner.enable_live_debug_server(
                    gdb_core0_port=payload.get("gdb_core0_port"),
                    gdb_core1_port=payload.get("gdb_core1_port"),
                    rtt_bridge_port=payload.get("rtt_bridge_port"),
                    probe_selector=payload.get("probe"),
                    halt_on_reset=payload.get("halt_on_reset"),
                    reset=bool(payload.get("reset", False)),
                    resolve=resolve_specs,
                )
                self._send_json({"success": True, "debug": dbg})
            except Exception as exc:
                self._send_json({"error": str(exc)}, status=400)
        elif path == "/api/debug/stop":
            try:
                dbg = runner.disable_live_debug_server()
                self._send_json({"success": True, "debug": dbg})
            except Exception as exc:
                self._send_json({"error": str(exc)}, status=400)
        elif path == "/api/debug/sync_symbols":
            try:
                if hasattr(runner, "sync_active_symbols"):
                    dbg = runner.sync_active_symbols()
                elif hasattr(runner, "get_debug_status"):
                    dbg = runner.get_debug_status()
                else:
                    dbg = {}
                self._send_json({"success": True, "debug": dbg})
            except Exception as exc:
                self._send_json({"error": str(exc)}, status=400)
        elif path == "/api/sync/cancel":
            res = runner.cancel_sync()
            self._send_json(res)
        elif path == "/api/cli/execute":
            cmd = payload.get("command", "")
            wait_for_output = bool(payload.get("wait_for_output", False))
            if wait_for_output:
                timeout = float(payload.get("timeout", DEFAULT_CLI_OUTPUT_TIMEOUT_SEC))
                output = runner.execute_cli(cmd, wait_for_output=True, timeout=timeout)
            else:
                output = runner.execute_cli(cmd)
            if not getattr(runner, "_streams_live_cli", False):
                _get_test_manager(runner).record_cli_exchange(
                    cmd,
                    output if wait_for_output else "",
                    device_ts=_resolve_runner_device_ts(runner),
                    prefix="shell>",
                )
            self._send_json({"success": True, "output": output})
        elif path == "/api/fs/execute":
            cmd = payload.get("command", "")
            subcmd = payload.get("subcommand", cmd)
            args = payload.get("args", [])
            output = runner.execute_fs(subcmd, args)
            if not getattr(runner, "_streams_live_cli", False):
                fs_cmd_label = f"{subcmd} {' '.join(str(a) for a in args)}".strip()
                _get_test_manager(runner).record_cli_exchange(
                    fs_cmd_label,
                    output,
                    device_ts=_resolve_runner_device_ts(runner),
                    prefix="fs>",
                )
            self._send_json({"success": True, "output": output})
        elif path == "/api/mode":
            mode = payload.get("mode", "CLI").upper()
            if mode in ("CLI", "FS", "TEST", "TRACE"):
                runner.set_mode(mode)
                self._send_json({"success": True, "mode": mode})
            else:
                self._send_json({"error": "Invalid mode. Expected: CLI, FS, TEST, TRACE"}, status=400)
        elif path == "/api/trace/control":
            action = str(payload.get("action") or "").strip().lower()
            try:
                if hasattr(runner, "control_trace_session"):
                    res = runner.control_trace_session(action)
                else:
                    store = _get_trace_store(runner)
                    if action == "clear":
                        store.clear()
                    elif action == "pause":
                        store.set_recording(False)
                    elif action == "resume":
                        store.set_recording(True)
                    else:
                        raise ValueError(f"Unsupported trace control action '{action}'.")
                    res = {
                        "success": True,
                        "action": action,
                        "recording": store.recording,
                        "trace_mode": _get_runner_trace_mode(runner).value,
                    }
                self._send_json(res)
                if hasattr(runner, "log") and not hasattr(runner, "control_trace_session"):
                    runner.log(f"Trace session control '{action}' applied", channel="trace")
            except ValueError as exc:
                self._send_json({"error": str(exc)}, status=400)
        elif path == "/api/trace/export":
            fmt = str(payload.get("format") or "perfetto")
            dest_raw = payload.get("path")
            try:
                if hasattr(runner, "export_trace_session"):
                    res = runner.export_trace_session(fmt, dest_raw)
                else:
                    store = _get_trace_store(runner)
                    cur_tmode = _get_runner_trace_mode(runner)
                    stem = f"{getattr(runner, 'bin_name', None) or 'target'}_{cur_tmode.value.lower()}"
                    res = store.export_to_disk(
                        runner.repo_root,
                        fmt,
                        dest_raw,
                        chip=getattr(runner, "chip", None),
                        default_stem=stem,
                    )
                self._send_json(res)
            except (OSError, ValueError) as exc:
                self._send_json({"error": str(exc)}, status=400)
        elif path == "/api/trace/open_perfetto":
            server_host = str(self.server.server_address[0] or "127.0.0.1")
            server_port = int(self.server.server_address[1])
            try:
                if hasattr(runner, "open_trace_in_perfetto"):
                    res = runner.open_trace_in_perfetto(
                        daemon_host=server_host,
                        daemon_port=server_port,
                    )
                else:
                    store = _get_trace_store(runner)
                    trace_url, perfetto_url = build_perfetto_deep_link(server_host)
                    opened = open_url_in_system_browser(perfetto_url)
                    res = {
                        "success": opened,
                        "url": perfetto_url,
                        "trace_url": trace_url,
                        "event_count": len(store.snapshot_all()),
                    }
                self._send_json(res)
            except (OSError, RuntimeError, ValueError) as exc:
                self._send_json({"error": str(exc)}, status=400)
        elif path == "/api/trace/open_rerun":
            spawn_viewer = bool(payload.get("spawn", True))
            rrd_raw = payload.get("rrd_path")
            try:
                if hasattr(runner, "open_trace_in_rerun"):
                    res = runner.open_trace_in_rerun(spawn=spawn_viewer, save_rrd_path=rrd_raw)
                else:
                    store = _get_trace_store(runner)
                    cur_tmode = _get_runner_trace_mode(runner)
                    rrd_path = Path(str(rrd_raw)) if rrd_raw else None
                    res = open_trace_in_rerun(
                        store.snapshot_all(),
                        app_id=f"{getattr(runner, 'bin_name', None) or 'xerxes'}_{cur_tmode.value.lower()}",
                        spawn=spawn_viewer,
                        save_rrd_path=rrd_path,
                    )
                self._send_json(res)
            except (OSError, RuntimeError, ValueError) as exc:
                self._send_json({"error": str(exc)}, status=400)
        elif path == "/api/tests/scan":
            probe_board = bool(payload.get("probe_board", True))
            res = _get_test_manager(runner).scan_tests(payload, probe_board=probe_board)
            self._send_json({"success": True, **res})
        elif path == "/api/tests/run":
            server_host = str(self.server.server_address[0] or "127.0.0.1")
            if server_host in ("0.0.0.0", "::"):
                server_host = "127.0.0.1"
            server_port = int(self.server.server_address[1])
            try:
                status = _get_test_manager(runner).start_run(
                    payload,
                    daemon_host=server_host,
                    daemon_port=server_port,
                )
            except ValueError as exc:
                self._send_json({"error": str(exc)}, status=400)
                return api_detail
            self._send_json({"success": True, "status": status})
        elif path == "/api/tests/cancel":
            status = _get_test_manager(runner).cancel_run()
            self._send_json({"success": True, "status": status})
        elif path == "/api/reset":
            res = runner.reset_target()
            self._send_json(res)
        elif path == "/api/reboot":
            if hasattr(runner, "reboot_target"):
                res = runner.reboot_target()
            else:
                res = runner.reset_target()
            self._send_json(res)
        elif path == "/api/manual_prompt":
            if hasattr(runner, "update_manual_prompt"):
                state = runner.update_manual_prompt(payload)
            else:
                state = {"active": bool(payload.get("active", False))}
            self._send_json({"success": True, "manual_prompt": state})
        elif path == "/api/manual_prompt/respond":
            try:
                if hasattr(runner, "respond_manual_prompt"):
                    state = runner.respond_manual_prompt(payload)
                elif hasattr(runner, "manual_prompt"):
                    with runner._lock:
                        state = runner.manual_prompt.submit_response(payload)
                else:
                    raise ValueError("Manual prompt responder is not supported on this runner")
            except ValueError as exc:
                self._send_json({"error": str(exc)}, status=400)
                return api_detail
            self._send_json({"success": True, "manual_prompt": state})
        elif path == "/api/target_bug":
            title = str(payload.get("title") or "").strip()
            if not title:
                self._send_json({"error": "Bug title is required"}, status=400)
                return api_detail
            result = create_target_worm_report(
                runner.repo_root,
                payload,
                runner=runner,
                test_manager_getter=_get_test_manager,
            )
            self._send_json(result)
        elif path == "/api/stop":
            self._send_json({"success": True, "message": "Stopping host daemon"})

            def _shutdown() -> None:
                runner.stop()
                self.server.shutdown()

            threading.Thread(target=_shutdown, daemon=True).start()
        else:
            self.send_error(404, "Unknown endpoint")
        return api_detail


class HostDaemonServer(ThreadingHTTPServer):
    """Threading HTTP server running Host Daemon UI and REST APIs."""

    def __init__(self, host: str, port: int, runner: DaemonRunner) -> None:
        """Initialize host daemon HTTP server and configure Jinja template environment."""
        super().__init__((host, port), HostDaemonRequestHandler)
        self.runner = runner
        template_dir = Path(__file__).resolve().parents[2] / "templates"
        self.jinja_env = jinja2.Environment(
            loader=jinja2.FileSystemLoader(str(template_dir)),
            autoescape=jinja2.select_autoescape(["html", "xml"]),
        )
