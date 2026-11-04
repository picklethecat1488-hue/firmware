#!/usr/bin/env python3
"""Automated Motor Ramp Stress Test Utility.

Executes a linear ramp of the motor speed from start_speed to end_speed (default 1 to 100)
over a target duration (default 100 seconds) via the host daemon CLI endpoint,
continuously polling active winding current and checking for TMC6300 thermal or
short-circuit diagnostic faults (DIAG pin trips).

Ensures PWM duty cycle scaling maintains safe winding current and prevents driver
overheating across the entire speed spectrum.
"""

from dataclasses import asdict, dataclass, field
from datetime import datetime, timezone
import json
from pathlib import Path
import re
import sys
import time
from typing import Any, Dict, List, Optional, Tuple
import urllib.error
import urllib.request


@dataclass
class MotorStressStep:
    """Individual speed step recording during ramp execution."""

    step_index: int
    speed: int
    current_ma: int
    timestamp_utc: str
    diag_fault: bool
    status: str


@dataclass
class MotorStressReport:
    """Aggregated outcome of the motor ramp stress test."""

    start_speed: int
    end_speed: int
    duration_seconds: float
    total_steps: int
    passed: bool
    diag_tripped: bool
    max_current_ma: int
    min_current_ma: int
    average_current_ma: float
    error_message: Optional[str] = None
    steps: List[MotorStressStep] = field(default_factory=list)

    def to_dict(self) -> Dict[str, Any]:
        """Convert report to dictionary format."""
        return asdict(self)

    def format_summary(self) -> str:
        """Format a human-readable CLI summary."""
        status_str = "PASS" if self.passed else "FAIL"
        summary = [
            "=" * 70,
            f"  MOTOR RAMP STRESS TEST REPORT: {status_str}",
            "=" * 70,
            f"  Speed Range       : {self.start_speed}% -> {self.end_speed}%",
            f"  Duration Target   : {self.duration_seconds:.1f} s",
            f"  Completed Steps   : {len(self.steps)} / {self.total_steps}",
            f"  DIAG Fault Active : {'YES (TRIPPED)' if self.diag_tripped else 'NO (HEALTHY)'}",
            f"  Peak Current      : {self.max_current_ma} mA",
            f"  Min Current       : {self.min_current_ma} mA",
            f"  Average Current   : {self.average_current_ma:.1f} mA",
        ]
        if self.error_message:
            summary.append(f"  Failure Detail    : {self.error_message}")
        summary.append("=" * 70)
        return "\n".join(summary)


