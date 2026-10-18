"""Reproduction and regression test for WORM-BRISTLY-GLOWWORM-390.

Ensures that the host tool target bug page (host_daemon.html.j2) includes the
"Attachments & Screenshots" UI section with:
- Drag-and-drop / click-to-browse dropzone (#targetBugDropzone)
- File input element (#targetBugFileInput)
- Attachments grid (#targetBugAttachmentsGrid)
- Paste from clipboard button and paste handlers
- Git LFS non-confidential notice
- Removal of attached items before submission
- Server API /api/target_bug handles manual attachments and screenshots,
  persisting them to disk and recording them in the worm report.
"""

import base64
import json
from pathlib import Path
import tempfile
from unittest.mock import MagicMock

from host_daemon.provider.daemon.runner import DaemonRunner
from host_daemon.provider.daemon.server import HostDaemonRequestHandler, HostDaemonServer


TEMPLATES_DIR = Path(__file__).resolve().parent.parent / "host_daemon" / "templates"


def test_worm_bristly_glowworm_390_attachments_ui_in_template() -> None:
    """Verify Attachments & Screenshots UI elements exist in host_daemon.html.j2."""
    template_path = TEMPLATES_DIR / "host_daemon.html.j2"
    assert template_path.exists(), "host_daemon.html.j2 not found"
    content = template_path.read_text(encoding="utf-8")

    # 1. Section Header / Label
    assert "Attachments &amp; Screenshots" in content or "Attachments & Screenshots" in content, (
        "Missing 'Attachments & Screenshots' section in host_daemon.html.j2"
    )

    # 2. Dropzone & File Input
    assert 'id="targetBugDropzone"' in content, "Missing #targetBugDropzone dropzone container"
    assert 'id="targetBugFileInput"' in content, "Missing #targetBugFileInput file input"
    assert 'id="targetBugAttachmentsGrid"' in content, "Missing #targetBugAttachmentsGrid grid"

    # 3. Paste Clipboard button & handler functions
    assert "pasteTargetBugClipboard" in content, "Missing pasteTargetBugClipboard function"
    assert "handleTargetBugFileUpload" in content, "Missing handleTargetBugFileUpload function"
    assert "removeTargetBugAttachment" in content, "Missing removeTargetBugAttachment function"

    # 4. LFS non-confidential notice
    assert "Attachments are tracked in GitHub LFS and considered non-confidential" in content, (
        "Missing GitHub LFS non-confidentiality notice"
    )


def test_worm_bristly_glowworm_390_server_api_processes_manual_attachments() -> None:
    """Verify /api/target_bug persists manual screenshots/attachments to disk and markdown."""
    with tempfile.TemporaryDirectory() as tmp_dir:
        repo_root = Path(tmp_dir)
        (repo_root / "feedback").mkdir()
        (repo_root / "attachments").mkdir()
        (repo_root / "target").mkdir()

        runner = DaemonRunner(repo_root=repo_root, package="cat_detector", bin_name="cat_detector_shell")
        runner.log("Host session active", level="INFO", channel="daemon")

        server = HostDaemonServer("127.0.0.1", 0, runner)

        # Create dummy screenshot base64
        dummy_png_bytes = (
            b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15c4"
        )
        dummy_png_b64 = "data:image/png;base64," + base64.b64encode(dummy_png_bytes).decode("ascii")

        dummy_log_text = "TRACE 12:34:56.789 Motor PWM undervoltage detected\nTRACE 12:34:56.790 DIAG active"

        payload = {
            "title": "Phase V low side fault under load",
            "severity": "HIGH",
            "category": "FIRMWARE",
            "component": "tmc6300",
            "description": "Observed overcurrent on Phase V during Step 0 commutation.",
            "reproduction_steps": ["1. Set motor speed 50", "2. Monitor DIAG pin"],
            "expected_behavior": "Smooth commutation without fault",
            "actual_behavior": "DIAG asserts on step 0",
            "attach_system_logs": True,
            "attach_crash_log": False,
            "attach_telemetry": False,
            "attach_metadata": False,
            "manual_attachments": [
                {
                    "filename": "scope_phase_v.png",
                    "file_type": "screenshot",
                    "content_base64": dummy_png_b64,
                    "description": "Oscilloscope capture of VL and VH pins",
                },
                {
                    "filename": "trace_dump.log",
                    "file_type": "log",
                    "content_text": dummy_log_text,
                    "description": "Logic analyzer trace export",
                },
            ],
        }

        handler = HostDaemonRequestHandler.__new__(HostDaemonRequestHandler)
        handler.server = server
        handler.path = "/api/target_bug"
        handler.headers = {"Content-Length": str(len(json.dumps(payload)))}
        handler.rfile = MagicMock()
        handler.rfile.read.return_value = json.dumps(payload).encode("utf-8")
        handler._send_json = MagicMock()
        handler.send_error = MagicMock()

        handler.do_POST()

        assert handler._send_json.called, f"Expected _send_json, got error: {handler.send_error.call_args}"
        res_data = handler._send_json.call_args[0][0]
        assert res_data.get("success") is True
        worm_id = res_data.get("worm_id")
        assert worm_id

        # 1 system log + 2 manual attachments = 3 total attachments
        assert res_data.get("attachments_count") == 3

        worm_att_dir = repo_root / "attachments" / worm_id
        assert worm_att_dir.exists(), f"Attachment directory {worm_att_dir} does not exist"

        scope_file = worm_att_dir / "scope_phase_v.png"
        assert scope_file.exists(), f"File {scope_file} was not written to disk"
        assert scope_file.read_bytes() == dummy_png_bytes

        trace_file = worm_att_dir / "trace_dump.log"
        assert trace_file.exists(), f"File {trace_file} was not written to disk"
        assert trace_file.read_text(encoding="utf-8") == dummy_log_text

        # Verify exported markdown contains references to manual attachments
        clean_id = worm_id.removeprefix("WORM-").removeprefix("WORM_")
        worm_md = repo_root / "feedback" / f"WORM_{clean_id}.md"
        assert worm_md.exists()
        md_text = worm_md.read_text(encoding="utf-8")
        assert "scope_phase_v.png" in md_text
        assert "trace_dump.log" in md_text
