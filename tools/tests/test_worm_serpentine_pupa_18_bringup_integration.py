"""Regression test for WORM-SERPENTINE-PUPA-18: Bringup Integration with Host Daemon.

Tests:
1. Parsing Bringup YAML schema (cat_detector_bringup.yaml, carrier_board_bringup.yaml).
2. BringupManager script discovery, step execution, regex evaluation, and markdown reporting.
3. Host Daemon REST API endpoints (/api/bringup/scripts, /api/bringup/load, /api/bringup/status, /api/bringup/step, /api/bringup/respond).
4. CLI subcommand 'bringup' in tools/host.py and host_daemon/cli.py.
5. Workstation template host_daemon.html.j2 support for BRINGUP mode, script selector, prompt cards, and CLI output.
"""

from io import StringIO
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import MagicMock, patch

_repo_root = Path(__file__).resolve().parent.parent.parent
_tools_dir = _repo_root / "tools"
_dashboard_dir = _tools_dir / "dashboard"
for p in (str(_repo_root), str(_tools_dir), str(_dashboard_dir)):
    if p not in sys.path:
        sys.path.insert(0, p)

from host_daemon.model.bringup import (
    BringupConfig,
    BringupStep,
    discover_bringup_scripts,
    load_bringup_yaml,
)
from host_daemon.provider.bringup.bringup_manager import BringupManager
from host_daemon.cli import parse_arguments
from host_daemon.provider.daemon.runner import DaemonRunner
from host_daemon.provider.daemon.server import HostDaemonRequestHandler, HostDaemonServer