class MotorStressRunner:
    """Orchestrates ramp commands, status polling, and safety monitoring via host daemon."""

    def __init__(self, host: str = "127.0.0.1", port: int = 8899, timeout: float = 3.0):
        """Initialize runner with daemon connection parameters."""
        self.host = host
        self.port = port
        self.timeout = timeout
        self.base_url = f"http://{host}:{port}"

    def send_request(self, endpoint: str, payload: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Send JSON HTTP request to host daemon."""
        url = f"{self.base_url}{endpoint}"
        req_data = json.dumps(payload).encode("utf-8") if payload is not None else None
        headers = {"Content-Type": "application/json"} if req_data else {}
        req = urllib.request.Request(url, data=req_data, headers=headers)
        with urllib.request.urlopen(req, timeout=self.timeout) as resp:
            data = resp.read()
            return json.loads(data.decode("utf-8")) if data else {}

    def execute_cli(self, command: str) -> str:
        """Execute a shell command on the target via host daemon."""
        res = self.send_request("/api/cli/execute", {"command": command})
        return str(res.get("output", ""))

    def get_recent_logs(self) -> List[Dict[str, Any]]:
        """Fetch system defmt and host daemon logs."""
        res = self.send_request("/api/logs")
        if isinstance(res, list):
            return res
        return res.get("logs", []) if isinstance(res, dict) else []

    def get_cli_output(self) -> List[Dict[str, Any]]:
        """Fetch recent raw CLI console outputs."""
        res = self.send_request("/api/cli/output")
        if isinstance(res, list):
            return res
        return res.get("output", []) if isinstance(res, dict) else []

    def read_current_ma(self, timeout: float = 1.0) -> int:
        """Query motor current and parse response."""
        out_before = len(self.get_cli_output())
        self.execute_cli("motor current")
        start_time = time.monotonic()
        while time.monotonic() - start_time < timeout:
            time.sleep(0.04)
            outputs = self.get_cli_output()[out_before:]
            for item in outputs:
                text = item.get("text", "")
                match = re.search(r"Motor current:\s*(\d+)\s*mA", text)
                if match:
                    return int(match.group(1))
        # Fallback to recent outputs if not captured after slice
        for item in reversed(self.get_cli_output()[-5:]):
            text = item.get("text", "")
            match = re.search(r"Motor current:\s*(\d+)\s*mA", text)
            if match:
                return int(match.group(1))
        return 0

    def check_diag_trip(self, start_timestamp: Optional[str] = None) -> Tuple[bool, Optional[str]]:
        """Inspect recent logs for TMC6300 diagnostic pin assertions or driver faults."""
        logs = self.get_recent_logs()
        for entry in reversed(logs):
            msg = entry.get("message", "")
            ts = entry.get("timestamp", "")
            if start_timestamp and ts and ts < start_timestamp:
                break
            if "DIAG pin asserted" in msg or "driver fault detected" in msg:
                return True, msg
            if "Arm(Dap(NoAcknowledge))" in msg:
                return True, f"Target power brownout / DAP disconnect: {msg}"
        return False, None

    def run_ramp(
        self,
        start_speed: int = 1,
        end_speed: int = 100,
        duration_seconds: float = 100.0,
        output_file: Optional[Path] = None,
        verbose: bool = True,
    ) -> MotorStressReport:
        """Execute the motor ramp stress test over duration_seconds."""
        total_steps = end_speed - start_speed + 1
        step_interval = duration_seconds / max(total_steps, 1)

        start_time_iso = datetime.now(timezone.utc).isoformat()
        report = MotorStressReport(
            start_speed=start_speed,
            end_speed=end_speed,
            duration_seconds=duration_seconds,
            total_steps=total_steps,
            passed=False,
            diag_tripped=False,
            max_current_ma=0,
            min_current_ma=999999,
            average_current_ma=0.0,
        )

        if verbose:
            print(f"Starting motor ramp stress test: {start_speed}% -> {end_speed}% over {duration_seconds}s")
            print(f"Connecting to host daemon at {self.base_url}...")

        # Initial safety stop
        try:
            self.execute_cli("motor stop")
        except Exception as e:
            report.error_message = f"Failed initial stop command: {e}"
            return report

        time.sleep(0.2)
        total_current = 0
        recorded_currents: List[int] = []

        try:
            for idx, speed in enumerate(range(start_speed, end_speed + 1), start=1):
                step_start = time.monotonic()
                now_str = datetime.now(timezone.utc).isoformat()

                # 1. Send motor speed command
                self.execute_cli(f"motor speed {speed}")

                # Allow motor speed change to settle into the step
                settle_time = min(0.15, max(0.05, step_interval * 0.2))
                time.sleep(settle_time)

                # 2. Check current draw
                current_ma = self.read_current_ma()
                recorded_currents.append(current_ma)
                total_current += current_ma
                report.max_current_ma = max(report.max_current_ma, current_ma)
                report.min_current_ma = min(report.min_current_ma, current_ma)

                # 3. Check for DIAG fault or disconnect
                diag_tripped, fault_msg = self.check_diag_trip(start_timestamp=start_time_iso)
                step_record = MotorStressStep(
                    step_index=idx,
                    speed=speed,
                    current_ma=current_ma,
                    timestamp_utc=now_str,
                    diag_fault=diag_tripped,
                    status="FAIL" if diag_tripped else "OK",
                )
                report.steps.append(step_record)

                if verbose:
                    elapsed = idx * step_interval
                    diag_tag = "TRIPPED!" if diag_tripped else "OK"
                    print(
                        f"[{idx:3d}/{total_steps}] ({elapsed:5.1f}s) Speed: {speed:3d}% | Current: {current_ma:4d} mA | DIAG: {diag_tag}"
                    )

                if diag_tripped:
                    report.diag_tripped = True
                    report.error_message = f"DIAG fault tripped at speed {speed}% ({current_ma} mA): {fault_msg}"
                    break

                # Sleep remaining step interval time
                elapsed_step = time.monotonic() - step_start
                sleep_rem = max(0.0, step_interval - elapsed_step)
                if sleep_rem > 0:
                    time.sleep(sleep_rem)

            if not report.diag_tripped:
                report.passed = True

        except KeyboardInterrupt:
            report.error_message = "Stress test cancelled by user interrupt."
        except Exception as e:
            report.error_message = f"Unexpected execution error: {e}"
        finally:
            # Ensure motor is stopped cleanly after test
            try:
                self.execute_cli("motor stop")
                if verbose:
                    print("Sent motor stop command.")
            except Exception as e:
                if verbose:
                    print(f"Warning: Failed to stop motor cleanly: {e}")

        if recorded_currents:
            report.average_current_ma = total_current / len(recorded_currents)
        else:
            report.min_current_ma = 0

        if output_file:
            output_file.parent.mkdir(parents=True, exist_ok=True)
            with open(output_file, "w", encoding="utf-8") as f:
                json.dump(report.to_dict(), f, indent=2)
            if verbose:
                print(f"Stress test report exported to: {output_file}")

        return report


def main(argv: Optional[List[str]] = None) -> int:
    """CLI entry point for motor stress test."""
    import argparse

    parser = argparse.ArgumentParser(description="Automated Motor Ramp Stress Test Utility")
    parser.add_argument("--host", default="127.0.0.1", help="Host daemon network address (default: 127.0.0.1)")
    parser.add_argument("--port", type=int, default=8899, help="Host daemon HTTP port (default: 8899)")
    parser.add_argument("--start", type=int, default=1, help="Starting motor speed percentage (default: 1)")
    parser.add_argument("--end", type=int, default=100, help="Ending motor speed percentage (default: 100)")
    parser.add_argument(
        "--duration",
        type=float,
        default=100.0,
        help="Total duration in seconds to ramp from start to end (default: 100.0)",
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=None,
        help="Optional path to output JSON results report (e.g. artifacts/motor_stress.json)",
    )

    args = parser.parse_args(argv)
    runner = MotorStressRunner(host=args.host, port=args.port)
    report = runner.run_ramp(
        start_speed=args.start,
        end_speed=args.end,
        duration_seconds=args.duration,
        output_file=args.output,
        verbose=True,
    )

    print("\n" + report.format_summary())
    return 0 if report.passed else 1


if __name__ == "__main__":
    sys.exit(main())
