#!/usr/bin/env python3
"""Regression unit tests for project-filtered device suites and manual prompt streaming (`WORM-SERPENTINE-PUPA-18`)."""

import json
from pathlib import Path
import sys
import threading
import unittest
import urllib.request

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
TOOLS_DIR = REPO_ROOT / "tools"
DASHBOARD_DIR = TOOLS_DIR / "dashboard"

for p in (str(REPO_ROOT), str(TOOLS_DIR), str(DASHBOARD_DIR)):
    if p not in sys.path:
        sys.path.insert(0, p)

from device_tests.model import ScenarioTier, check_regex, discover_device_tests, infer_scenario_from_script
from host_daemon.cli import parse_arguments
from host_daemon.provider.daemon.runner import DaemonRunner
from host_daemon.provider.daemon.server import HostDaemonServer


class TestBringupIntegration(unittest.TestCase):
    """Test suite verifying `device_tests` discovery, `--project` filtering, and `/api/manual_prompt`."""

    def setUp(self) -> None:
        """Set up paths and ensure legacy YAML files do not exist."""
        self.repo_root = REPO_ROOT
        self.assertFalse((self.repo_root / "app" / "cat_detector_bringup.yaml").exists())
        self.assertFalse((self.repo_root / "app" / "carrier_board_bringup.yaml").exists())

    def test_project_discovery_across_modular_listings(self) -> None:
        """Verify `discover_device_tests` filters across multiple listings by `project` without `step_id`."""
        cat_tests = discover_device_tests(self.repo_root, project="cat_detector")
        carrier_tests = discover_device_tests(self.repo_root, project="carrier_board")

        self.assertGreaterEqual(len(cat_tests), 14)
        self.assertGreaterEqual(len(carrier_tests), 12)
        for idx, entry in enumerate(cat_tests, start=1):
            self.assertEqual(entry["order"], idx)
            self.assertEqual(entry["project"], "cat_detector")
            self.assertNotIn("step_id", entry)

        cat_files = {entry["file_path"] for entry in cat_tests}
        self.assertGreaterEqual(len(cat_files), 3)

        smoke_infer = infer_scenario_from_script(
            "tools/device_tests/suites/test_cat_detector_smoke.py",
            self.repo_root,
        )
        self.assertEqual(smoke_infer, ScenarioTier.SMOKE.value)

    def test_regex_checker(self) -> None:
        """Verify multiline regex evaluation helper."""
        sample_output = "Reading battery...\nVoltage: 4120 mV\nSoC: 94%\nDone."
        matched, line = check_regex(sample_output, r"Voltage:\s*\d+\s*mV")
        self.assertTrue(matched)
        self.assertEqual(line, "Voltage: 4120 mV")

    def test_cli_subcommand_parsing(self) -> None:
        """Verify `host.py test` parses `--project`, optional `--script`, and rejects removed `bringup`."""
        with self.assertRaises(SystemExit):
            parse_arguments(["bringup", "--list"])

        proj_args = parse_arguments(["test", "--project", "cat_detector", "--scenario", "bringup", "--list"])
        self.assertEqual(proj_args.subcommand, "test")
        self.assertEqual(proj_args.project, "cat_detector")
        self.assertEqual(proj_args.scenario, "bringup")
        self.assertTrue(proj_args.list_tests)

        script_args = parse_arguments(["test", "--script", "tools/device_tests/suites/test_cat_detector_smoke.py"])
        self.assertEqual(script_args.script, "tools/device_tests/suites/test_cat_detector_smoke.py")
        self.assertIsNone(script_args.scenario)

    def test_server_manual_prompt_endpoints(self) -> None:
        """Verify `GET`/`POST /api/manual_prompt` updates `DaemonRunner` and logs context on `host.py`."""
        runner = DaemonRunner(repo_root=self.repo_root)
        server = HostDaemonServer("127.0.0.1", 0, runner)
        port = server.server_address[1]
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()

        base_url = f"http://127.0.0.1:{port}"
        try:
            payload = {
                "active": True,
                "project": "cat_detector",
                "test_name": "test_motor_calibration_routine",
                "procedure": "Fill fountain reservoir above min line.",
                "prompt": "Confirm steady water flow without DIAG faults.",
            }
            req = urllib.request.Request(
                f"{base_url}/api/manual_prompt",
                data=json.dumps(payload).encode("utf-8"),
                headers={"Content-Type": "application/json"},
                method="POST",
            )
            with urllib.request.urlopen(req, timeout=5) as resp:
                self.assertEqual(resp.status, 200)
                post_res = json.loads(resp.read().decode("utf-8"))
                self.assertTrue(post_res["success"])
                self.assertEqual(post_res["manual_prompt"]["heartbeat_count"], 1)

            with urllib.request.urlopen(f"{base_url}/api/manual_prompt", timeout=5) as resp:
                state = json.loads(resp.read().decode("utf-8"))
                self.assertTrue(state["active"])
                self.assertEqual(state["test_name"], "test_motor_calibration_routine")
                self.assertIn("Fill fountain reservoir", state["procedure"])
        finally:
            server.shutdown()
            server.server_close()


if __name__ == "__main__":
    unittest.main()
