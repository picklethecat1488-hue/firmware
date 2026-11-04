"""Regression test for WORM-SOILBOUND-VERMIS-341: CLI console is broken and not updating when buffer hits maxlen."""

from pathlib import Path
import sys

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
sys.path.insert(0, str(REPO_ROOT / "tools"))

from host_daemon.provider.daemon.runner import DaemonRunner


def test_worm_soilbound_vermis_341_monotonic_entry_ids_across_maxlen():
    """Verify that runner entries have monotonic IDs that continue incrementing when deque hits maxlen."""
    runner = DaemonRunner(repo_root=REPO_ROOT, package="cat_detector", bin_name="cat_detector_shell")

    # Add 5010 entries to cli_outputs (maxlen is 5000)
    for i in range(1, 5011):
        runner.log_cli(f"command {i}")

    assert len(runner.cli_outputs) == 5000
    assert runner.cli_outputs[0]["id"] == 11
    assert runner.cli_outputs[-1]["id"] == 5010

    # Ensure every entry has strictly increasing id
    for idx in range(len(runner.cli_outputs) - 1):
        assert runner.cli_outputs[idx]["id"] < runner.cli_outputs[idx + 1]["id"]

    # Verify logs monotonic IDs
    for i in range(1, 5011):
        runner.log(f"log {i}")
    assert len(runner.logs) == 5000
    assert runner.logs[0]["id"] == 11
    assert runner.logs[-1]["id"] == 5010

    # Verify fs_outputs monotonic IDs
    for i in range(1, 5011):
        runner.log_fs(f"fs {i}")
    assert len(runner.fs_outputs) == 5000
    assert runner.fs_outputs[0]["id"] == 11
    assert runner.fs_outputs[-1]["id"] == 5010


def test_worm_soilbound_vermis_341_template_handles_circular_buffer_updates():
    """Verify host_daemon.html.j2 tracks entry IDs rather than solely checking array length."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    assert template_path.exists(), "host_daemon.html.j2 not found"
    content = template_path.read_text(encoding="utf-8")

    # Verify tracking uses ID-based incremental update so that 5000-line full buffers continue updating
    assert "lastCliId" in content, "host_daemon.html.j2 must track lastCliId for circular buffer support"
    assert "lastLogId" in content, "host_daemon.html.j2 must track lastLogId for circular buffer support"
    assert "lastFsId" in content, "host_daemon.html.j2 must track lastFsId for circular buffer support"
