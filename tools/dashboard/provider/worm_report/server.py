"""HTTP server and REST API for interactive Worm Report dashboard.

Serves the engineering worm report workstation UI and handles JSON API endpoints
for worm creation, triage, reproduction steps, log and screenshot attachments,
and automated Markdown persistence (feedback/WORMS.md).
"""

import base64
from collections.abc import Collection
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
from pathlib import Path
import shutil
import subprocess
import sys
import threading
from typing import Any, Dict, List, Optional
import urllib.parse
import uuid

import jinja2

from model.worm_report import (
    WormAttachmentModel,
    WormCategory,
    WormDatabaseModel,
    WormReportModel,
    WormReviewTaskModel,
    WormReviewTaskStatus,
    WormSectionModel,
    WormSeverity,
    WormStatus,
)
from provider.worm_report.markdown_exporter import MarkdownWormExporter
from provider.worm_report.sqlite_store import SQLiteWormStore
from provider.sanitizer import elide_personal_info

TEMPLATES_DIR = Path(__file__).resolve().parent.parent.parent / "templates"
MAX_ATTACHMENT_SIZE_BYTES: int = 2 * 1024 * 1024 * 1024  # 2 GiB (GitHub LFS maximum file limit)


class WormReportRequestHandler(BaseHTTPRequestHandler):
    """HTTP request handler dispatching worm report UI and REST API endpoints."""

    server: "WormReportServer"

    def log_message(self, format: str, *args: Any) -> None:  # noqa: A002
        """Suppress default HTTP server logging to preserve clean console output."""
        return

    def do_GET(self) -> None:  # noqa: N802
        """Route GET requests for UI dashboard and data query endpoints."""
        self.server.check_file_watch()
        parsed = urllib.parse.urlparse(self.path)
        path = parsed.path

        if path.startswith("/static/"):
            self._handle_serve_static(path)
            return

        if (
            path.startswith("/attachments/")
            or path.startswith("/target/attachments/")
            or path.startswith("/build/attachments/")
        ):
            self._handle_serve_attachment(path)
            return

        match path:
            case "/" | "/worms" | "/worms/":
                self._handle_serve_ui()
            case "/api/database":
                self._send_json(self.server.database.model_dump(mode="json"))
            case "/api/worms" | "/api/bugs":
                worms_data = [w.model_dump(mode="json") for w in self.server.database.worms]
                self._send_json(worms_data)
            case "/api/metadata":
                self._send_json(
                    {
                        "statuses": [s.value for s in WormStatus],
                        "severities": [s.value for s in WormSeverity],
                        "categories": [c.value for c in WormCategory],
                        "counts_status": self.server.database.count_by_status(),
                        "counts_severity": self.server.database.count_by_severity(),
                        "counts_category": self.server.database.count_by_category(),
                    }
                )
            case "/api/files":
                files = self._list_reference_files()
                self._send_json(files)
            case "/api/next_worm_id" | "/api/next_bug_id":
                self._send_json({"next_id": self.server.database.generate_worm_id()})
            case "/api/version":
                self._send_json(
                    {
                        "version": self.server.db_version,
                        "worms_count": len(self.server.database.worms),
                        "mtime": self.server.feedback_mtime,
                    }
                )
            case _:
                self.send_error(404, "Endpoint not found")

    def do_POST(self) -> None:  # noqa: N802
        """Route POST requests for worm creation, updates, uploads, and export."""
        self.server.check_file_watch()
        parsed = urllib.parse.urlparse(self.path)
        path = parsed.path

        length = int(self.headers.get("Content-Length", 0))
        if length >= MAX_ATTACHMENT_SIZE_BYTES:
            self.send_error(413, "Payload Too Large: Request exceeds maximum allowed size of 2 GB (GitHub LFS limit)")
            return

        try:
            body = self.rfile.read(length).decode("utf-8") if length > 0 else "{}"
            data = json.loads(body)
        except (MemoryError, OverflowError):
            self.send_error(413, "Payload Too Large: Insufficient memory to process request payload")
            return
        except json.JSONDecodeError:
            data = {}

        match path:
            case "/api/worms" | "/api/bugs":
                self._handle_save_worm(data)
            case "/api/worms/status" | "/api/bugs/status":
                self._handle_update_status(data)
            case "/api/worms/accept" | "/api/bugs/accept":
                self._handle_accept_worm(data)
            case "/api/worms/reject" | "/api/bugs/reject":
                self._handle_reject_worm(data)
            case "/api/worms/attach_commit" | "/api/bugs/attach_commit":
                self._handle_attach_commit(data)
            case "/api/worms/add_task" | "/api/bugs/add_task":
                self._handle_add_task(data)
            case "/api/worms/update_task" | "/api/bugs/update_task":
                self._handle_update_task(data)
            case "/api/worms/run_command" | "/api/bugs/run_command":
                self._handle_run_command(data)
            case "/api/worms/delete" | "/api/bugs/delete":
                self._handle_delete_worm(data)
            case "/api/worms/migrate_legacy" | "/api/bugs/migrate_legacy":
                self._handle_migrate_legacy(data)
            case "/api/upload":
                self._handle_file_upload(data)
            case "/api/export":
                out_path = self.server.save_and_sync()
                self._send_json({"status": "exported", "path": str(out_path)})
            case "/api/sync_feedback":
                stats = self.server.sync_with_feedback_dir()
                self._send_json({"status": "ok", "stats": stats})
            case "/api/exit":
                out_path = self.server.save_and_sync()
                self._send_json({"status": "saved_and_exited", "path": str(out_path)})
                threading.Thread(target=self.server.shutdown, daemon=True).start()
            case _:
                self.send_error(404, "Endpoint not found")

    def _handle_serve_ui(self) -> None:
        """Render and serve the worm reporting HTML dashboard via Jinja2."""
        env = jinja2.Environment(
            loader=jinja2.FileSystemLoader(str(TEMPLATES_DIR)),
            autoescape=jinja2.select_autoescape(["html", "xml"]),
            trim_blocks=True,
            lstrip_blocks=True,
        )
        template = env.get_template("worm_report.html.j2")
        db_dump = self.server.database.model_dump(mode="json")
        components = sorted(
            {w.component.strip() for w in self.server.database.worms if w.component and w.component.strip()}
        )
        html_content = template.render(
            database=self.server.database,
            database_json=json.dumps(db_dump),
            statuses=[s.value for s in WormStatus],
            severities=[s.value for s in WormSeverity],
            categories=[c.value for c in WormCategory],
            components=components,
            server_port=self.server.actual_port,
        )
        self._send_html(html_content)

    def _handle_serve_attachment(self, path: str) -> None:
        """Serve uploaded file attachments from attachments directory."""
        clean_path = path.lstrip("/")
        if clean_path.startswith("target/attachments/"):
            rel_name = clean_path[len("target/attachments/") :]
        elif clean_path.startswith("build/attachments/"):
            rel_name = clean_path[len("build/attachments/") :]
        elif clean_path.startswith("attachments/"):
            rel_name = clean_path[len("attachments/") :]
        else:
            rel_name = clean_path
        file_path = self.server.attachments_dir / rel_name
        if not file_path.exists() or not file_path.is_file():
            fallback_target = self.server.repo_root / "target" / "attachments" / rel_name
            fallback_build = self.server.repo_root / "build" / "attachments" / rel_name
            if fallback_target.exists() and fallback_target.is_file():
                file_path = fallback_target
            elif fallback_build.exists() and fallback_build.is_file():
                file_path = fallback_build
            elif (self.server.repo_root / clean_path).is_file():
                file_path = self.server.repo_root / clean_path
            else:
                self.send_error(404, f"Attachment '{rel_name}' not found")
                return

        file_size = file_path.stat().st_size
        suffix = file_path.suffix.lower()
        content_type = {
            ".png": "image/png",
            ".jpg": "image/jpeg",
            ".jpeg": "image/jpeg",
            ".gif": "image/gif",
            ".svg": "image/svg+xml",
            ".webp": "image/webp",
            ".pdf": "application/pdf",
            ".txt": "text/plain",
            ".log": "text/plain",
            ".json": "application/json",
            ".yaml": "text/yaml",
            ".yml": "text/yaml",
            ".md": "text/markdown",
            ".csv": "text/csv",
        }.get(suffix, "application/octet-stream")

        try:
            self.send_response(200)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(file_size))
            self.send_header("Connection", "close")
            self.end_headers()
            with open(file_path, "rb") as f:
                while chunk := f.read(64 * 1024):
                    self.wfile.write(chunk)
        except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError):
            self.close_connection = True
        finally:
            self.close_connection = True

    def _handle_save_worm(self, data: Dict[str, Any]) -> None:
        """Create or update a worm report and persist to storage."""
        worm_id = data.get("id") or self.server.database.generate_worm_id()
        now_str = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")

        existing = self.server.database.get_worm(worm_id)
        created_at = existing.created_at if existing else now_str

        # Parse attachments
        attachments: List[WormAttachmentModel] = []
        for att_data in data.get("attachments", []):
            attachments.append(WormAttachmentModel.model_validate(att_data))

        # Parse reproduction steps
        raw_steps = data.get("reproduction_steps") or data.get("steps") or []
        if isinstance(raw_steps, str):
            steps = [s.strip() for s in raw_steps.splitlines() if s.strip()]
        else:
            steps = [str(s).strip() for s in raw_steps if str(s).strip()]

        status_val = data.get("status", WormStatus.OPEN.value)
        status = WormStatus(status_val) if status_val in [s.value for s in WormStatus] else WormStatus.OPEN
        incoming_notes = data.get("resolution_notes", "").strip()

        resolved_at = existing.resolved_at if existing else None
        if existing and existing.status in (WormStatus.RESOLVED, WormStatus.CLOSED):
            if existing.resolution_notes.strip() and status == WormStatus.OPEN and not incoming_notes:
                status = existing.status
                resolved_at = existing.resolved_at
                incoming_notes = existing.resolution_notes

        if status in (WormStatus.RESOLVED, WormStatus.CLOSED) and not resolved_at:
            resolved_at = now_str
        elif status in (WormStatus.OPEN, WormStatus.IN_PROGRESS):
            resolved_at = None

        cat_raw = data.get("category", WormCategory.FIRMWARE.value)
        try:
            cat_val = WormCategory(cat_raw)
        except ValueError:
            cat_val = WormCategory.GENERAL

        custom_sections = (
            [WormSectionModel.model_validate(c) for c in data.get("custom_sections", [])]
            if "custom_sections" in data
            else (existing.custom_sections if existing else [])
        )

        worm = WormReportModel(
            id=worm_id,
            uuid=data.get("uuid") or (existing.uuid if existing else str(uuid.uuid4())),
            title=data.get("title", "Untitled Worm"),
            status=status,
            severity=WormSeverity(data.get("severity", WormSeverity.MEDIUM.value)),
            category=cat_val,
            component=data.get("component", ""),
            description=elide_personal_info(data.get("description", "")),
            reproduction_steps=[elide_personal_info(s) for s in steps],
            expected_behavior=elide_personal_info(data.get("expected_behavior", "")),
            actual_behavior=elide_personal_info(data.get("actual_behavior", "")),
            logs=elide_personal_info(data.get("logs", "")),
            attachments=attachments,
            custom_sections=custom_sections,
            created_at=created_at,
            updated_at=now_str,
            resolved_at=resolved_at,
            resolution_notes=elide_personal_info(incoming_notes),
        )

        self.server.database.add_or_update(worm)
        self.server.save_and_sync(dirty_bug_ids=[worm.id])
        self._send_json(worm.model_dump(mode="json"))

    def _handle_update_status(self, data: Dict[str, Any]) -> None:
        """Update lifecycle status and resolution notes for a worm."""
        worm_id = data.get("id") or data.get("worm_id") or data.get("bug_id")
        new_status_str = data.get("status")
        notes = data.get("resolution_notes")

        worm = self.server.database.get_worm(worm_id)
        if not worm:
            self._send_json({"error": f"Worm {worm_id} not found"}, status=404)
            return

        now_str = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
        if new_status_str and new_status_str in [s.value for s in WormStatus]:
            worm.status = WormStatus(new_status_str)
            if worm.status in (WormStatus.RESOLVED, WormStatus.CLOSED):
                worm.resolved_at = now_str
            else:
                worm.resolved_at = None

        if notes is not None:
            worm.resolution_notes = elide_personal_info(notes)

        worm.updated_at = now_str
        self.server.save_and_sync(dirty_bug_ids=[worm.id])
        self._send_json(worm.model_dump(mode="json"))

    def _handle_accept_worm(self, data: Dict[str, Any]) -> None:
        """Handle accepting a resolved worm and transitioning directly to CLOSED."""
        worm_id = data.get("id") or data.get("worm_id")
        notes = data.get("notes", "").strip()
        worm = self.server.database.get_worm(worm_id) if worm_id else None
        if not worm:
            self._send_json({"error": f"Worm {worm_id} not found"}, status=404)
            return

        now_str = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
        worm.status = WormStatus.CLOSED
        worm.accepted_at = now_str
        if notes:
            if worm.resolution_notes:
                worm.resolution_notes += f"\n\n**Acceptance Note**: {notes}"
            else:
                worm.resolution_notes = f"**Acceptance Note**: {notes}"
        worm.updated_at = now_str
        self.server.save_and_sync(dirty_bug_ids=[worm.id])
        self._send_json({"status": "ok", "worm": worm.model_dump(mode="json")})

    def _handle_reject_worm(self, data: Dict[str, Any]) -> None:
        """Handle rejecting a resolved worm and transitioning directly back to OPEN."""
        worm_id = data.get("id") or data.get("worm_id")
        notes = data.get("notes", "").strip()
        worm = self.server.database.get_worm(worm_id) if worm_id else None
        if not worm:
            self._send_json({"error": f"Worm {worm_id} not found"}, status=404)
            return

        now_str = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
        worm.status = WormStatus.OPEN
        worm.resolved_at = None
        worm.rejected_at = now_str
        worm.rejection_notes = elide_personal_info(notes)
        worm.updated_at = now_str
        self.server.save_and_sync(dirty_bug_ids=[worm.id])
        self._send_json({"status": "ok", "worm": worm.model_dump(mode="json")})

    def _handle_migrate_legacy(self, data: Dict[str, Any]) -> None:
        """Handle migrating legacy resolved worms to CLOSED status."""
        before_ts = data.get("before", "2026-11-05 14:00:00 UTC")
        migrate_all = bool(data.get("all", False))
        exclude_raw = data.get("exclude", [])
        exclude_ids = set(exclude_raw if isinstance(exclude_raw, list) else str(exclude_raw).split(","))
        dry_run = bool(data.get("dry_run", False))

        candidates = []
        for w in self.server.database.worms:
            if w.status == WormStatus.RESOLVED and not w.accepted_at:
                if w.id.upper() in {x.upper().strip() for x in exclude_ids if x.strip()}:
                    continue
                if not migrate_all and before_ts:
                    check_ts = w.resolved_at or w.updated_at or w.created_at
                    if check_ts and check_ts > before_ts:
                        continue
                candidates.append(w)

        if dry_run:
            self._send_json(
                {
                    "status": "ok",
                    "dry_run": True,
                    "count": len(candidates),
                    "migrated_ids": [w.id for w in candidates],
                }
            )
            return

        now_str = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
        migrated_ids = []
        for w in candidates:
            w.status = WormStatus.CLOSED
            w.accepted_at = w.resolved_at or w.updated_at or w.created_at or now_str
            w.updated_at = now_str
            migrated_ids.append(w.id)

        if migrated_ids:
            self.server.save_and_sync(dirty_bug_ids=migrated_ids)

        self._send_json(
            {
                "status": "ok",
                "dry_run": False,
                "count": len(migrated_ids),
                "migrated_ids": migrated_ids,
            }
        )

    def _handle_attach_commit(self, data: Dict[str, Any]) -> None:
        """Attach a commit hash to a worm report."""
        worm_id = data.get("id") or data.get("worm_id")
        commit_id = data.get("commit_id", "").strip()
        worm = self.server.database.get_worm(worm_id) if worm_id else None
        if not worm:
            self._send_json({"error": f"Worm {worm_id} not found"}, status=404)
            return
        if not commit_id:
            self._send_json({"error": "Missing commit_id"}, status=400)
            return

        if commit_id not in worm.commit_ids:
            worm.commit_ids.append(commit_id)

        worm.updated_at = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
        self.server.save_and_sync(dirty_bug_ids=[worm.id])
        self._send_json({"status": "ok", "worm": worm.model_dump(mode="json")})

    def _handle_add_task(self, data: Dict[str, Any]) -> None:
        """Add a review task to a worm report."""
        worm_id = data.get("id") or data.get("worm_id")
        title = data.get("title", "").strip()
        cmd = data.get("command", "").strip()
        worm = self.server.database.get_worm(worm_id) if worm_id else None
        if not worm:
            self._send_json({"error": f"Worm {worm_id} not found"}, status=404)
            return
        if not title:
            self._send_json({"error": "Missing task title"}, status=400)
            return

        task = WormReviewTaskModel(title=title, command=cmd)
        worm.review_tasks.append(task)
        worm.updated_at = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
        self.server.save_and_sync(dirty_bug_ids=[worm.id])
        self._send_json({"status": "ok", "task": task.model_dump(mode="json"), "worm": worm.model_dump(mode="json")})

    def _handle_update_task(self, data: Dict[str, Any]) -> None:
        """Update review task status or notes."""
        worm_id = data.get("id") or data.get("worm_id")
        task_id = data.get("task_id")
        new_status = data.get("status")
        notes = data.get("notes")
        worm = self.server.database.get_worm(worm_id) if worm_id else None
        if not worm:
            self._send_json({"error": f"Worm {worm_id} not found"}, status=404)
            return

        target_task = None
        for t in worm.review_tasks:
            if t.id == task_id:
                target_task = t
                break

        if not target_task:
            self._send_json({"error": f"Task {task_id} not found"}, status=404)
            return

        if new_status:
            try:
                target_task.status = WormReviewTaskStatus(str(new_status).upper())
            except ValueError:
                target_task.status = WormReviewTaskStatus.PENDING
        if notes is not None:
            target_task.notes = elide_personal_info(str(notes))

        worm.updated_at = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
        self.server.save_and_sync(dirty_bug_ids=[worm.id])
        self._send_json(
            {"status": "ok", "task": target_task.model_dump(mode="json"), "worm": worm.model_dump(mode="json")}
        )

    def _handle_run_command(self, data: Dict[str, Any]) -> None:
        """Execute a review demo command, ensuring firmware contains fix and auto-syncing if needed."""
        worm_id = data.get("id") or data.get("worm_id")
        cmd = data.get("command", "").strip()
        auto_sync = data.get("auto_sync", True)
        if not cmd:
            self._send_json({"error": "Empty command"}, status=400)
            return

        worm = self.server.database.get_worm(worm_id) if worm_id else None

        # 1. Pre-flight check: if attached commits exist, verify workspace HEAD contains the fix commit
        head_commit = None
        try:
            head_proc = subprocess.run(
                ["git", "rev-parse", "HEAD"],
                cwd=str(self.server.repo_root),
                capture_output=True,
                text=True,
                check=False,
            )
            if head_proc.returncode == 0:
                head_commit = head_proc.stdout.strip()
        except Exception:
            pass

        if worm and worm.commit_ids and head_commit:
            is_present = False
            for cid in worm.commit_ids:
                if cid.startswith(head_commit) or head_commit.startswith(cid):
                    is_present = True
                    break
                anc_proc = subprocess.run(
                    ["git", "merge-base", "--is-ancestor", cid, head_commit],
                    cwd=str(self.server.repo_root),
                    check=False,
                )
                if anc_proc.returncode == 0:
                    is_present = True
                    break
            if not is_present:
                self._send_json(
                    {
                        "status": "error",
                        "error": f"Pre-flight check failed: Workspace HEAD ({head_commit[:8]}) does not contain attached fix commit(s): {', '.join(worm.commit_ids)}",
                    },
                    status=409,
                )
                return

        # 2. Host daemon connection & flash guarantee
        host_daemon_url = "http://127.0.0.1:8899"
        daemon_online = False
        flashed_commit = None
        try:
            import urllib.request

            req = urllib.request.Request(f"{host_daemon_url}/api/sync/status", headers={"User-Agent": "worm-dashboard"})
            with urllib.request.urlopen(req, timeout=1.0) as resp:
                if resp.status == 200:
                    daemon_online = True
                    sync_data = json.loads(resp.read().decode("utf-8"))
                    flashed_commit = sync_data.get("flashed_commit")
        except Exception:
            daemon_online = False

        if (
            daemon_online
            and auto_sync
            and (not flashed_commit or (head_commit and not flashed_commit.startswith(head_commit[:8])))
        ):
            try:
                sync_req = urllib.request.Request(
                    f"{host_daemon_url}/api/sync",
                    data=b"{}",
                    headers={"Content-Type": "application/json"},
                )
                with urllib.request.urlopen(sync_req, timeout=60.0) as resp:
                    sync_res = json.loads(resp.read().decode("utf-8"))
                    if not sync_res.get("success", False) and sync_res.get("step") == "failed":
                        self._send_json(
                            {
                                "status": "error",
                                "error": f"Auto-flash failed: {sync_res.get('error', 'Flash failed')}",
                                "console": sync_res.get("console", []),
                            },
                            status=500,
                        )
                        return
            except Exception as e:
                self._send_json(
                    {"status": "error", "error": f"Failed to trigger auto-sync on host daemon: {e}"},
                    status=500,
                )
                return

        # 3. Forward command execution
        if daemon_online:
            try:
                exec_req = urllib.request.Request(
                    f"{host_daemon_url}/api/cli/execute",
                    data=json.dumps({"command": cmd}).encode("utf-8"),
                    headers={"Content-Type": "application/json"},
                )
                with urllib.request.urlopen(exec_req, timeout=30.0) as resp:
                    exec_res = json.loads(resp.read().decode("utf-8"))
                    self._send_json(
                        {
                            "status": "ok" if exec_res.get("success", True) else "error",
                            "forwarded_to_host_daemon": True,
                            "exit_code": exec_res.get("exit_code", 0),
                            "stdout": exec_res.get("output", "") or exec_res.get("stdout", ""),
                            "stderr": exec_res.get("error", "") or exec_res.get("stderr", ""),
                        }
                    )
                    return
            except Exception:
                pass

        proc = subprocess.run(
            cmd,
            shell=True,
            cwd=str(self.server.repo_root),
            capture_output=True,
            text=True,
        )
        self._send_json(
            {
                "status": "ok" if proc.returncode == 0 else "error",
                "forwarded_to_host_daemon": False,
                "exit_code": proc.returncode,
                "stdout": proc.stdout,
                "stderr": proc.stderr,
            }
        )

    def _handle_delete_worm(self, data: Dict[str, Any]) -> None:
        """Delete a worm report by ID."""
        worm_id = data.get("id") or data.get("worm_id") or data.get("bug_id")
        if worm_id:
            self.server.delete_worm(worm_id)
            self._send_json({"status": "deleted", "id": worm_id})
        else:
            self._send_json({"error": "Missing worm ID"}, status=400)

    def _handle_file_upload(self, data: Dict[str, Any]) -> None:
        """Save base64 encoded file upload or log string to attachments directory."""
        filename = data.get("filename", f"attachment_{uuid.uuid4().hex[:8]}.txt")
        file_type = data.get("file_type", "reference")
        desc = data.get("description", "")
        b64_content = data.get("content_base64", "")
        text_content = data.get("content_text", "")

        raw_worm_id = str(
            data.get("worm_id") or data.get("wormId") or data.get("bug_id") or data.get("bugId") or ""
        ).strip()
        if not raw_worm_id and (
            str(data.get("id", "")).startswith("WORM-") or str(data.get("id", "")).startswith("BUG-")
        ):
            raw_worm_id = str(data.get("id")).strip()
        worm_id = Path(raw_worm_id).name if raw_worm_id else ""
        filename = Path(filename).name

        dest_dir = (self.server.attachments_dir / worm_id) if worm_id else self.server.attachments_dir
        dest_dir.mkdir(parents=True, exist_ok=True)
        dest_path = dest_dir / filename

        if dest_path.exists():
            stem = dest_path.stem
            suffix = dest_path.suffix
            counter = 1
            while (dest_dir / f"{stem}_{counter}{suffix}").exists():
                counter += 1
            filename = f"{stem}_{counter}{suffix}"
            dest_path = dest_dir / filename

        if b64_content and (len(b64_content) * 3) // 4 >= MAX_ATTACHMENT_SIZE_BYTES:
            self._send_json(
                {"error": "Payload Too Large: Attachment exceeds 2 GB limit (GitHub LFS limit)"},
                status=413,
            )
            return
        if text_content and len(text_content.encode("utf-8")) >= MAX_ATTACHMENT_SIZE_BYTES:
            self._send_json(
                {"error": "Payload Too Large: Attachment exceeds 2 GB limit (GitHub LFS limit)"},
                status=413,
            )
            return

        try:
            if b64_content:
                file_bytes = base64.b64decode(b64_content)
                if len(file_bytes) >= MAX_ATTACHMENT_SIZE_BYTES:
                    self._send_json(
                        {"error": "Payload Too Large: Attachment exceeds 2 GB limit (GitHub LFS limit)"},
                        status=413,
                    )
                    return
                dest_path.write_bytes(file_bytes)
            elif text_content:
                file_bytes = text_content.encode("utf-8")
                if len(file_bytes) >= MAX_ATTACHMENT_SIZE_BYTES:
                    self._send_json(
                        {"error": "Payload Too Large: Attachment exceeds 2 GB limit (GitHub LFS limit)"},
                        status=413,
                    )
                    return
                dest_path.write_bytes(file_bytes)
            else:
                file_bytes = b""
                dest_path.write_bytes(file_bytes)
        except (MemoryError, OverflowError):
            self._send_json(
                {"error": "Payload Too Large: Insufficient memory to process attachment"},
                status=413,
            )
            return

        rel_path = (
            dest_path.relative_to(self.server.repo_root)
            if dest_path.is_relative_to(self.server.repo_root)
            else dest_path
        )

        now_str = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
        att = WormAttachmentModel(
            id=uuid.uuid4().hex[:8],
            filename=filename,
            file_type=file_type,
            file_path=str(rel_path),
            size_bytes=len(file_bytes),
            description=desc,
            created_at=now_str,
        )
        self._send_json(att.model_dump(mode="json"))

    def _list_reference_files(self) -> List[Dict[str, str]]:
        """List relevant preview screenshots, logs, and artifacts in workspace."""
        results: List[Dict[str, str]] = []
        for b_dir in [self.server.repo_root / "target", self.server.repo_root / "build"]:
            if b_dir.exists():
                for p in b_dir.glob("*.png"):
                    results.append(
                        {"name": p.name, "path": str(p.relative_to(self.server.repo_root)), "type": "screenshot"}
                    )
        rec_dir = self.server.repo_root / "recordings" / "previews"
        if rec_dir.exists():
            for p in rec_dir.glob("*.png"):
                results.append(
                    {"name": p.name, "path": str(p.relative_to(self.server.repo_root)), "type": "screenshot"}
                )
        return results

    def _handle_serve_static(self, path: str) -> None:
        """Serve static assets such as favicon and vendor bundles."""
        static_dir = Path(__file__).resolve().parent.parent / "code_review" / "static"
        filename = path.removeprefix("/static/").strip("/")
        file_target = (static_dir / filename).resolve()
        if not str(file_target).startswith(str(static_dir)) or not file_target.is_file():
            self.send_error(404, "Static asset not found")
            return
        if file_target.suffix == ".svg":
            content_type = "image/svg+xml"
        elif file_target.suffix == ".js":
            content_type = "application/javascript"
        elif file_target.suffix == ".css":
            content_type = "text/css"
        else:
            content_type = "text/plain"
        data = file_target.read_bytes()
        try:
            self.send_response(200)
            self.send_header("Content-Type", f"{content_type}; charset=utf-8")
            self.send_header("Content-Length", str(len(data)))
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(data)
        except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError):
            self.close_connection = True
        finally:
            self.close_connection = True

    def _send_html(self, html: str) -> None:
        """Send UTF-8 encoded HTML HTTP response payload."""
        encoded = html.encode("utf-8")
        try:
            self.send_response(200)
            self.send_header("Content-Type", "text/html; charset=utf-8")
            self.send_header("Content-Length", str(len(encoded)))
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(encoded)
        except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError):
            self.close_connection = True
        finally:
            self.close_connection = True

    def _send_json(self, data: Any, status: int = 200) -> None:
        """Send JSON HTTP response payload."""
        encoded = json.dumps(data, indent=2).encode("utf-8")
        try:
            self.send_response(status)
            self.send_header("Content-Type", "application/json; charset=utf-8")
            self.send_header("Content-Length", str(len(encoded)))
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(encoded)
        except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError):
            self.close_connection = True
        finally:
            self.close_connection = True


