"""Unit tests for motor ramp stress test utility."""

import sys
from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest

_tools_dir = Path(__file__).resolve().parent.parent
if str(_tools_dir) not in sys.path:
    sys.path.insert(0, str(_tools_dir))

from motor_stress import MotorStressReport, MotorStressRunner, MotorStressStep


def test_motor_stress_report_summary():
    """Verify that MotorStressReport produces accurate text formatting."""
    step = MotorStressStep(
        step_index=1,
        speed=1,
        current_ma=75,
        timestamp_utc="2026-10-23T17:00:00Z",
        diag_fault=False,
        status="OK",
    )
    report = MotorStressReport(
        start_speed=1,
        end_speed=10,
        duration_seconds=10.0,
        total_steps=10,
        passed=True,
        diag_tripped=False,
        max_current_ma=150,
        min_current_ma=70,
        average_current_ma=110.0,
        steps=[step],
    )
    summary = report.format_summary()
    assert "MOTOR RAMP STRESS TEST REPORT: PASS" in summary
    assert "1% -> 10%" in summary
    assert "Peak Current      : 150 mA" in summary
    assert "Average Current   : 110.0 mA" in summary
    assert "DIAG Fault Active : NO (HEALTHY)" in summary

    # Failure report
    fail_report = MotorStressReport(
        start_speed=1,
        end_speed=100,
        duration_seconds=100.0,
        total_steps=100,
        passed=False,
        diag_tripped=True,
        max_current_ma=1350,
        min_current_ma=70,
        average_current_ma=600.0,
        error_message="DIAG pin asserted at speed 50",
    )
    fail_summary = fail_report.format_summary()
    assert "MOTOR RAMP STRESS TEST REPORT: FAIL" in fail_summary
    assert "DIAG Fault Active : YES (TRIPPED)" in fail_summary
    assert "DIAG pin asserted at speed 50" in fail_summary


def test_motor_stress_runner_read_current():
    """Verify regex parsing of motor current from daemon CLI output."""
    runner = MotorStressRunner(host="127.0.0.1", port=8899)
    with patch.object(runner, "execute_cli") as mock_exec, patch.object(runner, "get_cli_output") as mock_cli_out:
        mock_cli_out.return_value = [
            {"text": "shell> motor current"},
            {"text": "Motor current: 345 mA"},
            {"text": "shell> "},
        ]
        curr = runner.read_current_ma()
        mock_exec.assert_called_with("motor current")
        assert curr == 345


def test_motor_stress_runner_check_diag_trip():
    """Verify detection of DIAG fault and brownout messages in system logs."""
    runner = MotorStressRunner(host="127.0.0.1", port=8899)

    # 1. Healthy logs
    with patch.object(runner, "get_recent_logs") as mock_logs:
        mock_logs.return_value = [
            {"timestamp": "2026-10-23T17:00:00Z", "message": "Motor starting..."},
            {"timestamp": "2026-10-23T17:00:01Z", "message": "Motor current: 100 mA"},
        ]
        tripped, fault_msg = runner.check_diag_trip()
        assert not tripped
        assert fault_msg is None

    # 2. DIAG assertion fault
    with patch.object(runner, "get_recent_logs") as mock_logs:
        mock_logs.return_value = [
            {"timestamp": "2026-10-23T17:00:00Z", "message": "Motor starting..."},
            {
                "timestamp": "2026-10-23T17:00:02Z",
                "message": "TMC6300 PIO DIAG pin asserted: driver fault detected",
            },
        ]
        tripped, fault_msg = runner.check_diag_trip()
        assert tripped
        assert "driver fault detected" in fault_msg

    # 3. DAP disconnect / brownout fault
    with patch.object(runner, "get_recent_logs") as mock_logs:
        mock_logs.return_value = [
            {
                "timestamp": "2026-10-23T17:00:03Z",
                "message": 'Connection lost: defmt read error: "Arm(Dap(NoAcknowledge))". Reconnecting...',
            }
        ]
        tripped, fault_msg = runner.check_diag_trip()
        assert tripped
        assert "DAP disconnect" in fault_msg


def test_motor_stress_runner_run_ramp_happy_path(tmp_path: Path):
    """Verify full ramp execution when no faults occur."""
    runner = MotorStressRunner(host="127.0.0.1", port=8899)
    out_file = tmp_path / "motor_stress.json"

    with patch.object(runner, "execute_cli") as mock_exec, patch.object(
        runner, "read_current_ma", return_value=120
    ), patch.object(runner, "check_diag_trip", return_value=(False, None)):
        report = runner.run_ramp(
            start_speed=1,
            end_speed=5,
            duration_seconds=0.01,
            output_file=out_file,
            verbose=False,
        )

        assert report.passed
        assert not report.diag_tripped
        assert report.total_steps == 5
        assert len(report.steps) == 5
        assert report.max_current_ma == 120
        assert report.min_current_ma == 120
        assert report.average_current_ma == 120.0
        assert out_file.exists()

        # Initial stop + 5 speed steps + final stop
        assert mock_exec.call_count >= 7
        assert mock_exec.call_args_list[0][0][0] == "motor stop"
        assert mock_exec.call_args_list[-1][0][0] == "motor stop"


def test_motor_stress_runner_run_ramp_diag_trip_aborts():
    """Verify ramp immediately halts motor and aborts if DIAG trips."""
    runner = MotorStressRunner(host="127.0.0.1", port=8899)

    with patch.object(runner, "execute_cli") as mock_exec, patch.object(
        runner, "read_current_ma", return_value=1450
    ), patch.object(
        runner,
        "check_diag_trip",
        side_effect=[
            (False, None),  # Speed 1
            (True, "TMC6300 PIO DIAG pin asserted: driver fault detected"),  # Speed 2
        ],
    ):
        report = runner.run_ramp(
            start_speed=1,
            end_speed=10,
            duration_seconds=0.01,
            verbose=False,
        )

        assert not report.passed
        assert report.diag_tripped
        assert "DIAG fault tripped at speed 2%" in report.error_message
        assert len(report.steps) == 2
        # Final emergency stop must be called
        assert mock_exec.call_args_list[-1][0][0] == "motor stop"
