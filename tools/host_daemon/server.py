"""HTTP server and REST API for Xerxes Host Daemon workstation."""

from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
from pathlib import Path
import sys
import threading
from typing import Any, List, Optional
import uuid as uuid_pkg

import jinja2

dashboard_dir = Path(__file__).resolve().parent.parent / "dashboard"
if str(dashboard_dir) not in sys.path:
    sys.path.insert(0, str(dashboard_dir))

from model.id_generator import generate_docker_pattern_id
from model.worm_report import (
    WormAttachmentModel,
    WormCategory,
    WormReportModel,
    WormSeverity,
    WormStatus,
)
from provider.worm_report.markdown_exporter import MarkdownWormExporter
from provider.worm_report.sqlite_store import SQLiteWormStore

from .runner import DaemonRunner


class HostDaemonRequestHandler(BaseHTTPRequestHandler):
    """HTTP request handler for host daemon web workstation and IPC APIs."""

    server: "HostDaemonServer"

    def log_message(self, format: str, *args) -> None:  # noqa: A002
        """Suppress default HTTP logging to keep console clean."""
        return

    def _send_json(self, data: Any, status: int = 200) -> None:
        payload = json.dumps(data).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(payload)))
        self.send_header("Access-Control-Allow-Origin", "*")
        self.end_headers()
        self.wfile.write(payload)

    def do_GET(self) -> None:  # noqa: N802
        """Handle GET requests for UI and telemetry APIs."""
        path = self.path.split("?")[0]
        runner = self.server.runner

        if path in ("/", "/index.html"):
            try:
                template = self.server.jinja_env.get_template("host_daemon.html.j2")
                html = template.render(runner=runner)
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
        elif path in ("/api/sync", "/api/sync/status"):
            with runner._lock:
                self._send_json(dict(runner.sync_status))
        elif path == "/api/logs":
            with runner._lock:
                self._send_json(list(runner.logs))
        elif path == "/api/cli/output":
            with runner._lock:
                self._send_json(list(runner.cli_outputs))
        elif path == "/api/fs/output":
            with runner._lock:
                self._send_json(list(runner.fs_outputs))
        elif path.startswith("/static/"):
            rel_file = path[len("/static/"):]
            static_dir = Path(__file__).resolve().parent / "static"
            file_path = static_dir / rel_file
            if file_path.exists() and file_path.is_file():
                ext = file_path.suffix.lower()
                content_types = {
                    ".ico": "image/x-icon",
                    ".png": "image/png",
                    ".svg": "image/svg+xml",
                    ".json": "application/json",
                    ".js": "application/javascript",
                    ".css": "text/css",
                    ".icns": "application/octet-stream",
                }
                ctype = content_types.get(ext, "application/octet-stream")
                data = file_path.read_bytes()
                self.send_response(200)
                self.send_header("Content-Type", ctype)
                self.send_header("Content-Length", str(len(data)))
                self.send_header("Cache-Control", "no-cache, no-store, must-revalidate")
                self.send_header("Pragma", "no-cache")
                self.send_header("Expires", "0")
                self.end_headers()
                self.wfile.write(data)
            else:
                self.send_error(404, "Static asset not found")
        else:
            self.send_error(404, "Endpoint not found")

    def do_POST(self) -> None:  # noqa: N802
        """Handle POST requests for command execution and controls."""
        path = self.path.split("?")[0]
        runner = self.server.runner

        content_len = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(content_len).decode("utf-8") if content_len > 0 else "{}"
        try:
            payload = json.loads(body) if body else {}
        except Exception:
            payload = {}

        if path == "/api/sync":
            if payload:
                runner.update_config(
                    target=payload.get("target"),
                    package=payload.get("package"),
                    bin_name=payload.get("bin") or payload.get("bin_name"),
                    release=payload.get("release"),
                    speed=payload.get("speed"),
                    chip=payload.get("chip"),
                )
                try:
                    from .cli import get_lock_data, write_lock_file

                    lock_data = get_lock_data() or {}
                    lock_data.update({
                        "target": runner.target,
                        "package": runner.package,
                        "bin": runner.bin_name,
                        "profile": "release" if runner.release else "debug",
                        "speed": runner.speed,
                    })
                    write_lock_file(lock_data)
                except Exception:
                    pass
            runner.sync_async()
            self._send_json({"success": True, "sync": runner.sync_status})
        elif path == "/api/sync/cancel":
            res = runner.cancel_sync()
            self._send_json(res)
        elif path == "/api/cli/execute":
            cmd = payload.get("command", "")
            output = runner.execute_cli(cmd)
            self._send_json({"success": True, "output": output})
        elif path == "/api/fs/execute":
            cmd = payload.get("command", "")
            subcmd = payload.get("subcommand", cmd)
            args = payload.get("args", [])
            output = runner.execute_fs(subcmd, args)
            self._send_json({"success": True, "output": output})
        elif path == "/api/mode":
            mode = payload.get("mode", "CLI").upper()
            if mode in ("CLI", "FS"):
                runner.set_mode(mode)
                self._send_json({"success": True, "mode": mode})
            else:
                self._send_json({"error": "Invalid mode. Expected: CLI, FS"}, status=400)
        elif path == "/api/reset":
            res = runner.reset_target()
            self._send_json(res)
        elif path == "/api/target_bug":
            title = (payload.get("title") or "").strip()
            if not title:
                self._send_json({"error": "Bug title is required"}, status=400)
                return

            severity_str = (payload.get("severity") or "MEDIUM").upper()
            try:
                severity = WormSeverity(severity_str)
            except Exception:
                severity = WormSeverity.MEDIUM

            category_str = (payload.get("category") or "FIRMWARE").upper()
            try:
                category = WormCategory(category_str)
            except Exception:
                category = WormCategory.FIRMWARE

            component = (payload.get("component") or "target").strip()
            description = (payload.get("description") or "").strip()
            raw_steps = payload.get("reproduction_steps") or []
            if isinstance(raw_steps, str):
                steps = [s.strip() for s in raw_steps.splitlines() if s.strip()]
            else:
                steps = [str(s).strip() for s in raw_steps if str(s).strip()]

            expected = (payload.get("expected_behavior") or "").strip()
            actual = (payload.get("actual_behavior") or "").strip()

            worm_id = generate_docker_pattern_id("WORM")

            # Determine repo directories
            repo_root = runner.repo_root
            attachments_dir = repo_root / "attachments"
            if not attachments_dir.exists() and (repo_root / "feedback" / "attachments").exists():
                attachments_dir = repo_root / "feedback" / "attachments"
            worm_att_dir = attachments_dir / worm_id
            worm_att_dir.mkdir(parents=True, exist_ok=True)

            attachments: List[WormAttachmentModel] = []
            now_str = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")

            # 1. Attach relevant system logs if requested
            logs_content = ""
            if payload.get("attach_system_logs", True):
                with runner._lock:
                    log_entries = list(runner.logs)
                if log_entries:
                    lines = [
                        f"[{e.get('time', '')}] [{e.get('channel', 'daemon')}] [{e.get('level', 'INFO')}] {e.get('message', '')}"
                        for e in log_entries
                    ]
                    logs_content = "\n".join(lines)
                else:
                    logs_content = f"Host daemon session initialized at {now_str}. No logs recorded yet."

                log_filename = f"target_system_logs_{worm_id}.log"
                log_file_path = worm_att_dir / log_filename
                log_file_path.write_text(logs_content, encoding="utf-8")

                rel_att_path = f"attachments/{worm_id}/{log_filename}"
                attachments.append(
                    WormAttachmentModel(
                        id=f"att-{len(attachments)+1}",
                        filename=log_filename,
                        file_type="log",
                        file_path=rel_att_path,
                        size_bytes=len(logs_content.encode("utf-8")),
                        description="Target system and defmt logs from host daemon session",
                        created_at=now_str,
                    )
                )

            # 2. Attach target crash-log if selected
            if payload.get("attach_crash_log"):
                try:
                    crash_out = runner.execute_fs("crash-log")
                except Exception as e:
                    crash_out = f"Failed to retrieve crash log: {e}"
                crash_filename = f"target_crash_log_{worm_id}.log"
                crash_file_path = worm_att_dir / crash_filename
                crash_file_path.write_text(crash_out, encoding="utf-8")
                attachments.append(
                    WormAttachmentModel(
                        id=f"att-{len(attachments)+1}",
                        filename=crash_filename,
                        file_type="log",
                        file_path=f"attachments/{worm_id}/{crash_filename}",
                        size_bytes=len(crash_out.encode("utf-8")),
                        description="Target crash log retrieved via host_fs",
                        created_at=now_str,
                    )
                )

            # 3. Attach target telemetry if selected
            if payload.get("attach_telemetry"):
                try:
                    telem_out = runner.execute_fs("export-telemetry")
                except Exception as e:
                    telem_out = f"Failed to retrieve telemetry: {e}"
                telem_filename = f"target_telemetry_{worm_id}.json"
                telem_file_path = worm_att_dir / telem_filename
                telem_file_path.write_text(telem_out, encoding="utf-8")
                attachments.append(
                    WormAttachmentModel(
                        id=f"att-{len(attachments)+1}",
                        filename=telem_filename,
                        file_type="reference",
                        file_path=f"attachments/{worm_id}/{telem_filename}",
                        size_bytes=len(telem_out.encode("utf-8")),
                        description="Target telemetry export retrieved via host_fs",
                        created_at=now_str,
                    )
                )

            # 4. Attach target filesystem metadata if selected
            if payload.get("attach_metadata"):
                try:
                    meta_out = runner.execute_fs("cat metadata")
                except Exception as e:
                    meta_out = f"Failed to retrieve metadata: {e}"
                meta_filename = f"target_metadata_{worm_id}.txt"
                meta_file_path = worm_att_dir / meta_filename
                meta_file_path.write_text(meta_out, encoding="utf-8")
                attachments.append(
                    WormAttachmentModel(
                        id=f"att-{len(attachments)+1}",
                        filename=meta_filename,
                        file_type="reference",
                        file_path=f"attachments/{worm_id}/{meta_filename}",
                        size_bytes=len(meta_out.encode("utf-8")),
                        description="Filesystem metadata retrieved via host_fs",
                        created_at=now_str,
                    )
                )

            # Build WormReportModel
            log_lines = logs_content.strip().splitlines()
            log_summary = "\n".join(log_lines[-50:]) if len(log_lines) > 50 else logs_content

            worm = WormReportModel(
                id=worm_id,
                uuid=str(uuid_pkg.uuid4()),
                title=title,
                status=WormStatus.OPEN,
                severity=severity,
                category=category,
                component=component,
                description=description,
                reproduction_steps=steps,
                expected_behavior=expected,
                actual_behavior=actual,
                logs=log_summary,
                attachments=attachments,
                created_at=now_str,
                updated_at=now_str,
            )

            # Persist to SQLite
            db_file = repo_root / "target" / "worms.sqlite"
            db_file.parent.mkdir(parents=True, exist_ok=True)
            store = SQLiteWormStore(db_file)
            store.save_worm(worm)

            # Export to feedback/ markdown
            feedback_dir = repo_root / "feedback"
            exporter = MarkdownWormExporter(repo_root=repo_root)
            exported_path = exporter.export_individual_worm(worm, feedback_dir=feedback_dir)

            # Update aggregated feedback/WORMS.md
            try:
                full_db = store.load_all()
                exporter.export_worm_report(full_db, output_path=feedback_dir / "WORMS.md")
            except Exception:
                pass

            runner.log(f"Target bug created: {worm_id} ({title}) with {len(attachments)} attachments", level="INFO", channel="daemon")

            self._send_json({
                "success": True,
                "worm_id": worm_id,
                "file_path": str(exported_path),
                "attachments_count": len(attachments),
            })
        elif path == "/api/stop":
            self._send_json({"success": True, "message": "Stopping host daemon"})

            def _shutdown() -> None:
                runner.stop()
                self.server.shutdown()

            threading.Thread(target=_shutdown, daemon=True).start()
        else:
            self.send_error(404, "Unknown endpoint")


class HostDaemonServer(ThreadingHTTPServer):
    """Threading HTTP server running Host Daemon UI and REST APIs."""

    def __init__(self, host: str, port: int, runner: DaemonRunner) -> None:
        """Initialize host daemon HTTP server and configure Jinja template environment."""
        super().__init__((host, port), HostDaemonRequestHandler)
        self.runner = runner
        template_dir = Path(__file__).resolve().parent / "templates"
        self.jinja_env = jinja2.Environment(
            loader=jinja2.FileSystemLoader(str(template_dir)),
            autoescape=jinja2.select_autoescape(["html", "xml"]),
        )
