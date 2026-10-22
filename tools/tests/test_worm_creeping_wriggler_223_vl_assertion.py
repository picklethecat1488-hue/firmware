#!/usr/bin/env python3
"""Regression functional test for WORM-CREEPING-WRIGGLER-223.

Ensures that the TMC6300 6-step trapezoidal commutation sequence asserts the VL (Low-side Phase V)
pin (bit 3 / GP19) during commutation steps 0 and 5 in forward rotation and step 0 and 5 in reverse
rotation:
  - Bit 0: UH (GP16)
  - Bit 1: UL (GP17)
  - Bit 2: VH (GP18)
  - Bit 3: VL (GP19)
  - Bit 4: WH (GP20)
  - Bit 5: WL (GP21)

Rather than checking for source code strings or AST tokens, this test executes the actual
compiled functional unit/integration test in peripheral/tests/integration.rs.
"""

from pathlib import Path
import subprocess


def test_worm_creeping_wriggler_223_vl_assertion_functional():
    """Verify VL assertion and correct commutation phase masks by executing the Rust functional integration test."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    cmd = [
        "cargo",
        "test",
        "--package",
        "peripheral",
        "--test",
        "integration",
        "test_worm_creeping_wriggler_223_vl_pin_assertion_and_commutation_masks",
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
        "test test_worm_creeping_wriggler_223_vl_pin_assertion_and_commutation_masks ... ok"
        in proc.stdout
    ), f"Expected test success in cargo test output, got:\n{proc.stdout}"
    assert "1 passed; 0 failed" in proc.stdout, (
        f"Expected 1 passed in cargo test output, got:\n{proc.stdout}"
    )