class TestBringupIntegration(unittest.TestCase):
    """Test bringup script integration with host and workstation."""

    def setUp(self):
        self.repo_root = Path(__file__).resolve().parent.parent.parent
        self.cat_detector_yaml = self.repo_root / "app" / "cat_detector_bringup.yaml"
        self.carrier_board_yaml = self.repo_root / "app" / "carrier_board_bringup.yaml"

    def test_yaml_schema_parsing(self):
        """Verify YAML schema parsing for both cat_detector and carrier_board bringup files."""
        self.assertTrue(self.cat_detector_yaml.exists(), f"Missing {self.cat_detector_yaml}")
        self.assertTrue(self.carrier_board_yaml.exists(), f"Missing {self.carrier_board_yaml}")

        cfg1 = load_bringup_yaml(self.cat_detector_yaml)
        self.assertEqual(cfg1.project_name, "Cat Detector")
        self.assertEqual(cfg1.device_chip, "RP2040")
        self.assertGreater(len(cfg1.steps), 0)
        first_step = cfg1.steps[0]
        self.assertEqual(first_step.id, "1")
        self.assertEqual(first_step.type, "shell_command")
        self.assertEqual(first_step.command, "fs format")
        self.assertTrue(first_step.reboot)

        cfg2 = load_bringup_yaml(self.carrier_board_yaml)
        self.assertEqual(cfg2.project_name, "Carrier Board 2.0")
        self.assertEqual(cfg2.device_chip, "MCXN947")
        self.assertGreater(len(cfg2.steps), 0)
        step_hw = cfg2.steps[0]
        self.assertEqual(step_hw.type, "hardware_check")
        self.assertIn("TP1", step_hw.test_points)

    def test_discover_bringup_scripts(self):
        """Verify discovering available bringup scripts in repo."""
        scripts = discover_bringup_scripts(self.repo_root)
        self.assertIsInstance(scripts, list)
        self.assertGreaterEqual(len(scripts), 2)
        paths = [s["path"] for s in scripts]
        self.assertTrue(any("cat_detector_bringup.yaml" in p for p in paths))
        self.assertTrue(any("carrier_board_bringup.yaml" in p for p in paths))

    def test_bringup_manager_execution_and_report(self):
        """Verify BringupManager step execution, regex checking, and report generation."""
        mock_runner = MagicMock(spec=DaemonRunner)
        mock_runner.repo_root = self.repo_root
        mock_runner.execute_cli.return_value = "Direct battery reading: 3950 mV, 85% state of charge\nCommand succeeded."

        manager = BringupManager(runner=mock_runner, repo_root=self.repo_root)
        cfg = manager.load_script(self.cat_detector_yaml)
        self.assertIsNotNone(cfg)
        self.assertEqual(manager.status, "idle")

        # Set step to step 2 (battery status)
        manager.current_step_idx = 1
        step_result = manager.step()
        self.assertIn(step_result["status"], ("passed", "waiting_prompt"))

        # Test responding to a prompt / manual pass
        resp = manager.respond(action="pass", notes="Verified OK")
        self.assertTrue(resp["success"])
        self.assertEqual(len(manager.results), 1)
        self.assertEqual(manager.results[0]["status"], "passed")
        self.assertEqual(manager.results[0]["notes"], "Verified OK")

        # Test markdown report generation
        with tempfile.NamedTemporaryFile(suffix=".md", delete=False) as tf:
            report_path = Path(tf.name)
        try:
            report_text = manager.generate_report(output_path=report_path)
            self.assertTrue(report_path.exists())
            self.assertIn("# Bringup Verification Report: Cat Detector", report_text)
            self.assertIn("Verified OK", report_text)
        finally:
            if report_path.exists():
                report_path.unlink()

    def test_cli_subcommand_parsing(self):
        """Verify 'bringup' subcommand parsing in tools/host.py CLI."""
        args = parse_arguments(["bringup", "--script", "app/cat_detector_bringup.yaml", "--status"])
        self.assertEqual(args.subcommand, "bringup")
        self.assertEqual(args.script, "app/cat_detector_bringup.yaml")
        self.assertTrue(args.status)

    def test_server_bringup_endpoints(self):
        """Verify REST API endpoints for bringup script management."""
        mock_runner = MagicMock(spec=DaemonRunner)
        mock_runner.repo_root = self.repo_root
        mock_runner.get_status.return_value = {"mode": "BRINGUP"}
        mock_runner.logs = []
        mock_runner.cli_outputs = []
        mock_runner.fs_outputs = []
        mock_runner._lock = MagicMock()
        mock_runner._lock.__enter__.return_value = None
        mock_runner._lock.__exit__.return_value = None

        server = HostDaemonServer("127.0.0.1", 0, mock_runner)
        server.server_close()  # Just testing handler dispatch logic

        # Test GET /api/bringup/scripts
        handler = HostDaemonRequestHandler.__new__(HostDaemonRequestHandler)
        handler.server = server
        handler.path = "/api/bringup/scripts"
        handler.requestline = "GET /api/bringup/scripts HTTP/1.1"
        handler.headers = {}
        handler.wfile = MagicMock()

        sent_payloads = []

        def fake_send_json(data, status=200):
            sent_payloads.append((status, data))

        handler._send_json = fake_send_json
        handler.do_GET()

        self.assertTrue(sent_payloads)
        status, scripts = sent_payloads[-1]
        self.assertEqual(status, 200)
        self.assertTrue(any("cat_detector" in str(s) for s in scripts))

    def test_host_daemon_html_bringup_view(self):
        """Verify template includes Bringup mode, script dropdown, interactive prompt card, and CLI view."""
        template_file = self.repo_root / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
        content = template_file.read_text(encoding="utf-8")

        # Must have BRINGUP mode option
        self.assertIn('value="BRINGUP"', content)
        # Must have Bringup toolbar / script selector
        self.assertIn('id="bringupToolbar"', content)
        self.assertIn('id="bringupScriptSelect"', content)
        # Must have Bringup view container
        self.assertIn('id="bringupView"', content)
        # Must have interactive prompt container
        self.assertIn('id="bringupPromptCard"', content)
        # Must have step list container
        self.assertIn('id="bringupStepList"', content)


    def test_bringup_step_types(self):
        """Verify hardware_check and host_command step execution in BringupManager."""
        mock_runner = MagicMock(spec=DaemonRunner)
        mock_runner.repo_root = self.repo_root

        manager = BringupManager(runner=mock_runner, repo_root=self.repo_root)
        cfg = manager.load_script(self.carrier_board_yaml)
        self.assertEqual(cfg.project_name, "Carrier Board 2.0")

        # Step 1 is hardware_check
        res_hw = manager.step()
        self.assertEqual(res_hw["status"], "waiting_prompt")
        self.assertIn("TP1", res_hw["prompt"]["test_points"])
        resp1 = manager.respond("pass", notes="Voltages nominal")
        self.assertTrue(resp1["success"])

        # Reset checklist
        res_rst = manager.reset()
        self.assertTrue(res_rst["success"])
        self.assertEqual(manager.current_step_idx, 0)

    def test_server_post_bringup_endpoints(self):
        """Verify POST endpoints for bringup load, step, respond, reset, report, and mode change."""
        mock_runner = MagicMock(spec=DaemonRunner)
        mock_runner.repo_root = self.repo_root
        mock_runner.bringup_manager = BringupManager(runner=mock_runner, repo_root=self.repo_root)
        mock_runner.get_status.return_value = {"mode": "BRINGUP"}

        server = HostDaemonServer("127.0.0.1", 0, mock_runner)
        server.server_close()

        handler = HostDaemonRequestHandler.__new__(HostDaemonRequestHandler)
        handler.server = server
        handler.headers = {}
        handler.wfile = MagicMock()

        sent_payloads = []

        def fake_send_json(data, status=200):
            sent_payloads.append((status, data))

        handler._send_json = fake_send_json

        from io import BytesIO

        # Test POST /api/mode with BRINGUP
        handler.path = "/api/mode"
        handler.requestline = "POST /api/mode HTTP/1.1"
        handler.rfile = BytesIO(json.dumps({"mode": "BRINGUP"}).encode("utf-8"))
        handler.headers["Content-Length"] = str(len(json.dumps({"mode": "BRINGUP"}).encode("utf-8")))
        handler.do_POST()
        self.assertEqual(sent_payloads[-1][0], 200)
        self.assertEqual(sent_payloads[-1][1]["mode"], "BRINGUP")

        # Test POST /api/bringup/load
        handler.path = "/api/bringup/load"
        handler.requestline = "POST /api/bringup/load HTTP/1.1"
        load_body = json.dumps({"script_path": "app/cat_detector_bringup.yaml"}).encode("utf-8")
        handler.rfile = BytesIO(load_body)
        handler.headers["Content-Length"] = str(len(load_body))
        handler.do_POST()
        self.assertEqual(sent_payloads[-1][0], 200)
        self.assertTrue(sent_payloads[-1][1]["success"])
        self.assertEqual(sent_payloads[-1][1]["config"]["project_name"], "Cat Detector")

        # Test POST /api/bringup/step
        handler.path = "/api/bringup/step"
        handler.requestline = "POST /api/bringup/step HTTP/1.1"
        handler.rfile = BytesIO(b"{}")
        handler.headers["Content-Length"] = "2"
        handler.do_POST()
        self.assertEqual(sent_payloads[-1][0], 200)
        self.assertEqual(sent_payloads[-1][1]["status"], "waiting_prompt")

        # Test POST /api/bringup/respond
        handler.path = "/api/bringup/respond"
        handler.requestline = "POST /api/bringup/respond HTTP/1.1"
        resp_body = json.dumps({"action": "pass", "notes": "Format passed"}).encode("utf-8")
        handler.rfile = BytesIO(resp_body)
        handler.headers["Content-Length"] = str(len(resp_body))
        handler.do_POST()
        self.assertEqual(sent_payloads[-1][0], 200)
        self.assertTrue(sent_payloads[-1][1]["success"])

        # Test POST /api/bringup/reset
        handler.path = "/api/bringup/reset"
        handler.requestline = "POST /api/bringup/reset HTTP/1.1"
        handler.rfile = BytesIO(b"{}")
        handler.headers["Content-Length"] = "2"
        handler.do_POST()
        self.assertEqual(sent_payloads[-1][0], 200)
        self.assertTrue(sent_payloads[-1][1]["success"])

        # Test POST /api/bringup/report
        handler.path = "/api/bringup/report"
        handler.requestline = "POST /api/bringup/report HTTP/1.1"
        with tempfile.NamedTemporaryFile(suffix=".md") as tf:
            report_body = json.dumps({"output_path": tf.name}).encode("utf-8")
            handler.rfile = BytesIO(report_body)
            handler.headers["Content-Length"] = str(len(report_body))
            handler.do_POST()
            self.assertEqual(sent_payloads[-1][0], 200)
            self.assertTrue(sent_payloads[-1][1]["success"])


if __name__ == "__main__":
    unittest.main()
