#!/usr/bin/env python3
"""Regression functional test for WORM-GLIDING-LARVA-522.

Verifies that the TMC6300 commutation sequence correctly advances across all 6 trapezoidal
commutation phases (steps 0..=5) without stalling or faulting at phase 1.

Verifies canonical board pin assignments (GP16=UH, GP17=UL, GP18=VH, GP19=VL, GP20=WH, GP21=WL, GP22=DIAG)
and executes the compiled integration tests in peripheral and board crates.
"""

from pathlib import Path
import subprocess


def test_worm_gliding_larva_522_commutation_phase_progression_functional():
    """Verify forward commutation advances past phase 1 across multiple electrical cycles."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    cmd = [
        "cargo",
        "test",
        "--package",
        "peripheral",
        "--test",
        "integration",
        "test_worm_gliding_larva_522_pin_mapping_and_forward_commutation_progression",
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
        "test test_worm_gliding_larva_522_pin_mapping_and_forward_commutation_progression ... ok"
        in proc.stdout
    ), f"Expected test success in cargo test output, got:\n{proc.stdout}"
    assert "1 passed; 0 failed" in proc.stdout, (
        f"Expected 1 passed in cargo test output, got:\n{proc.stdout}"
    )


def test_worm_gliding_larva_522_canonical_board_pins_functional():
    """Verify board pin constants match canonical physical PCB wiring."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    cmd = [
        "cargo",
        "test",
        "--package",
        "board",
        "--test",
        "board_tests",
        "test_board_pin_constants",
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
    assert "test test_board_pin_constants ... ok" in proc.stdout, (
        f"Expected test success in cargo test output, got:\n{proc.stdout}"
    )
    assert "1 passed; 0 failed" in proc.stdout, (
        f"Expected 1 passed in cargo test output, got:\n{proc.stdout}"
    )