class WormReportServer(ThreadingHTTPServer):
    """Local Threading HTTP web server hosting the interactive worm report dashboard."""

    allow_reuse_address = True
    daemon_threads = True

    def handle_error(self, request: Any, client_address: Any) -> None:
        """Handle client connection errors gracefully without printing tracebacks on client disconnects."""
        exc_type, _, _ = sys.exc_info()
        if exc_type is not None and issubclass(
            exc_type, (BrokenPipeError, ConnectionResetError, ConnectionAbortedError)
        ):
            return
        super().handle_error(request, client_address)

    def server_bind(self) -> None:
        """Override server_bind to avoid slow reverse DNS lookups via getfqdn."""
        from socketserver import TCPServer

        TCPServer.server_bind(self)
        self.server_name = self.server_address[0]
        self.server_port = self.server_address[1]

    def __init__(
        self,
        host: str = "127.0.0.1",
        port: int = 8876,
        repo_root: Optional[Path] = None,
        markdown_output: Optional[Path] = None,
        feedback_dir: Optional[Path] = None,
        state_file: Optional[Path] = None,
        sqlite_file: Optional[Path] = None,
        attachments_dir: Optional[Path] = None,
        fresh: bool = False,
        bind_and_activate: bool = True,
    ) -> None:
        """Initialize server with persistent storage paths."""
        self.host = host
        self.port = port
        self.repo_root = repo_root or Path.cwd()
        self.markdown_output = markdown_output or (self.repo_root / "target" / "WORMS.md")
        if feedback_dir is not None:
            self.feedback_dir = feedback_dir
        elif markdown_output is not None and markdown_output.parent.name not in ("build", "target"):
            self.feedback_dir = markdown_output.parent
        else:
            self.feedback_dir = self.repo_root / "feedback"
        self.state_file = state_file or (self.repo_root / "target" / "worms_state.json")

        if sqlite_file:
            self.sqlite_file = sqlite_file
        else:
            target_sqlite = self.repo_root / "target" / "worms.sqlite"
            build_sqlite = self.repo_root / "build" / "worms.sqlite"
            old_bugs = self.repo_root / "target" / "bugs.sqlite"
            if not target_sqlite.exists():
                if build_sqlite.exists():
                    try:
                        target_sqlite.parent.mkdir(parents=True, exist_ok=True)
                        shutil.copy2(str(build_sqlite), str(target_sqlite))
                    except Exception:
                        pass
                elif old_bugs.exists():
                    try:
                        target_sqlite.parent.mkdir(parents=True, exist_ok=True)
                        shutil.copy2(str(old_bugs), str(target_sqlite))
                    except Exception:
                        pass
            self.sqlite_file = target_sqlite

        self.attachments_dir = attachments_dir or (self.repo_root / "attachments")
        self.fresh = fresh
        self.bind_and_activate = bind_and_activate

        self._lock = threading.RLock()
        self._is_internal_saving = False

        self.sqlite_store = SQLiteWormStore(self.sqlite_file)
        self.exporter = MarkdownWormExporter(repo_root=self.repo_root)
        self.markdown_mtime = self.markdown_output.stat().st_mtime if self.markdown_output.exists() else 0.0
        self.feedback_mtime = self._get_feedback_dir_mtime()
        self.db_version: int = 1
        self.database = self._initialize_database()
        self._watcher_stop = threading.Event()
        if bind_and_activate:
            self._start_file_watcher()
            super().__init__((host, port), WormReportRequestHandler)
            self.actual_port = self.server_address[1]
        else:
            self.actual_port = port

    def get_request(self) -> Any:
        """Accept incoming connection and set a socket timeout to prevent lingering sockets."""
        sock, addr = super().get_request()
        sock.settimeout(10.0)
        return sock, addr

    def _get_feedback_dir_mtime(self) -> float:
        """Compute maximum mtime across feedback_dir and all WORM_*.md / BUG_*.md files within it."""
        if not self.feedback_dir.exists():
            return 0.0
        try:
            max_mtime = self.feedback_dir.stat().st_mtime
            for p in sorted(self.feedback_dir.glob("WORM_*.md")) + sorted(self.feedback_dir.glob("BUG_*.md")):
                try:
                    m = p.stat().st_mtime
                    if m > max_mtime:
                        max_mtime = m
                except OSError:
                    pass
            return max_mtime
        except OSError:
            return 0.0

    def check_file_watch(self) -> bool:
        """Check if feedback_dir or markdown_output was modified externally and reload state."""
        with self._lock:
            if self._is_internal_saving:
                return False
            changed = False
            if self.markdown_output.exists():
                try:
                    curr_mtime = self.markdown_output.stat().st_mtime
                    if curr_mtime > self.markdown_mtime + 0.001:
                        changed = True
                except OSError:
                    pass

            curr_fb_mtime = self._get_feedback_dir_mtime()
            if curr_fb_mtime > self.feedback_mtime + 0.001:
                changed = True

            if changed:
                self.sync_with_feedback_dir()
                return True
            return False

    def _start_file_watcher(self) -> None:
        """Start background polling thread to watch feedback_dir and WORMS.md for external changes."""

        def watch_loop() -> None:
            import time

            while not self._watcher_stop.is_set():
                time.sleep(1.0)
                try:
                    self.check_file_watch()
                except Exception:
                    pass

        t = threading.Thread(target=watch_loop, daemon=True)
        t.start()

    def server_close(self) -> None:
        """Stop background file watcher and close server."""
        self._watcher_stop.set()
        if hasattr(self, "socket"):
            super().server_close()

    def _ensure_unique_worm_ids(self, db: WormDatabaseModel) -> None:
        """Ensure all worm IDs and UUIDs in database are unique, disambiguating any duplicates."""
        seen_ids: set[str] = set()
        seen_uuids: set[str] = set()
        for worm in db.worms:
            if worm.id in seen_ids:
                worm.id = db.generate_worm_id()
                worm.uuid = str(uuid.uuid4())
            if worm.uuid in seen_uuids:
                worm.uuid = str(uuid.uuid4())
            seen_ids.add(worm.id)
            seen_uuids.add(worm.uuid)

    def _initialize_database(self) -> WormDatabaseModel:
        """Load existing database state from SQLite/JSON and perform R+M+W sync with feedback/."""
        db = None
        if not self.fresh:
            if self.sqlite_file.exists():
                db = self.sqlite_store.load_database()
            elif self.state_file.exists():
                db = self.exporter.load_state_json(self.state_file)

        if db is None:
            db = WormDatabaseModel(
                title="Firmware Worm Tracker",
                summary="Firmware engineering defects, driver anomalies, and reproduction tracking.",
            )

        if not self.fresh and not db.worms and (self.repo_root / ".git").exists():
            try:
                from provider.worm_report.detached_branch import DetachedBranchWormStore

                detached_db = DetachedBranchWormStore(self.repo_root).load_database()
                if detached_db and detached_db.worms:
                    db = detached_db
            except Exception:
                pass

        if not self.fresh:
            fallback_md = self.repo_root / "build" / "WORMS.md"
            md_to_read = (
                self.markdown_output
                if self.markdown_output.exists()
                else (fallback_md if fallback_md.exists() else None)
            )
            if md_to_read and md_to_read.exists():
                md_db = self.exporter.parse_markdown(md_to_read)
                if md_db and md_db.worms:
                    db = self.exporter.merge_databases(db, md_db)
                self.markdown_mtime = md_to_read.stat().st_mtime

        if not self.fresh and self.feedback_dir.exists():
            self.exporter.scan_and_sync_feedback_dir(self.feedback_dir, db, self.sqlite_store)

        self._ensure_unique_worm_ids(db)
        if not self.fresh:
            self.sqlite_store.save_database(db)
        return db

    def sync_with_markdown(self) -> Path:
        """Perform Read-Modify-Write (R+M+W) sync with feedback/ and WORMS.md and persist to stores."""
        with self._lock:
            if self.feedback_dir.exists():
                self.exporter.scan_and_sync_feedback_dir(self.feedback_dir, self.database, self.sqlite_store)
            if self.markdown_output.exists():
                md_db = self.exporter.parse_markdown(self.markdown_output)
                if md_db and md_db.worms:
                    self.database = self.exporter.merge_databases(self.database, md_db)
                    self._ensure_unique_worm_ids(self.database)
            self._is_internal_saving = True
            try:
                return self.save_and_sync()
            finally:
                self._is_internal_saving = False

    def sync_with_feedback_dir(self) -> Dict[str, Any]:
        """Scan feedback/ directory, detect file renames, merge into SQLite, and update markdown."""
        with self._lock:
            if self.markdown_output.exists():
                md_db = self.exporter.parse_markdown(self.markdown_output)
                if md_db and md_db.worms:
                    self.database = self.exporter.merge_databases(self.database, md_db)
            stats = self.exporter.scan_and_sync_feedback_dir(self.feedback_dir, self.database, self.sqlite_store)
            self.save_and_sync()
            return stats

    def save_and_sync(self, dirty_bug_ids: Optional[Collection[str]] = None) -> Path:
        """Persist worm database to SQLite, JSON, WORMS.md, and individual WORM_<id>.md files."""
        with self._lock:
            self._is_internal_saving = True
            try:
                self.database.updated_at = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
                if dirty_bug_ids is not None:
                    dirty_set = set(dirty_bug_ids)
                    for worm in self.database.worms:
                        if worm.id in dirty_set:
                            self.sqlite_store.save_worm(worm)
                    with self.sqlite_store._get_connection() as conn:
                        conn.execute(
                            "INSERT INTO metadata (key, value) VALUES ('updated_at', ?) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                            (self.database.updated_at,),
                        )
                else:
                    self.sqlite_store.save_database(self.database)

                self.exporter.export_state_json(self.database, self.state_file)
                out = self.exporter.export_markdown(
                    self.database,
                    self.markdown_output,
                    feedback_dir=self.feedback_dir,
                    store=self.sqlite_store,
                    dirty_bug_ids=dirty_bug_ids,
                )
                if self.markdown_output.exists():
                    self.markdown_mtime = self.markdown_output.stat().st_mtime
                self.feedback_mtime = self._get_feedback_dir_mtime()
                self.db_version += 1
                if (self.repo_root / ".git").exists():
                    try:
                        from provider.worm_report.detached_branch import DetachedBranchWormStore

                        DetachedBranchWormStore(self.repo_root).commit_database(
                            self.database,
                            commit_message=f"Update worm database (v{self.db_version})",
                            sync_local_disk=False,
                            dirty_bug_ids=dirty_bug_ids,
                        )
                    except Exception:
                        pass
                return out
            finally:
                self._is_internal_saving = False

    def save_worm(self, worm: WormReportModel) -> None:
        """Save a single worm and sync to stores."""
        with self._lock:
            self.database.add_or_update(worm)
            self.save_and_sync(dirty_bug_ids=[worm.id])

    def delete_worm(self, worm_id: str) -> bool:
        """Delete a worm from all stores."""
        with self._lock:
            self._is_internal_saving = True
            try:
                deleted = self.sqlite_store.delete_worm(worm_id)
                self.database.worms = [w for w in self.database.worms if w.id != worm_id]
                self.exporter.export_state_json(self.database, self.state_file)
                self.exporter.export_markdown(self.database, self.markdown_output, feedback_dir=self.feedback_dir)
                md_path = self.feedback_dir / f"{worm_id.replace('-', '_')}.md"
                if md_path.exists():
                    try:
                        md_path.unlink()
                    except OSError:
                        pass
                return deleted
            finally:
                self._is_internal_saving = False

    def get_url(self) -> str:
        """Return reachable HTTP URL for browser."""
        port = getattr(self, "actual_port", None) or self.port
        return f"http://{self.host}:{port}"
