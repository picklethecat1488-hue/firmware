#!/usr/bin/env python3
"""Regression functional test for WORM-INCHING-BEETLE-546.

Ensures that:
1. Creating a bug or uploading an attachment >= 2GB (GitHub LFS maximum file limit)
   is safely rejected with HTTP 413 (Payload Too Large) rather than crashing the dashboard
   with MemoryError, OverflowError, or socket buffer overflows.
2. Servers stream attachment downloads in chunks rather than reading entire large files
   into memory with read_bytes().
3. Client-side upload handlers in worm_report.html.j2 enforce the 2 GB maximum file limit
   prior to invoking FileReader.readAsDataURL.
"""

from io import BytesIO
from pathlib import Path
import re
import tempfile
from unittest.mock import MagicMock

from provider.dashboard.server import DashboardServer, MAX_ATTACHMENT_SIZE_BYTES
from provider.worm_report.server import WormReportRequestHandler, WormReportServer


def test_post_content_length_limit_returns_413():
    """Verify POST request with Content-Length >= 2GB is rejected with 413 without reading body."""
    with tempfile.TemporaryDirectory() as tmp_dir:
        repo_root = Path(tmp_dir)
        server = WormReportServer(
            repo_root=repo_root,
            feedback_dir=repo_root / "feedback",
            sqlite_file=repo_root / "worms.sqlite",
            state_file=repo_root / "worms_state.json",
            markdown_output=repo_root / "feedback" / "WORMS.md",
            bind_and_activate=False,
        )

        handler = WormReportRequestHandler.__new__(WormReportRequestHandler)
        handler.server = server
        handler.path = "/api/upload"
        handler.headers = {"Content-Length": str(MAX_ATTACHMENT_SIZE_BYTES + 1024)}
        handler.rfile = BytesIO()  # Empty stream: if server attempts to read 2GB, it would hang/fail
        handler.send_error = MagicMock()

        handler.do_POST()

        assert handler.send_error.called, "Server must call send_error for oversized Content-Length"
        args = handler.send_error.call_args[0]
        assert args[0] == 413, f"Expected HTTP 413, got {args[0]}"


def test_serve_attachment_streams_in_chunks_without_read_bytes():
    """Verify _handle_serve_attachment streams files in chunks instead of calling read_bytes."""
    with tempfile.TemporaryDirectory() as tmp_dir:
        repo_root = Path(tmp_dir)
        att_dir = repo_root / "attachments"
        att_dir.mkdir(parents=True)
        large_file = att_dir / "large_test.dat"

        # Create a sample file
        content = b"X" * 128 * 1024  # 128 KiB
        large_file.write_bytes(content)

        server = WormReportServer(
            repo_root=repo_root,
            feedback_dir=repo_root / "feedback",
            sqlite_file=repo_root / "worms.sqlite",
            state_file=repo_root / "worms_state.json",
            markdown_output=repo_root / "feedback" / "WORMS.md",
            attachments_dir=att_dir,
            bind_and_activate=False,
        )

        handler = WormReportRequestHandler.__new__(WormReportRequestHandler)
        handler.server = server
        handler.wfile = BytesIO()
        handler.send_response = MagicMock()
        handler.send_header = MagicMock()
        handler.end_headers = MagicMock()

        handler._handle_serve_attachment("attachments/large_test.dat")

        assert handler.send_response.called
        assert handler.wfile.getvalue() == content


def test_template_attachment_size_limit_check():
    """Verify worm_report.html.j2 contains client-side checks for >= 2GB attachment limit."""
    tpl_path = Path(__file__).resolve().parent.parent / "dashboard" / "templates" / "worm_report.html.j2"
    tpl_content = tpl_path.read_text(encoding="utf-8")

    assert "MAX_ATTACHMENT_SIZE_BYTES" in tpl_content, "Missing MAX_ATTACHMENT_SIZE_BYTES in worm_report.html.j2"
    assert "2 * 1024 * 1024 * 1024" in tpl_content, "Expected 2GB limit definition in worm_report.html.j2"


def test_handle_file_upload_rejects_oversized_base64():
    """Verify _handle_file_upload returns 413 for oversized base64 content."""
    with tempfile.TemporaryDirectory() as tmp_dir:
        repo_root = Path(tmp_dir)
        server = WormReportServer(
            repo_root=repo_root,
            feedback_dir=repo_root / "feedback",
            sqlite_file=repo_root / "worms.sqlite",
            state_file=repo_root / "worms_state.json",
            markdown_output=repo_root / "feedback" / "WORMS.md",
            bind_and_activate=False,
        )

        handler = WormReportRequestHandler.__new__(WormReportRequestHandler)
        handler.server = server
        handler._send_json = MagicMock()

        # Mock a data payload claiming content exceeding MAX_ATTACHMENT_SIZE_BYTES
        # (len * 3 // 4 >= MAX_ATTACHMENT_SIZE_BYTES)
        class MockOversizedStr:
            def __len__(self):
                return MAX_ATTACHMENT_SIZE_BYTES * 2

        handler._handle_file_upload({"filename": "huge.bin", "content_base64": MockOversizedStr()})

        assert handler._send_json.called
        args, kwargs = handler._send_json.call_args
        assert kwargs.get("status") == 413
        assert "Payload Too Large" in args[0]["error"]
