"""HTTP server and REST API for unified System Shock 2 - Xerxes VCS Dashboard, Code Review, and Worm Report.

Serves the primary engineering diff workstation UI, DAG ancestor tree visualization,
working tree stage/unstage/discard/commit operations, commit split/combine,
merge conflict resolution, interactive line-by-line Code Review, and Worm Tracker.
"""

import base64
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import mimetypes
from pathlib import Path
import re
import socket
import sys
from typing import Any, Dict, List, Optional
import urllib.parse
import uuid

import jinja2

from model.code_review import CommentModel, ReviewSessionModel, ReviewSeverity, ReviewStatus
from model.vcs import (
    BranchInfoModel,
    CommitNodeModel,
    CommitWormTagModel,
    DiffViewSessionModel,
    FileDiffModel,
    MergeConflictFileModel,
    WorkingTreeFileModel,
)
from model.worm_report import (
    WormAttachmentModel,
    WormCategory,
    WormDatabaseModel,
    WormReportModel,
    WormSeverity,
    WormStatus,
)
from provider.code_review.server import ReviewServer
from provider.vcs.git_engine import GitEngine, extract_line_snippet, get_git_root
from provider.worm_report.server import WormReportServer


class DashboardRequestHandler(BaseHTTPRequestHandler):
    """HTTP request handler dispatching unified VCS dashboard, code review, and worm report APIs."""

    server: "DashboardServer"

    def log_message(self, format: str, *args) -> None:  # noqa: A002
        """Suppress default HTTP server logging to preserve clean console output."""
        return

    def handle_one_request(self) -> None:
        """Handle a single HTTP request, catching client disconnects gracefully."""
        try:
            super().handle_one_request()
        except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError):
            self.close_connection = True

    def handle(self) -> None:
        """Handle incoming requests on this connection until closed."""
        try:
            super().handle()
        except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError):
            self.close_connection = True

    def do_GET(self) -> None:  # noqa: N802
        """Route GET requests for UI dashboards and data query endpoints."""
        if hasattr(self.server, "review_server") and self.server.review_server:
            try:
                self.server.review_server.check_file_watch()
            except Exception:
                pass
        if hasattr(self.server, "worm_server") and self.server.worm_server:
            try:
                self.server.worm_server.check_file_watch()
            except Exception:
                pass

        parsed = urllib.parse.urlparse(self.path)
        path = parsed.path
        query = urllib.parse.parse_qs(parsed.query)

        if path.startswith("/static/"):
            self._handle_serve_static(path)
            return

        if path in ("/favicon.ico", "/favicon.svg"):
            self._handle_serve_static("/static/favicon.svg")
            return

        if (
            path.startswith("/attachments/")
            or path.startswith("/target/attachments/")
            or path.startswith("/build/attachments/")
        ):
            self._handle_serve_attachment(path)
            return

        match path:
            case "/" | "/index.html":
                self._handle_serve_diff_ui()
            case "/review" | "/review/":
                self._handle_serve_review_ui(query)
            case "/worms" | "/worms/":
                self._handle_serve_worm_ui()
            case "/api/session":
                referer = self.headers.get("Referer", "")
                rev_param = query.get("revisions", [""])[0] or query.get("commit", [""])[0]
                if not rev_param and "/review" in referer and "?" in referer:
                    ref_query = urllib.parse.parse_qs(urllib.parse.urlparse(referer).query)
                    rev_param = ref_query.get("revisions", [""])[0] or ref_query.get("commit", [""])[0]
                revs = [r.strip() for r in rev_param.split(",") if r.strip()] if rev_param else None

                if query.get("type", [""])[0] == "review" or "/review" in referer or revs:
                    session = self.server.get_review_session(revs)
                    self._send_json(session.model_dump(mode="json"))
                else:
                    session = self.server.build_session()
                    self._send_json(session.model_dump(mode="json"))
            case "/api/review/session":
                referer = self.headers.get("Referer", "")
                rev_param = query.get("revisions", [""])[0] or query.get("commit", [""])[0]
                if not rev_param and "/review" in referer and "?" in referer:
                    ref_query = urllib.parse.parse_qs(urllib.parse.urlparse(referer).query)
                    rev_param = ref_query.get("revisions", [""])[0] or ref_query.get("commit", [""])[0]
                revs = [r.strip() for r in rev_param.split(",") if r.strip()] if rev_param else None
                session = self.server.get_review_session(revs)
                self._send_json(session.model_dump(mode="json"))
            case "/api/commits":
                referer = self.headers.get("Referer", "")
                is_review = "/review" in referer or query.get("type", [""])[0] == "review"
                rev_param = query.get("revisions", [""])[0] or query.get("commit", [""])[0]
                if not rev_param and is_review and "?" in referer:
                    ref_query = urllib.parse.parse_qs(urllib.parse.urlparse(referer).query)
                    rev_param = ref_query.get("revisions", [""])[0] or ref_query.get("commit", [""])[0]

                if is_review or rev_param:
                    revs = [r.strip() for r in rev_param.split(",") if r.strip()] if rev_param else None
                    if not revs and self.server.review_server.session.revisions:
                        revs = self.server.review_server.session.revisions
                    commits = self.server.git_engine.get_commits(rev_args=revs)
                    self._send_json([c.model_dump(mode="json") for c in commits])
                else:
                    limit_str = query.get("limit", ["40"])[0]
                    limit = int(limit_str) if limit_str.isdigit() else 40
                    commits = self.server.git_engine.get_smartlog_dag(limit=limit)
                    self._send_json([c.model_dump(mode="json") for c in commits])
            case "/api/branches":
                branches = self.server.git_engine.get_branches()
                self._send_json([b.model_dump(mode="json") for b in branches])
            case "/api/working":
                files = self.server.git_engine.get_working_tree_files()
                self._send_json([f.model_dump(mode="json") for f in files])
            case "/api/head_commit_message":
                self._send_json({"message": self.server.git_engine.get_head_commit_message()})
            case "/api/conflicts":
                conflicts = self.server.git_engine.get_merge_conflicts()
                self._send_json([c.model_dump(mode="json") for c in conflicts])
            case "/api/files":
                commit = query.get("commit", ["working"])[0]
                referer = self.headers.get("Referer", "")
                default_fb = "false" if "/review" in referer else "true"
                include_feedback = query.get("include_feedback", [default_fb])[0].lower() in ["true", "1"]
                files = self.server.git_engine.get_changed_files(commit, include_feedback=include_feedback)
                self._send_json(files)
            case "/api/diff":
                commit = query.get("commit", ["working"])[0]
                file_path = query.get("file", [""])[0]
                if not file_path:
                    self._send_json({"error": "Missing file parameter"}, status=400)
                    return
                diff_model = self.server.git_engine.get_file_diff(commit, file_path)
                self._send_json(diff_model.model_dump(mode="json"))
            case "/api/search":
                q = query.get("q", [""])[0]
                commit = query.get("commit", ["working"])[0]
                results = self.server.git_engine.search_code(q, commit=commit)
                self._send_json({"query": q, "commit": commit, "results": results})
            case "/api/raw":
                commit = query.get("commit", ["working"])[0]
                file_path = query.get("file", [""])[0]
                side = query.get("side", ["new"])[0]
                parent = (side == "old") or (query.get("parent", ["false"])[0].lower() in ["true", "1"])
                if not file_path:
                    self._send_json({"error": "Missing file parameter"}, status=400)
                    return
                raw_bytes = self.server.git_engine.get_file_bytes(commit, file_path, parent=parent)
                mime_type, _ = mimetypes.guess_type(file_path)
                if not mime_type:
                    mime_type = "application/octet-stream"
                try:
                    self.send_response(200)
                    self.send_header("Content-Type", mime_type)
                    self.send_header("Content-Length", str(len(raw_bytes)))
                    filename = Path(file_path).name
                    self.send_header("Content-Disposition", f'inline; filename="{filename}"')
                    self.send_header("Access-Control-Allow-Origin", "*")
                    self.send_header("Connection", "close")
                    self.end_headers()
                    self.wfile.write(raw_bytes)
                except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError):
                    self.close_connection = True
            case "/api/database" | "/api/worms":
                worm_db = self.server.worm_server.database
                self._send_json(worm_db.model_dump(mode="json"))
            case "/api/next_worm_id":
                worm_server = self.server.worm_server
                next_id = worm_server.database.generate_worm_id()
                self._send_json({"next_id": next_id, "id": next_id})
            case "/api/version":
                worm_server = self.server.worm_server
                self._send_json(
                    {
                        "version": worm_server.db_version,
                        "worms_count": len(worm_server.database.worms),
                        "mtime": worm_server.feedback_mtime,
                    }
                )
            case "/api/sync_status":
                self._send_json({"initial_sync_done": self.server.initial_sync_done})
            case _:
                self.send_error(404, "Endpoint not found")

    def do_OPTIONS(self) -> None:  # noqa: N802
        """Handle CORS preflight requests."""
        self.send_response(200)
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, DELETE, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        self.send_header("Connection", "close")
        self.end_headers()

    def do_POST(self) -> None:  # noqa: N802
        """Route POST requests for mutating actions."""
        try:
            self.server.worm_server.check_file_watch()
        except Exception:
            pass
        parsed = urllib.parse.urlparse(self.path)

        path = parsed.path

        content_len = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(content_len).decode("utf-8") if content_len > 0 else "{}"
        try:
            data = json.loads(body) if body else {}
        except json.JSONDecodeError:
            self._send_json({"error": "Invalid JSON body"}, status=400)
            return

        match path:
            case "/api/sync":
                res = self.server.git_engine.sync_repo()
                try:
                    feedback_res = self.server.sync_feedback()
                    res["feedback"] = feedback_res
                except Exception:
                    pass
                self._send_json(res)
            case "/api/rebase":
                upstream = data.get("upstream") if isinstance(data, dict) else None
                res = self.server.git_engine.rebase_branch(upstream=upstream)
                self._send_json(res)
            case "/api/checkout_branch":
                branch = data.get("branch", "").strip()
                if not branch:
                    self._send_json({"error": "Missing branch parameter"}, status=400)
                    return
                try:
                    self.server.git_engine.checkout_branch(branch)
                    self._send_json({"status": "ok", "branch": branch})
                except RuntimeError as e:
                    self._send_json({"error": str(e)}, status=400)
            case "/api/goto":
                target = (data.get("commit", "") or data.get("target", "")).strip()
                if not target:
                    self._send_json({"error": "Missing commit target"}, status=400)
                    return
                try:
                    self.server.git_engine.checkout_branch(target)
                    self._send_json({"status": "ok", "commit": target})
                except RuntimeError as e:
                    self._send_json({"error": str(e)}, status=400)
            case "/api/stage":
                file_path = data.get("file", "").strip()
                if not file_path:
                    self._send_json({"error": "Missing file parameter"}, status=400)
                    return
                try:
                    self.server.git_engine.stage_file(file_path)
                    self._send_json({"status": "ok", "staged": file_path})
                except RuntimeError as e:
                    self._send_json({"error": str(e)}, status=400)
            case "/api/unstage":
                file_path = data.get("file", "").strip()
                if not file_path:
                    self._send_json({"error": "Missing file parameter"}, status=400)
                    return
                try:
                    self.server.git_engine.unstage_file(file_path)
                    self._send_json({"status": "ok", "unstaged": file_path})
                except RuntimeError as e:
                    self._send_json({"error": str(e)}, status=400)
            case "/api/discard":
                file_path = data.get("file", "").strip()
                files = data.get("files", [])
                if not file_path and not files:
                    self._send_json({"error": "Missing file parameter"}, status=400)
                    return
                targets = [f.strip() for f in files if f.strip()] if files else [file_path]
                try:
                    self.server.git_engine.discard_files(targets)
                    self._send_json({"status": "ok", "discarded": targets})
                except RuntimeError as e:
                    self._send_json({"error": str(e)}, status=400)
            case "/api/commit":
                message = data.get("message", "").strip()
                files = data.get("files")
                amend = bool(data.get("amend", False))
                if not message:
                    self._send_json({"error": "Commit message cannot be empty"}, status=400)
                    return
                try:
                    sha = self.server.git_engine.commit_files(message=message, file_paths=files, amend=amend)
                    self._send_json({"status": "ok", "commit_hash": sha})
                except (RuntimeError, ValueError) as e:
                    self._send_json({"error": str(e)}, status=400)
            case "/api/amend":
                message = data.get("message", "").strip()
                files = data.get("files")
                if not message:
                    message = self.server.git_engine.get_head_commit_message()
                if not message:
                    self._send_json({"error": "No commit message provided and no HEAD commit to amend"}, status=400)
                    return
                try:
                    sha = self.server.git_engine.commit_files(message=message, file_paths=files, amend=True)
                    self._send_json({"status": "ok", "commit_hash": sha})
                except (RuntimeError, ValueError) as e:
                    self._send_json({"error": str(e)}, status=400)
            case "/api/split":
                commit_hash = data.get("commit", "HEAD").strip()
                try:
                    res = self.server.git_engine.split_commit(commit_hash)
                    self._send_json(res)
                except RuntimeError as e:
                    self._send_json({"error": str(e)}, status=400)
            case "/api/combine":
                commits = data.get("commits", [])
                message = data.get("message", "Combined commits").strip()
                if not commits:
                    self._send_json({"error": "No commits provided to combine"}, status=400)
                    return
                try:
                    sha = self.server.git_engine.combine_commits(commits, message)
                    self._send_json({"status": "ok", "commit_hash": sha})
                except (RuntimeError, ValueError) as e:
                    self._send_json({"error": str(e)}, status=400)
            case "/api/resolve_conflict":
                file_path = data.get("file", "").strip()
                resolution = data.get("resolution", "mark_resolved").strip()
                if not file_path:
                    self._send_json({"error": "Missing file parameter"}, status=400)
                    return
                try:
                    res = self.server.git_engine.resolve_conflict(file_path, resolution)
                    self._send_json(res)
                except (RuntimeError, ValueError) as e:
                    self._send_json({"error": str(e)}, status=400)
            case "/api/pr/create" | "/api/pr/submit":
                try:
                    res = self.server.git_engine.submit_prs()
                    self._send_json(
                        {
                            "status": "ok",
                            "created": res.get("created", []),
                            "logs": res.get("logs", []),
                        }
                    )
                except (ValueError, RuntimeError) as e:
                    self._send_json({"error": str(e)}, status=400)
            case "/api/pr/unlink":
                commits = data.get("commits", [])
                if not commits:
                    self._send_json({"error": "No commits provided to unlink PR"}, status=400)
                    return
                try:
                    res = self.server.git_engine.unlink_prs_for_commits(commits)
                    self._send_json({"status": "ok", "unlinked": res})
                except (ValueError, RuntimeError) as e:
                    self._send_json({"error": str(e)}, status=400)
            case "/api/open_code_review":
                commits = data.get("commits", [])
                rev_str = ",".join(commits)
                if commits:
                    self.server.get_review_session(commits)
                target_url = f"/review?revisions={rev_str}" if rev_str else "/review"
                self._send_json({"status": "ok", "url": target_url})
            case "/api/open_worm":
                worm_id = data.get("worm_id", "").strip() or data.get("id", "").strip()
                commit = data.get("commit", "").strip()
                clean_id = worm_id.replace("WORM-", "")
                if worm_id:
                    target_url = f"/worms#WORM-{clean_id}"
                elif commit:
                    target_url = f"/worms#new?commit={commit}"
                else:
                    target_url = "/worms"
                self._send_json({"status": "ok", "url": target_url})
            case "/api/comment":
                if "id" in data and "body" in data and not data.get("file_path"):
                    self._handle_edit_comment(data)
                else:
                    self._handle_add_comment(data)
            case "/api/comment/edit":
                self._handle_edit_comment(data)
            case "/api/file_status":
                self._handle_update_file_status(data)
            case "/api/verdict":
                self._handle_update_verdict(data)
            case "/api/export":
                out_path = self.server.review_server.save_and_sync()
                self._send_json({"status": "ok", "path": str(out_path)})
            case "/api/sync_feedback" | "/api/initial_sync":
                res = self.server.sync_feedback()
                self._send_json(res)
            case "/api/commit_reviewed":
                query = urllib.parse.parse_qs(parsed.query)
                commit = query.get("commit", [""])[0] or (data.get("commit", "") if isinstance(data, dict) else "")
                short_rev = commit[:8] if commit else "current"
                print(f"\n[Dashboard Review] ✨ All changed files reviewed for commit {short_rev}!\n")
                self._send_json({"status": "ok", "commit": commit})
            case "/api/commit_update":
                self._handle_commit_update(data)
            case "/api/worms" | "/api/worm/save":
                self._handle_save_worm(data)
            case "/api/worms/delete":
                self._handle_delete_worm(data)
            case "/api/upload":
                self._handle_file_upload(data)
            case "/api/exit":
                worm_srv = self.server.worm_server
                try:
                    worm_srv.check_file_watch()
                except Exception:
                    pass
                out_path = worm_srv.save_and_sync()
                self._send_json({"status": "saved_and_exited", "path": str(out_path), "redirect_to": "/"})

            case _:
                self.send_error(404, "Endpoint not found")

    def do_DELETE(self) -> None:  # noqa: N802
        """Route DELETE requests."""
        parsed = urllib.parse.urlparse(self.path)
        path = parsed.path
        query = urllib.parse.parse_qs(parsed.query)

        match path:
            case "/api/comment":
                cid = query.get("id", [""])[0]
                if not cid:
                    self._send_json({"error": "Missing id parameter"}, status=400)
                    return
                self.server.review_server.session.comments = [
                    c for c in self.server.review_server.session.comments if c.id != cid
                ]
                self.server.review_server.save_and_sync()
                self._send_json({"status": "ok", "deleted": cid})
            case _:
                self.send_error(404, "Endpoint not found")

    def _handle_serve_diff_ui(self) -> None:
        """Render and return Jinja2 VCS diff view dashboard template."""
        templates_dir = Path(__file__).resolve().parents[2] / "templates"
        env = jinja2.Environment(
            loader=jinja2.FileSystemLoader(str(templates_dir)),
            trim_blocks=True,
            lstrip_blocks=True,
            autoescape=False,
        )
        template = env.get_template("diff_view.html.j2")
        session = self.server.build_session()
        html_out = template.render(
            session=session,
            active_branch=self.server.active_branch,
            active_commit=self.server.active_commit,
        )
        self._send_html(html_out)

    def _handle_serve_review_ui(self, query: Optional[Dict[str, List[str]]] = None) -> None:
        """Render and return Jinja2 code review dashboard template."""
        templates_dir = Path(__file__).resolve().parents[2] / "templates"
        env = jinja2.Environment(
            loader=jinja2.FileSystemLoader(str(templates_dir)),
            trim_blocks=True,
            lstrip_blocks=True,
            autoescape=False,
        )
        template = env.get_template("code_review.html.j2")
        rev_param = ""
        if query:
            rev_param = query.get("revisions", [""])[0] or query.get("commit", [""])[0]
        revs = [r.strip() for r in rev_param.split(",") if r.strip()] if rev_param else None
        session = self.server.get_review_session(revs)
        html_out = template.render(session=session)
        self._send_html(html_out)

    def _handle_serve_worm_ui(self) -> None:
        """Render and return Jinja2 worm report dashboard template."""
        templates_dir = Path(__file__).resolve().parents[2] / "templates"
        env = jinja2.Environment(
            loader=jinja2.FileSystemLoader(str(templates_dir)),
            autoescape=jinja2.select_autoescape(["html", "xml"]),
            trim_blocks=True,
            lstrip_blocks=True,
        )
        template = env.get_template("worm_report.html.j2")
        worm_server = self.server.worm_server
        db_dump = worm_server.database.model_dump(mode="json")
        html_content = template.render(
            database=worm_server.database,
            database_json=json.dumps(db_dump),
            statuses=[s.value for s in WormStatus],
            severities=[s.value for s in WormSeverity],
            categories=[c.value for c in WormCategory],
            server_port=self.server.actual_port,
        )
        self._send_html(html_content)

    def _handle_serve_static(self, path: str) -> None:
        """Serve static files such as JavaScript vendor bundles and CSS."""
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
        else:
            content_type = "text/css"
        data = file_target.read_bytes()
        try:
            self.send_response(200)
            self.send_header("Content-Type", f"{content_type}; charset=utf-8")
            self.send_header("Content-Length", str(len(data)))
            self.send_header("Access-Control-Allow-Origin", "*")
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(data)
        except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError):
            self.close_connection = True
        finally:
            self.close_connection = True

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

        att_dir = getattr(self.server.worm_server, "attachments_dir", None) or (self.server.repo_root / "attachments")
        file_path = att_dir / rel_name
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

        suffix = file_path.suffix.lower()
        content_type = {
            ".png": "image/png",
            ".jpg": "image/jpeg",
            ".jpeg": "image/jpeg",
            ".gif": "image/gif",
            ".svg": "image/svg+xml",
            ".txt": "text/plain; charset=utf-8",
            ".log": "text/plain; charset=utf-8",
            ".md": "text/markdown; charset=utf-8",
            ".json": "application/json",
            ".csv": "text/csv",
        }.get(suffix, "application/octet-stream")

        content = file_path.read_bytes()
        try:
            self.send_response(200)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(len(content)))
            self.send_header("Access-Control-Allow-Origin", "*")
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(content)
        except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError):
            self.close_connection = True
        finally:
            self.close_connection = True

    def _handle_add_comment(self, data: Dict[str, Any]) -> None:
        """Add a review comment to the session."""
        file_path = str(data.get("file_path", "")).strip()
        body = str(data.get("body", "")).strip()
        if not file_path or not body:
            self._send_json(
                {"status": "error", "error": "Missing file_path or body", "message": "Missing file_path or body"},
                status=400,
            )
            return

        commit_target = data.get("commit", "working")
        snippet = extract_line_snippet(
            file_path=file_path,
            start_line=int(data.get("start_line", 1)),
            end_line=int(data.get("end_line", 1)),
            commit=commit_target,
            repo_root=self.server.repo_root,
        )
        now_str = datetime.now(timezone.utc).isoformat()
        sev_raw = str(data.get("severity", "MUST_FIX")).strip().upper().replace(" ", "_").replace("-", "_")
        match sev_raw:
            case "MUST_FIX" | "MUSTFIX" | "FIX" | "MF":
                severity = ReviewSeverity.MUST_FIX
            case "PROPOSAL" | "PROP":
                severity = ReviewSeverity.PROPOSAL
            case "NIT" | "NITPICK":
                severity = ReviewSeverity.NIT
            case _:
                severity = ReviewSeverity.MUST_FIX

        c_uuid = str(data.get("uuid") or uuid.uuid4())
        cid = str(data.get("id") or c_uuid[:8])
        comment = CommentModel(
            id=cid,
            uuid=c_uuid,
            file_path=file_path,
            start_line=int(data.get("start_line", 1)),
            end_line=int(data.get("end_line", 1)),
            severity=severity,
            body=data.get("body", ""),
            author=data.get("author", "Reviewer"),
            code_snippet=snippet,
            created_at=now_str,
            commit=commit_target,
        )
        self.server.review_server.session.comments.append(comment)
        self.server.review_server.session.auto_update_status_on_comment()
        self.server.review_server.save_and_sync()
        resp = comment.model_dump(mode="json")
        resp["status"] = "ok"
        resp["comment"] = comment.model_dump(mode="json")
        resp["verdict"] = self.server.review_server.session.verdict.value
        self._send_json(resp)

    def _handle_edit_comment(self, data: Dict[str, Any]) -> None:
        """Edit an existing review comment."""
        cid = data.get("id", "")
        for comment in self.server.review_server.session.comments:
            if comment.id == cid:
                if "body" in data:
                    comment.body = data["body"]
                if "severity" in data:
                    sev_raw = str(data["severity"]).strip().upper().replace(" ", "_").replace("-", "_")
                    match sev_raw:
                        case "MUST_FIX" | "MUSTFIX" | "FIX" | "MF":
                            comment.severity = ReviewSeverity.MUST_FIX
                        case "PROPOSAL" | "PROP":
                            comment.severity = ReviewSeverity.PROPOSAL
                        case "NIT" | "NITPICK":
                            comment.severity = ReviewSeverity.NIT
                if "resolved" in data:
                    comment.resolved = bool(data["resolved"])
                self.server.review_server.session.auto_update_status_on_comment()
                self.server.review_server.save_and_sync()
                resp = comment.model_dump(mode="json")
                resp["status"] = "ok"
                resp["comment"] = comment.model_dump(mode="json")
                self._send_json(resp)
                return
        self._send_json({"status": "error", "error": "Comment not found", "message": "Comment not found"}, status=404)

    def _handle_update_file_status(self, data: Dict[str, Any]) -> None:
        """Update review status of a file."""
        file_path = data.get("file_path", "")
        reviewed = bool(data.get("reviewed", False))
        commit = data.get("commit", "working")
        for f in self.server.review_server.session.files:
            if f.file_path == file_path and f.commit == commit:
                f.reviewed = reviewed
                break
        else:
            from model.code_review import FileReviewModel

            self.server.review_server.session.files.append(
                FileReviewModel(file_path=file_path, reviewed=reviewed, commit=commit)
            )
        self.server.review_server.save_and_sync()
        self._send_json({"status": "ok", "file_path": file_path, "reviewed": reviewed})

    def _handle_update_verdict(self, data: Dict[str, Any]) -> None:
        """Update code review verdict and persist without server shutdown."""
        verdict_str = data.get("verdict", "")
        if verdict_str in [s.value for s in ReviewStatus]:
            self.server.review_server.session.verdict = ReviewStatus(verdict_str)
        out_path = self.server.review_server.save_and_sync()
        self._send_json(
            {
                "status": "ok",
                "session": self.server.review_server.session.model_dump(mode="json"),
                "exported_to": str(out_path),
                "terminating": False,
                "redirect_to": "/",
            }
        )

    def _handle_commit_update(self, data: Dict[str, Any]) -> None:
        """Handle commit update event (rebase, amend)."""
        orig_commit = data.get("original_commit", "")
        new_commit = data.get("new_commit", "")
        if orig_commit and new_commit:
            self.server.review_server.update_commit_hash(orig_commit, new_commit)
            self._send_json({"status": "ok", "original_commit": orig_commit, "new_commit": new_commit})
        else:
            self._send_json({"error": "Missing commit hashes"}, status=400)

    def _handle_save_worm(self, data: Dict[str, Any]) -> None:
        """Save or update a worm report."""
        worm_id = data.get("id")
        title = data.get("title", "")
        if not worm_id:
            worm_id = self.server.worm_server.database.generate_worm_id()
        worm = self.server.worm_server.database.get_worm(worm_id)
        now_str = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")

        if not worm:
            repro_steps = data.get("reproduction_steps") or data.get("steps_to_reproduce") or []
            cat_raw = data.get("category", WormCategory.FIRMWARE.value)
            try:
                cat_val = WormCategory(cat_raw)
            except ValueError:
                cat_val = WormCategory.GENERAL
            worm = WormReportModel(
                id=worm_id,
                title=title or "Untitled Defect",
                status=WormStatus(data.get("status", WormStatus.OPEN.value)),
                severity=WormSeverity(data.get("severity", WormSeverity.MEDIUM.value)),
                category=cat_val,
                component=data.get("component", ""),
                description=data.get("description", ""),
                reproduction_steps=repro_steps if isinstance(repro_steps, list) else [str(repro_steps)],
                expected_behavior=data.get("expected_behavior", ""),
                actual_behavior=data.get("actual_behavior", ""),
                logs=data.get("logs", ""),
                resolution_notes=data.get("resolution_notes", ""),
                attachments=[WormAttachmentModel(**a) for a in data.get("attachments", [])],
                created_at=now_str,
                updated_at=now_str,
            )
            self.server.worm_server.database.add_or_update(worm)
        else:
            if title:
                worm.title = title
            if "status" in data and data["status"] in [s.value for s in WormStatus]:
                new_status = WormStatus(data["status"])
                if (
                    worm.status in (WormStatus.RESOLVED, WormStatus.CLOSED)
                    and worm.resolution_notes.strip()
                    and new_status == WormStatus.OPEN
                    and not data.get("resolution_notes", "").strip()
                ):
                    pass
                else:
                    worm.status = new_status
                    if worm.status in (WormStatus.RESOLVED, WormStatus.CLOSED):
                        worm.resolved_at = now_str
                    else:
                        worm.resolved_at = None

            if "severity" in data and data["severity"] in [s.value for s in WormSeverity]:
                worm.severity = WormSeverity(data["severity"])
            if "category" in data and data["category"] in [c.value for c in WormCategory]:
                worm.category = WormCategory(data["category"])
            if "component" in data:
                worm.component = data["component"]
            if "description" in data:
                worm.description = data["description"]
            if "expected_behavior" in data:
                worm.expected_behavior = data["expected_behavior"]
            if "actual_behavior" in data:
                worm.actual_behavior = data["actual_behavior"]
            if "logs" in data:
                worm.logs = data["logs"]
            if "resolution_notes" in data:
                incoming_notes = data["resolution_notes"].strip()
                if incoming_notes:
                    worm.resolution_notes = incoming_notes
                elif worm.status not in (WormStatus.RESOLVED, WormStatus.CLOSED):
                    worm.resolution_notes = ""
            if "reproduction_steps" in data:
                repro_steps = data["reproduction_steps"]
                worm.reproduction_steps = repro_steps if isinstance(repro_steps, list) else [str(repro_steps)]
            elif "steps_to_reproduce" in data:
                repro_steps = data["steps_to_reproduce"]
                worm.reproduction_steps = repro_steps if isinstance(repro_steps, list) else [str(repro_steps)]
            if "attachments" in data:
                worm.attachments = [WormAttachmentModel(**a) for a in data["attachments"]]
            worm.updated_at = now_str

        self.server.worm_server.save_and_sync()
        self._send_json(worm.model_dump(mode="json"))

    def _handle_delete_worm(self, data: Dict[str, Any]) -> None:
        """Delete a worm report by ID."""
        worm_id = data.get("id")
        self.server.worm_server.database.worms = [w for w in self.server.worm_server.database.worms if w.id != worm_id]
        self.server.worm_server.save_and_sync()
        self._send_json({"status": "deleted", "id": worm_id})

    def _handle_file_upload(self, data: Dict[str, Any]) -> None:
        """Save base64 encoded file upload to attachments directory."""
        filename = data.get("filename", f"attachment_{uuid.uuid4().hex[:8]}.txt")
        file_type = data.get("file_type", "reference")
        desc = data.get("description", "")
        b64_content = data.get("content_base64", "")
        text_content = data.get("content_text", "")

        raw_worm_id = str(data.get("worm_id") or data.get("wormId") or "").strip()
        if not raw_worm_id and str(data.get("id", "")).startswith("WORM-"):
            raw_worm_id = str(data.get("id")).strip()
        worm_id = Path(raw_worm_id).name if raw_worm_id else ""
        filename = Path(filename).name

        att_dir = getattr(self.server.worm_server, "attachments_dir", None) or (self.server.repo_root / "attachments")
        dest_dir = (att_dir / worm_id) if worm_id else att_dir
        dest_dir.mkdir(parents=True, exist_ok=True)
        dest_path = dest_dir / filename

        if b64_content:
            file_bytes = base64.b64decode(b64_content)
            dest_path.write_bytes(file_bytes)
        elif text_content:
            file_bytes = text_content.encode("utf-8")
            dest_path.write_bytes(file_bytes)
        else:
            file_bytes = b""
            dest_path.write_bytes(file_bytes)

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

    def _send_html(self, html: str) -> None:
        """Send HTML payload with UTF-8 encoding."""
        encoded = html.encode("utf-8")
        try:
            self.send_response(200)
            self.send_header("Content-Type", "text/html; charset=utf-8")
            self.send_header("Content-Length", str(len(encoded)))
            self.send_header("Access-Control-Allow-Origin", "*")
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(encoded)
        except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError):
            self.close_connection = True
        finally:
            self.close_connection = True

    def _send_json(self, data: Any, status: int = 200) -> None:
        """Send JSON response payload."""
        encoded = json.dumps(data, indent=2).encode("utf-8")
        try:
            self.send_response(status)
            self.send_header("Content-Type", "application/json; charset=utf-8")
            self.send_header("Content-Length", str(len(encoded)))
            self.send_header("Access-Control-Allow-Origin", "*")
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(encoded)
        except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError):
            self.close_connection = True
        finally:
            self.close_connection = True


