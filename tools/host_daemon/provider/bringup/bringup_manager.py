"""Bringup checklist execution lifecycle manager and reporting engine."""

from collections import deque
import datetime
from pathlib import Path
import subprocess
import threading
from typing import Any, Deque, Dict, List, Optional, Union

from host_daemon.model.bringup import (
    DEFAULT_HOST_COMMAND_TIMEOUT_SEC,
    BringupConfig,
    BringupStepStatus,
    BringupStepType,
    check_regex,
    load_bringup_yaml,
    parse_cargo_target,
)


class BringupManager:
    """Manages bringup script execution lifecycle, state transitions, and reporting."""

    def __init__(self, runner: Optional[Any] = None, repo_root: Optional[Path] = None) -> None:
        """Initialize BringupManager with reference to runner and repository root."""
        self.runner = runner
        self.repo_root = repo_root or (runner.repo_root if runner else Path.cwd())
        self.config: Optional[BringupConfig] = None
        self.current_step_idx: int = 0
        self.status: str = "idle"  # idle, running, waiting_prompt, complete, aborted
        self.active_prompt: Optional[Dict[str, Any]] = None
        self.results: List[Dict[str, Any]] = []
        self.logs: Deque[str] = deque(maxlen=2000)
        self._lock = threading.Lock()

    def log(self, message: str) -> None:
        """Record timestamped bringup message into log buffer."""
        timestamp = datetime.datetime.now().strftime("%H:%M:%S")
        line = f"[{timestamp}] [bringup] {message}"
        with self._lock:
            self.logs.append(line)
        if self.runner and hasattr(self.runner, "log"):
            self.runner.log(message, channel="bringup")

    def load_script(self, script_path_or_content: Union[str, Path]) -> BringupConfig:
        """Load, parse, and initialize bringup checklist script."""
        with self._lock:
            path = Path(script_path_or_content)
            if not path.is_absolute():
                path = self.repo_root / path

            self.config = load_bringup_yaml(path)
            self.current_step_idx = 0
            self.status = "idle"
            self.active_prompt = None
            self.results = []
            self.logs.clear()

        self.log(f"Loaded bringup checklist '{self.config.project_name}' ({len(self.config.steps)} steps)")
        return self.config

    def get_variables(self) -> Dict[str, str]:
        """Construct variable dictionary for host command interpolation."""
        if not self.config:
            return {}
        shell_elf = ""
        app_elf = ""
        if self.config.cargo_target:
            shell_elf = parse_cargo_target(self.config.cargo_target, default_bin="cat_detector_shell")
        if self.config.app_cargo_target:
            app_elf = parse_cargo_target(self.config.app_cargo_target, default_bin="cat_detector_app")

        return {
            "device_chip": self.config.device_chip,
            "flash_addr": self.config.flash_addr,
            "flash_size": str(self.config.flash_size),
            "shell_elf": shell_elf,
            "app_elf": app_elf,
        }

    def get_status(self) -> Dict[str, Any]:
        """Return comprehensive thread-safe status dictionary."""
        with self._lock:
            if not self.config:
                return {
                    "loaded": False,
                    "script_name": "",
                    "script_path": "",
                    "current_step_idx": 0,
                    "total_steps": 0,
                    "status": "unloaded",
                    "active_prompt": None,
                    "summary": {"total": 0, "passed": 0, "failed": 0, "skipped": 0, "pending": 0},
                    "steps": [],
                    "current_step": None,
                    "logs": list(self.logs),
                }

            steps_dicts = [step.to_dict() for step in self.config.steps]
            passed = sum(1 for s in self.config.steps if s.status == BringupStepStatus.PASSED)
            failed = sum(1 for s in self.config.steps if s.status == BringupStepStatus.FAILED)
            skipped = sum(1 for s in self.config.steps if s.status == BringupStepStatus.SKIPPED)
            pending = sum(1 for s in self.config.steps if s.status == BringupStepStatus.PENDING)

            curr = None
            if 0 <= self.current_step_idx < len(self.config.steps):
                curr = self.config.steps[self.current_step_idx].to_dict()

            return {
                "loaded": True,
                "script_name": self.config.project_name,
                "script_path": self.config.file_path or "",
                "current_step_idx": self.current_step_idx,
                "total_steps": len(self.config.steps),
                "status": self.status,
                "active_prompt": self.active_prompt,
                "summary": {
                    "total": len(self.config.steps),
                    "passed": passed,
                    "failed": failed,
                    "skipped": skipped,
                    "pending": pending,
                },
                "steps": steps_dicts,
                "current_step": curr,
                "logs": list(self.logs),
            }

    def reset(self) -> Dict[str, Any]:
        """Reset execution progress back to initial step."""
        with self._lock:
            if not self.config:
                return {"success": False, "error": "No script loaded"}
            for step in self.config.steps:
                step.status = BringupStepStatus.PENDING
                step.output_log = ""
                step.matched_line = ""
                step.notes = ""
                step.timestamp = None
            self.current_step_idx = 0
            self.status = "idle"
            self.active_prompt = None
            self.results = []

        self.log("Checklist reset to step 1")
        return {"success": True, "status": self.get_status()}

    def step(self) -> Dict[str, Any]:
        """Execute or initiate prompt for the current step."""
        with self._lock:
            if not self.config:
                return {"success": False, "error": "No bringup checklist loaded"}
            if self.current_step_idx >= len(self.config.steps):
                self.status = "complete"
                return {"success": True, "status": "complete", "message": "All steps completed"}

            step = self.config.steps[self.current_step_idx]
            step.status = BringupStepStatus.RUNNING
            self.status = "running"

        self.log(f"Executing Step {step.id}: {step.name} [{step.type.value}]")

        auto_pass: Optional[bool] = None
        output_log = ""
        matched_line = ""

        # Step Type Execution
        if step.type == BringupStepType.SHELL_COMMAND:
            cmd = step.command or ""
            if self.runner and hasattr(self.runner, "execute_cli"):
                output_log = self.runner.execute_cli(cmd)
            else:
                output_log = f"Simulated shell output for '{cmd}'"

            if step.reboot and self.runner and hasattr(self.runner, "reset_target"):
                self.log(f"Rebooting target device for step {step.id}...")
                self.runner.reset_target()

            if step.expected_regex:
                has_match, line = check_regex(output_log, step.expected_regex)
                if has_match:
                    auto_pass = True
                    matched_line = line
                    self.log(f"Pattern matched: {line}")
                else:
                    auto_pass = False
                    self.log(f"Expected pattern '{step.expected_regex}' not found in output")

        elif step.type == BringupStepType.HOST_COMMAND:
            cmd = step.command or ""
            variables = self.get_variables()
            try:
                formatted_cmd = cmd.format(**variables)
            except KeyError:
                formatted_cmd = cmd

            self.log(f"Running host command: {formatted_cmd}")
            try:
                proc = subprocess.run(
                    formatted_cmd,
                    shell=True,
                    cwd=self.repo_root,
                    capture_output=True,
                    text=True,
                    timeout=DEFAULT_HOST_COMMAND_TIMEOUT_SEC,
                    check=False,
                )
                output_log = proc.stdout + ("\n" + proc.stderr if proc.stderr else "")
                if proc.returncode == 0:
                    auto_pass = True
                    if step.expected_regex:
                        has_match, line = check_regex(output_log, step.expected_regex)
                        if has_match:
                            auto_pass = True
                            matched_line = line
                        else:
                            auto_pass = False
                else:
                    auto_pass = False
            except Exception as e:
                output_log = f"Host command execution failed: {e}"
                auto_pass = False

        step.output_log = output_log
        step.matched_line = matched_line

        # Configure Interactive Prompt State
        default_status = "passed" if auto_pass is True else ("failed" if auto_pass is False else "passed")
        prompt_info = {
            "step_id": step.id,
            "step_name": step.name,
            "step_type": step.type.value,
            "description": step.description or "",
            "procedure": step.procedure or "",
            "expected": step.expected or "",
            "test_points": step.test_points,
            "control_pins": step.control_pins,
            "output_log": output_log,
            "matched_line": matched_line,
            "auto_pass": auto_pass,
            "default_status": default_status,
        }

        with self._lock:
            self.active_prompt = prompt_info
            self.status = "waiting_prompt"

        return {
            "success": True,
            "status": "waiting_prompt",
            "prompt": prompt_info,
            "step": step.to_dict(),
        }

    def respond(self, action: str, notes: str = "", extra: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Record response to active prompt and advance checklist to subsequent step."""
        with self._lock:
            if not self.config:
                return {"success": False, "error": "No script loaded"}
            if self.current_step_idx >= len(self.config.steps):
                return {"success": False, "error": "All steps already completed"}

            step = self.config.steps[self.current_step_idx]
            action_norm = action.strip().lower()

            if action_norm in ("pass", "passed"):
                step_status = BringupStepStatus.PASSED
            elif action_norm in ("fail", "failed"):
                step_status = BringupStepStatus.FAILED
            elif action_norm in ("skip", "skipped"):
                step_status = BringupStepStatus.SKIPPED
            else:
                step_status = BringupStepStatus.PASSED

            now_str = datetime.datetime.now().isoformat()
            step.status = step_status
            step.notes = notes
            step.timestamp = now_str

            self.results.append({
                "id": step.id,
                "name": step.name,
                "type": step.type.value,
                "status": step.status.value,
                "output_log": step.output_log,
                "matched_line": step.matched_line,
                "notes": notes,
                "timestamp": now_str,
            })

            self.active_prompt = None
            self.current_step_idx += 1

            if self.current_step_idx >= len(self.config.steps):
                self.status = "complete"
            else:
                self.status = "idle"

        self.log(f"Step {step.id} marked {step.status.value.upper()}{' (' + notes + ')' if notes else ''}")
        return {
            "success": True,
            "status": self.status,
            "step": step.to_dict(),
            "next_step_idx": self.current_step_idx,
        }

    def generate_report(self, output_path: Optional[Union[str, Path]] = None) -> str:
        """Generate markdown summary report for completed bringup verification checklist."""
        with self._lock:
            if not self.config:
                return "# Bringup Verification Report\n\nNo checklist loaded."

            passed_count = sum(1 for r in self.results if r["status"] == "passed")
            failed_count = sum(1 for r in self.results if r["status"] == "failed")
            skipped_count = sum(1 for r in self.results if r["status"] == "skipped")
            total_completed = len(self.results)
            total_steps = len(self.config.steps)

            lines: List[str] = [
                f"# Bringup Verification Report: {self.config.project_name}",
                f"**Date:** {datetime.date.today().isoformat()}",
                f"**Device Chip:** {self.config.device_chip}",
                f"**Flash Address:** {self.config.flash_addr}",
                "",
                "## Summary",
                f"- **Total Steps Run:** {total_completed} / {total_steps}",
                f"- **Passed:** {passed_count}",
                f"- **Failed:** {failed_count}",
                f"- **Skipped:** {skipped_count}",
                "",
                "## Detailed Results",
            ]

            for r in self.results:
                emoji = "✅" if r["status"] == "passed" else ("❌" if r["status"] == "failed" else "⚠️")
                lines.append(f"\n### {emoji} Step {r['id']}: {r['name']}")
                lines.append(f"- **Type:** {r['type']}")
                lines.append(f"- **Status:** {r['status'].upper()}")
                lines.append(f"- **Time:** {r['timestamp']}")
                if r.get("matched_line"):
                    lines.append(f"- **Actual Matching Output Line:** `{r['matched_line']}`")
                if r.get("notes"):
                    lines.append(f"- **Notes:** {r['notes']}")
                if r.get("output_log"):
                    lines.append("- **Command Output:**")
                    lines.append("  ```text")
                    for oline in r["output_log"].splitlines():
                        lines.append(f"  {oline}")
                    lines.append("  ```")

            report_text = "\n".join(lines) + "\n"

        if output_path:
            out_p = Path(output_path)
            if not out_p.is_absolute():
                out_p = self.repo_root / out_p
            out_p.parent.mkdir(parents=True, exist_ok=True)
            out_p.write_text(report_text, encoding="utf-8")
            self.log(f"Generated bringup verification report at: {out_p}")

        return report_text
