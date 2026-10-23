#!/usr/bin/env python3
"""Reproduction and regression functional test for WORM-LOAMY-CATERPILLAR-738.

Asserts that:
1. MotorTelemetry struct is defined with strongly typed fields:
   - speed: MotorSpeed
   - measured_rpm: u32
   - estimated_rpm: u32
   - current_ma: i32
   - modeled_peak_current_ma: i32
   - modeled_torque_unm: u32
2. Motor physical modeling functions calculate expected physical values:
   - calculate_estimated_rpm computes synchronous RPM from step interval
   - calculate_modeled_torque_unm computes electromagnetic torque using motor torque constant Kt
   - calculate_modeled_peak_current_ma computes ripple peak current from on-time and inductance
3. CBOR encoding/decoding and telemetry export pipeline handles MotorTelemetry correctly.
"""

from pathlib import Path
import subprocess


def test_worm_loamy_caterpillar_738_motor_telemetry():
    """Verify MotorTelemetry struct and physical modeling via model unit tests."""
    workspace_root = Path(__file__).resolve().parent.parent.parent

    cmd = [
        "cargo",
        "test",
        "--package",
        "model",
        "--test",
        "telemetry_tests",
        "test_motor_telemetry_modeling",
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
        f"cargo test model telemetry_tests failed with return code {proc.returncode}:\n"
        f"STDOUT:\n{proc.stdout}\nSTDERR:\n{proc.stderr}"
    )
    assert "test test_motor_telemetry_modeling ... ok" in proc.stdout
