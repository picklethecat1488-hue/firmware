"""Unit tests for bin/anvil remote execution and synchronization runner."""

import os
from pathlib import Path
import stat
import subprocess


def get_repo_root() -> Path:
    """Return repository root path."""
    return Path(__file__).resolve().parents[2]


def test_anvil_runner_executable_and_syntax() -> None:
    """Verify bin/anvil exists, has executable permissions, and passes bash syntax validation."""
    repo_root = get_repo_root()
    script_path = repo_root / "bin" / "anvil"
    assert script_path.exists(), "bin/anvil runner script must exist"

    file_stat = script_path.stat()
    assert bool(file_stat.st_mode & stat.S_IXUSR), "bin/anvil must be executable"

    # Bash syntax check (bash -n bin/anvil)
    result = subprocess.run(["bash", "-n", str(script_path)], capture_output=True, text=True)
    assert result.returncode == 0, f"bin/anvil syntax error: {result.stderr}"


def test_anvil_runner_usage_without_args() -> None:
    """Verify bin/anvil without arguments prints usage instructions and exits with code 1."""
    repo_root = get_repo_root()
    script_path = repo_root / "bin" / "anvil"

    result = subprocess.run([str(script_path)], capture_output=True, text=True)
    assert result.returncode == 1
    assert "Usage: bin/anvil" in result.stdout or "Usage: bin/anvil" in result.stderr


def test_anvil_runner_pull_requires_path() -> None:
    """Verify bin/anvil pull requires a remote path argument."""
    repo_root = get_repo_root()
    script_path = repo_root / "bin" / "anvil"

    result = subprocess.run([str(script_path), "pull"], capture_output=True, text=True)
    assert result.returncode == 1
    assert "Usage: bin/anvil pull <remote_path> [local_dest]" in result.stdout or (
        "Usage: bin/anvil pull <remote_path> [local_dest]" in result.stderr
    )


def test_anvil_runner_sync_and_pytest_mocked(tmp_path: Path) -> None:
    """Verify bin/anvil sync and pytest dispatch appropriate rsync and ssh commands with proper exclusions."""
    repo_root = get_repo_root()
    script_path = repo_root / "bin" / "anvil"

    # Create mock ssh and rsync executables
    mock_bin_dir = tmp_path / "bin"
    mock_bin_dir.mkdir(parents=True, exist_ok=True)
    log_file = tmp_path / "calls.log"

    mock_ssh = mock_bin_dir / "ssh"
    mock_ssh.write_text(
        f"""#!/usr/bin/env bash
echo "MOCK_SSH: $@" >> "{log_file}"
# If checking existence of remote file, report success (0)
if [[ "$*" == *"-e"* ]] || [[ "$*" == *"-f"* ]]; then
    exit 0
fi
exit 0
""",
        encoding="utf-8",
    )
    mock_ssh.chmod(0o755)

    mock_rsync = mock_bin_dir / "rsync"
    mock_rsync.write_text(
        f"""#!/usr/bin/env bash
echo "MOCK_RSYNC: $@" >> "{log_file}"
exit 0
""",
        encoding="utf-8",
    )
    mock_rsync.chmod(0o755)

    env = os.environ.copy()
    env["PATH"] = f"{mock_bin_dir}:{env.get('PATH', '')}"
    env["REMOTE_HOST"] = "mock_anvil"

    # Test 1: bin/anvil sync
    res_sync = subprocess.run([str(script_path), "sync"], env=env, capture_output=True, text=True)
    assert res_sync.returncode == 0
    assert "Sync complete" in res_sync.stdout

    # Test 2: bin/anvil pytest
    res_pytest = subprocess.run(
        [str(script_path), "pytest", "-k", "test_mock"], env=env, capture_output=True, text=True
    )
    assert res_pytest.returncode == 0

    # Test 3: bin/anvil verify
    res_verify = subprocess.run([str(script_path), "verify"], env=env, capture_output=True, text=True)
    assert res_verify.returncode == 0

    # Test 4: bin/anvil stop
    res_stop = subprocess.run([str(script_path), "stop"], env=env, capture_output=True, text=True)
    assert res_stop.returncode == 0
    assert "Remote processes cleaned up" in res_stop.stdout

    # Validate calls logged
    calls = log_file.read_text(encoding="utf-8")
    assert "MOCK_RSYNC:" in calls
    assert "--exclude target" in calls
    assert "--exclude .git" in calls
    assert "MOCK_SSH:" in calls
    assert "pytest" in calls
    assert "verify.sh" in calls
    assert "pkill" in calls
