#!/usr/bin/env python3
"""Regression functional test for WORM-SLICK-FIREWORM-677.

Ensures that host_cli `dump_logs` and `stream_logs` do not emit false-positive
"Error: unexpected EOF during frame decoding (possible target/ELF mismatch or truncated log buffer)"
when draining buffered RTT logs or reaching the end of available frames in a chunk.
"""

from pathlib import Path
import subprocess


def test_worm_slick_fireworm_677_dump_logs_no_unexpected_eof_functional():
    """Verify dump_logs and stream_logs handle UnexpectedEof cleanly without spurious error logs."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    cmd = [
        "cargo",
        "test",
        "--package",
        "host_cli",
        "--test",
        "host_cli_tests",
        "test_worm_slick_fireworm_677_dump_logs_no_unexpected_eof",
        "--",
        "--nocapture",
    ]
    proc = subprocess.run(
        cmd,
        cwd=workspace_root,
        capture_output=True,
        text=True,
        check=False,
    )
    assert proc.returncode == 0, (
        f"cargo test failed with return code {proc.returncode}:\nSTDOUT:\n{proc.stdout}\nSTDERR:\n{proc.stderr}"
    )
    assert (
        "test test_worm_slick_fireworm_677_dump_logs_no_unexpected_eof ... ok"
        in proc.stdout
    ), f"Expected test success in cargo test output, got:\n{proc.stdout}"
    assert "1 passed; 0 failed" in proc.stdout, (
        f"Expected 1 passed in cargo test output, got:\n{proc.stdout}"
    )
