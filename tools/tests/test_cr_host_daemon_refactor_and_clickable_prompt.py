"""Regression unit test for code review findings a5bbb33b and cfc27a10.

Verifies:
1. a5bbb33b: host_daemon directory hierarchy mirrors tools/dashboard (model/, provider/, templates/, static/, cli.py).
2. a5bbb33b: Canonical imports work cleanly from model and provider packages without legacy backward-compatibility shims.
3. cfc27a10: format_clickable_choice formats interactive prompt choices with OSC 8 hyperlinks and ANSI colors.
4. cfc27a10: host_daemon.html.j2 parses command/action hyperlinks and provides handleCliCommandClick callback.
"""

from pathlib import Path
import re
import sys

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))
_tools_dir = REPO_ROOT / "tools"
if str(_tools_dir) not in sys.path:
    sys.path.insert(0, str(_tools_dir))

from host_daemon.cli import format_clickable_choice
from host_daemon.model.bringup import (
    BringupConfig,
    BringupStep,
    BringupStepStatus,
    BringupStepType,
    discover_bringup_scripts,
    load_bringup_yaml,
)
from host_daemon.provider.bringup.bringup_manager import BringupManager
from host_daemon.provider.daemon.runner import DaemonRunner
from host_daemon.provider.daemon.server import HostDaemonRequestHandler, HostDaemonServer


def test_cr_a5bbb33b_host_daemon_library_structure():
    """Verify host_daemon follows the modular dashboard hierarchy (model/, provider/, templates/, cli.py)."""
    hd_dir = REPO_ROOT / "tools" / "host_daemon"
    assert (hd_dir / "model").is_dir(), "tools/host_daemon/model directory must exist"
    assert (hd_dir / "model" / "bringup.py").is_file(), "tools/host_daemon/model/bringup.py must exist"
    assert (hd_dir / "provider").is_dir(), "tools/host_daemon/provider directory must exist"
    assert (hd_dir / "provider" / "bringup" / "bringup_manager.py").is_file(), "bringup_manager.py must exist in provider/bringup/"
    assert (hd_dir / "provider" / "daemon" / "runner.py").is_file(), "runner.py must exist in provider/daemon/"
    assert (hd_dir / "provider" / "daemon" / "server.py").is_file(), "server.py must exist in provider/daemon/"
    assert (hd_dir / "templates" / "host_daemon.html.j2").is_file(), "templates/host_daemon.html.j2 must exist"
    assert (hd_dir / "cli.py").is_file(), "cli.py must exist at host_daemon root"

    # Enforce Rule 3: No legacy flat shims remain
    assert not (hd_dir / "bringup.py").exists(), "Legacy tools/host_daemon/bringup.py must not exist"
    assert not (hd_dir / "runner.py").exists(), "Legacy tools/host_daemon/runner.py must not exist"
    assert not (hd_dir / "server.py").exists(), "Legacy tools/host_daemon/server.py must not exist"


def test_cr_a5bbb33b_model_and_provider_classes():
    """Verify model and provider classes can be instantiated and operated."""
    step = BringupStep(
        id="1",
        name="Check LED",
        type=BringupStepType.HARDWARE_CHECK,
        procedure="Observe green LED",
        expected="LED lit",
    )
    assert step.id == "1"
    assert step.status == BringupStepStatus.PENDING

    cfg = BringupConfig(
        project_name="Test Project",
        device_chip="RP2040",
        flash_addr="0x101C0000",
        flash_size=262144,
        steps=[step],
    )
    assert cfg.project_name == "Test Project"
    assert len(cfg.steps) == 1

    runner = DaemonRunner(repo_root=REPO_ROOT)
    assert isinstance(runner.bringup_manager, BringupManager)
    server = HostDaemonServer("127.0.0.1", 0, runner)
    assert server is not None
    server.server_close()


def test_cr_cfc27a10_format_clickable_choices():
    """Verify format_clickable_choice formats options with OSC 8 hyperlinks and ANSI colors."""
    p_pass = format_clickable_choice("Pass", "P", "pass", "\033[1;32m")
    assert "\033]8;;command:pass\033\\" in p_pass
    assert "[P]ass" in p_pass
    assert "\033]8;;\033\\" in p_pass
    assert "\033[1;32m" in p_pass

    p_fail = format_clickable_choice("Fail", "F", "fail", "\033[1;31m")
    assert "\033]8;;command:fail\033\\" in p_fail
    assert "[F]ail" in p_fail
    assert "\033]8;;\033\\" in p_fail
    assert "\033[1;31m" in p_fail

    p_skip = format_clickable_choice("Skip", "S", "skip", "\033[1;33m")
    assert "\033]8;;command:skip\033\\" in p_skip
    assert "[S]kip" in p_skip
    assert "\033]8;;\033\\" in p_skip
    assert "\033[1;33m" in p_skip


def test_cr_cfc27a10_template_command_link_handling():
    """Verify host_daemon.html.j2 implements handleCliCommandClick and handles command:/action: URLs."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    content = template_path.read_text(encoding="utf-8")

    assert "handleCliCommandClick" in content
    assert 'currentUrl.startsWith("command:")' in content or "currentUrl.startsWith('command:')" in content
    assert "term-cmd-link" in content