class DashboardServer(ThreadingHTTPServer):
    """Unified HTTP server for VCS Smartlog diff workstation, code review, and worm tracking."""

    allow_reuse_address = True
    daemon_threads = True

    def server_bind(self) -> None:
        """Override server_bind to avoid slow reverse DNS lookups via getfqdn."""
        from socketserver import TCPServer

        TCPServer.server_bind(self)
        self.server_name = self.server_address[0]
        self.server_port = self.server_address[1]

    def get_request(self) -> Any:
        """Accept incoming connection and set client socket timeout to prevent lingering sockets."""
        sock, addr = super().get_request()
        sock.settimeout(10.0)
        return sock, addr

    def handle_error(self, request: Any, client_address: Any) -> None:
        """Handle client connection errors gracefully without printing tracebacks on client disconnects."""
        exc_type, _, _ = sys.exc_info()
        if exc_type is not None and issubclass(
            exc_type, (BrokenPipeError, ConnectionResetError, ConnectionAbortedError)
        ):
            return
        super().handle_error(request, client_address)

    def __init__(
        self,
        host: str = "127.0.0.1",
        port: int = 8777,
        repo_root: Optional[Path] = None,
        initial_branch: Optional[str] = None,
        initial_commit: Optional[str] = None,
        sqlite_worm_file: Optional[Path] = None,
        sqlite_review_file: Optional[Path] = None,
        markdown_worms_path: Optional[Path] = None,
        markdown_review_path: Optional[Path] = None,
        revisions: Optional[List[str]] = None,
        fresh: bool = False,
        bind_and_activate: bool = True,
    ) -> None:
        """Initialize the unified dashboard workstation server."""
        self.repo_root = (repo_root or get_git_root()).resolve()
        self.git_engine = GitEngine(repo_root=self.repo_root)
        self.host = host
        self.port = port
        self.active_branch = initial_branch or self.git_engine.get_current_branch()
        self.active_commit = initial_commit or "working"
        self.initial_sync_done: bool = False

        md_review = markdown_review_path
        if md_review is None and sqlite_review_file and sqlite_review_file.parent.name not in ("build", "target"):
            md_review = sqlite_review_file.parent / "CR.md"

        md_worms = markdown_worms_path
        if md_worms is None and sqlite_worm_file and sqlite_worm_file.parent.name not in ("build", "target"):
            md_worms = sqlite_worm_file.parent / "WORMS.md"

        review_state = sqlite_review_file.with_suffix(".json") if sqlite_review_file else None
        worm_state = sqlite_worm_file.with_suffix(".json") if sqlite_worm_file else None

        # Embedded review and worm servers without duplicate socket binding
        self.review_server = ReviewServer(
            host=host,
            port=port,
            repo_root=self.repo_root,
            markdown_output=md_review,
            state_file=review_state,
            sqlite_file=sqlite_review_file,
            revisions=revisions,
            fresh=fresh,
            bind_and_activate=False,
        )
        self.worm_server = WormReportServer(
            host=host,
            port=port,
            repo_root=self.repo_root,
            markdown_output=md_worms,
            state_file=worm_state,
            sqlite_file=sqlite_worm_file,
            fresh=fresh,
            bind_and_activate=False,
        )

        if bind_and_activate:
            bound_port = port
            while True:
                try:
                    super().__init__((host, bound_port), DashboardRequestHandler)
                    break
                except OSError as err:
                    if bound_port == 0 or bound_port > port + 50:
                        raise err
                    bound_port += 1
            self.actual_port = self.server_port
            self.socket.settimeout(10.0)
        else:
            self.actual_port = port

    def get_url(self) -> str:
        """Return the base local HTTP URL for the dashboard workstation."""
        return f"http://{self.host}:{self.actual_port}"

    def build_session(self) -> DiffViewSessionModel:
        """Build DiffViewSessionModel snapshot of the current repository state."""
        branches = self.git_engine.get_branches()
        curr_branch = self.git_engine.get_current_branch()
        head_sha = self.git_engine.get_head_commit()
        smartlog_nodes = self.git_engine.get_smartlog_dag(limit=50)
        working_files = self.git_engine.get_working_tree_files()
        conflicts = self.git_engine.get_merge_conflicts()
        agent_feedback = [f for f in working_files if f.is_feedback]

        # Match worm reports with commits
        worm_dict: Dict[str, Any] = {}
        for w in self.worm_server.database.worms:
            num = re.sub(r"^(?:WORM|BUG)[-_]", "", w.id, flags=re.IGNORECASE)
            worm_dict[f"WORM-{num}"] = w
            worm_dict[f"BUG-{num}"] = w
            worm_dict[w.id] = w

        for node in smartlog_nodes:
            # Canonicalize existing tag IDs
            for tag in node.worm_tags:
                num = re.sub(r"^(?:WORM|BUG)[-_]", "", tag.id, flags=re.IGNORECASE)
                tag.id = f"WORM-{num}"
                if tag.id in worm_dict:
                    w = worm_dict[tag.id]
                    tag.title = w.title
                    tag.status = w.status.value
                    tag.severity = w.severity.value

            worm_ids = re.findall(r"\b((?:WORM|BUG)[_-]\d+)\b", node.subject, re.IGNORECASE)
            existing_ids = {t.id for t in node.worm_tags}
            for wid in worm_ids:
                num = re.sub(r"^(?:WORM|BUG)[-_]", "", wid, flags=re.IGNORECASE)
                canonical_id = f"WORM-{num}"
                if canonical_id not in existing_ids:
                    if canonical_id in worm_dict:
                        w = worm_dict[canonical_id]
                        node.worm_tags.append(
                            CommitWormTagModel(
                                id=canonical_id,
                                title=w.title,
                                status=w.status.value,
                                severity=w.severity.value,
                            )
                        )
                    else:
                        node.worm_tags.append(
                            CommitWormTagModel(
                                id=canonical_id,
                                title=canonical_id,
                                status="OPEN",
                                severity="LOW",
                            )
                        )
                    existing_ids.add(canonical_id)

        # Load Code Review stats for commits (BUG-199)
        cr_stats: dict[str, dict[str, Any]] = {}
        cr_db_path = self.repo_root / "target" / "code_review.sqlite"
        if not cr_db_path.exists() and (self.repo_root / "build" / "code_review.sqlite").exists():
            cr_db_path = self.repo_root / "build" / "code_review.sqlite"
        if cr_db_path.exists():
            import sqlite3

            try:
                with sqlite3.connect(str(cr_db_path)) as conn:
                    cursor = conn.cursor()
                    cursor.execute(
                        "SELECT commit_hash, COUNT(*), SUM(CASE WHEN resolved = 0 THEN 1 ELSE 0 END), SUM(CASE WHEN resolved = 1 THEN 1 ELSE 0 END) "
                        "FROM comments GROUP BY commit_hash"
                    )
                    for row in cursor.fetchall():
                        c_hash, total, open_c, res_c = row
                        if c_hash:
                            cr_stats[c_hash] = {
                                "total": total or 0,
                                "open": open_c or 0,
                                "resolved": res_c or 0,
                                "reviewed": True,
                            }
                    cursor.execute("SELECT key, value FROM metadata WHERE key IN ('revisions', 'commit_hash')")
                    meta = dict(cursor.fetchall())
                    revs = meta.get("revisions", "")
                    for node in smartlog_nodes:
                        if node.commit_hash in revs or node.short_hash in revs:
                            cr_stats.setdefault(node.commit_hash, {"total": 0, "open": 0, "resolved": 0})[
                                "reviewed"
                            ] = True
            except Exception:
                pass

        # Ensure file watcher syncs any newly placed or edited CR feedback files (BUG-236)
        if hasattr(self, "review_server") and self.review_server:
            try:
                self.review_server.check_file_watch()
            except Exception:
                pass

        # Load Code Review stats for commits (BUG-199)
        cr_db_path = (
            self.review_server.sqlite_file
            if hasattr(self, "review_server") and self.review_server
            else (self.repo_root / "target" / "code_review.sqlite")
        )
        if not cr_db_path.exists() and (self.repo_root / "build" / "code_review.sqlite").exists():
            cr_db_path = self.repo_root / "build" / "code_review.sqlite"
        if cr_db_path.exists():
            import sqlite3

            try:
                with sqlite3.connect(str(cr_db_path)) as conn:
                    cursor = conn.cursor()
                    cursor.execute(
                        "SELECT commit_hash, COUNT(*), SUM(CASE WHEN resolved = 0 THEN 1 ELSE 0 END), SUM(CASE WHEN resolved = 1 THEN 1 ELSE 0 END) "
                        "FROM comments GROUP BY commit_hash"
                    )
                    for row in cursor.fetchall():
                        c_hash, total, open_c, res_c = row
                        if c_hash:
                            cr_stats[c_hash] = {
                                "total": total or 0,
                                "open": open_c or 0,
                                "resolved": res_c or 0,
                                "reviewed": True,
                            }
                    cursor.execute("SELECT key, value FROM metadata WHERE key IN ('revisions', 'commit_hash')")
                    meta = dict(cursor.fetchall())
                    revs = meta.get("revisions", "")
                    for node in smartlog_nodes:
                        if node.commit_hash in revs or node.short_hash in revs:
                            cr_stats.setdefault(node.commit_hash, {"total": 0, "open": 0, "resolved": 0})[
                                "reviewed"
                            ] = True
            except Exception:
                pass

        for node in smartlog_nodes:
            if node.commit_hash in cr_stats:
                st = cr_stats[node.commit_hash]
                node.cr_open_count = st["open"]
                node.cr_resolved_count = st["resolved"]
                node.cr_total_count = st["total"]
                node.cr_reviewed = st.get("reviewed", False)

        repo_web_url = self.git_engine.get_repo_web_url()
        github_repo = ""
        if repo_web_url and "github.com/" in repo_web_url:
            github_repo = repo_web_url.split("github.com/", 1)[1].rstrip("/")

        return DiffViewSessionModel(
            repo_name=self.repo_root.name,
            repo_root=str(self.repo_root),
            repo_web_url=repo_web_url,
            github_repo=github_repo,
            branches=branches,
            current_branch=curr_branch,
            head_commit=head_sha,
            active_commit=self.active_commit,
            smartlog_nodes=smartlog_nodes,
            working_files=working_files,
            conflicts=conflicts,
            agent_feedback_files=agent_feedback,
            initial_sync_done=self.initial_sync_done,
        )

    def get_review_session(self, revisions: Optional[List[str]] = None) -> ReviewSessionModel:
        """Get or initialize a review session for specific revisions or the default session."""
        if hasattr(self, "review_server") and self.review_server:
            try:
                self.review_server.check_file_watch()
            except Exception:
                pass

        if not revisions:
            if self.review_server.session:
                self.review_server.session.repo_root = str(self.repo_root)
            return self.review_server.session

        resolved = self.git_engine.resolve_revisions(revisions)
        if not resolved:
            if self.review_server.session:
                self.review_server.session.repo_root = str(self.repo_root)
            return self.review_server.session

        commit_hash = resolved[0]
        if (
            self.review_server.session
            and self.review_server.session.commit_hash == commit_hash
            and set(self.review_server.session.revisions) == set(resolved)
        ):
            self.review_server.session.repo_root = str(self.repo_root)
            return self.review_server.session

        session = ReviewSessionModel(
            title=f"Code Review: {self.repo_root.name}",
            repo_name=self.repo_root.name,
            repo_root=str(self.repo_root),
            commit_hash=commit_hash,
            revisions=resolved,
            created_at=datetime.now(timezone.utc).isoformat(),
            updated_at=datetime.now(timezone.utc).isoformat(),
        )

        if self.review_server.feedback_dir.exists():
            cr_commit_path = self.review_server.feedback_dir / f"CR_{commit_hash}.md"
            if cr_commit_path.exists():
                session = self.review_server.exporter.merge_commit_feedback(session, cr_commit_path)
            elif (self.review_server.feedback_dir / "CR.md").exists():
                session = self.review_server.exporter.merge_commit_feedback(
                    session, self.review_server.feedback_dir / "CR.md"
                )

        if self.review_server.sqlite_store:
            try:
                with self.review_server.sqlite_store._get_connection() as conn:
                    comment_rows = conn.execute(
                        "SELECT id, uuid, commit_hash, file_path, start_line, end_line, severity, body, author, code_snippet, created_at, resolved "
                        "FROM comments WHERE commit_hash = ? OR commit_hash = '' ORDER BY file_path, start_line",
                        (commit_hash,),
                    ).fetchall()
                    existing_ids = {c.id for c in session.comments}
                    for row in comment_rows:
                        if row["id"] not in existing_ids:
                            session.comments.append(
                                CommentModel(
                                    id=row["id"],
                                    uuid=row["uuid"] or row["id"],
                                    commit=row["commit_hash"] or "",
                                    file_path=row["file_path"],
                                    start_line=row["start_line"],
                                    end_line=row["end_line"],
                                    severity=ReviewSeverity(row["severity"]),
                                    body=row["body"],
                                    author=row["author"],
                                    code_snippet=row["code_snippet"],
                                    created_at=row["created_at"],
                                    resolved=bool(row["resolved"]),
                                )
                            )
            except Exception:
                pass

        self.review_server.session = session
        return session

    def sync_feedback(self) -> Dict[str, Any]:
        """Synchronize review and worm report feedback from workspace feedback/ directory."""
        review_res = self.review_server.sync_feedback()
        worm_res = self.worm_server.sync_with_feedback_dir()
        self.initial_sync_done = True
        return {"status": "ok", "initial_sync_done": True, "review": review_res, "worms": worm_res}

    def server_close(self) -> None:
        """Close server sockets and cleanup sub-servers."""
        self.review_server.server_close()
        self.worm_server.server_close()
        if hasattr(self, "socket") and self.socket:
            try:
                super().server_close()
            except Exception:
                pass
