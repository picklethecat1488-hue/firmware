import json
import os
import sys
import threading
import io
from unittest.mock import patch, MagicMock

# Ensure scripts directory is in path
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import validate_all


def test_thread_local_stdout_prefixing():
    """Verify that ThreadLocalStdout correctly prefixes lines when a thread name is active."""
    original = io.StringIO()
    proxy = validate_all.ThreadLocalStdout(original)

    # Without name set, should not prefix
    proxy.write("hello\nworld\n")
    assert original.getvalue() == "hello\nworld\n"

    # Reset original stream
    original.seek(0)
    original.truncate(0)

    # Patch stdout_proxy to proxy so log_status writes to our StringIO
    with patch("validate_all.stdout_proxy", proxy):
        # Set thread-local name
        proxy.local.name = "TestTask"
        proxy.write("hello\n")
        assert original.getvalue() == "[TestTask] hello\n"

        # Reset
        original.seek(0)
        original.truncate(0)

        # Multiline write
        proxy.write("first\nsecond\n")
        assert original.getvalue() == "[TestTask] first\n[TestTask] second\n"

        # Clean up name
        del proxy.local.name


def test_thread_local_stdout_buffering():
    """Verify that ThreadLocalStdout correctly captures output to a thread-local buffer."""
    original = io.StringIO()
    proxy = validate_all.ThreadLocalStdout(original)

    # Set thread-local buffer
    proxy.local.buffer = []
    proxy.write("buffered text")
    assert proxy.local.buffer == ["buffered text"]
    assert original.getvalue() == "buffered text"


def test_get_exclude_args():
    """Verify that get_exclude_args correctly extracts target workspace exclusions."""
    mock_metadata = {
        "packages": [
            {"name": "tool_a", "manifest_path": "/workspace/tools/tool_a/Cargo.toml"},
            {"name": "host_b", "manifest_path": "/workspace/platform/host/host_b/Cargo.toml"},
            {"name": "target_c", "manifest_path": "/workspace/controller/Cargo.toml"},
        ]
    }
    with patch("subprocess.run") as mock_run:
        mock_run.return_value = MagicMock(stdout=json.dumps(mock_metadata), returncode=0)
        exclude_args = validate_all.get_exclude_args()
        assert "--exclude" in exclude_args
        assert "tool_a" in exclude_args
        assert "host_b" in exclude_args
        assert "target_c" not in exclude_args


def test_run_cmd_success():
    """Verify that run_cmd handles subprocesses and reports execution success correctly."""
    with patch("subprocess.Popen") as mock_popen:
        mock_proc = MagicMock()
        mock_proc.stdout = ["hello line 1\n", "hello line 2\n"]
        mock_proc.returncode = 0
        mock_popen.return_value = mock_proc

        name, success = validate_all.run_cmd("TestCmd", ["echo", "hello"])
        assert name == "TestCmd"
        assert success is True


def test_run_python_validator_success():
    """Verify that run_python_validator executes imported validators and reports status."""

    def mock_func():
        print("inner print")
        return 0

    name, success = validate_all.run_python_validator("TestPyVal", mock_func)
    assert name == "TestPyVal"
    assert success is True
