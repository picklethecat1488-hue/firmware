"""Unit tests for bin/anvil remote execution, synchronization runner, and Vast.ai lifecycle."""

import json
import os
from pathlib import Path
import stat
import subprocess
import sys
import time

REPO_ROOT = Path(__file__).resolve().parents[2]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.vast_manager import (
    build_bootstrap_script,
    load_env_file,
    prune_locks,
    record_lock,
    remove_lock,
    write_env_file,
)


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
    assert "provision" in result.stdout
    assert "bootstrap" in result.stdout
    assert "release" in result.stdout
    assert "auto-release" in result.stdout
    assert "status" in result.stdout


def test_anvil_runner_pull_requires_path() -> None:
    """Verify bin/anvil pull requires a remote path argument."""
    repo_root = get_repo_root()
    script_path = repo_root / "bin" / "anvil"

    result = subprocess.run([str(script_path), "pull"], capture_output=True, text=True)
    assert result.returncode == 1
    assert "Usage: bin/anvil pull <remote_path> [local_dest]" in result.stdout or (
        "Usage: bin/anvil pull <remote_path> [local_dest]" in result.stderr
    )


def test_anvil_runner_cli_requires_command() -> None:
    """Verify bin/anvil cli requires a command argument."""
    repo_root = get_repo_root()
    script_path = repo_root / "bin" / "anvil"

    result = subprocess.run([str(script_path), "cli"], capture_output=True, text=True)
    assert result.returncode == 1
    assert "Usage: bin/anvil cli <command...>" in result.stdout or (
        "Usage: bin/anvil cli <command...>" in result.stderr
    )


def test_anvil_runner_host_requires_subcommand() -> None:
    """Verify bin/anvil host requires a subcommand argument."""
    repo_root = get_repo_root()
    script_path = repo_root / "bin" / "anvil"

    result = subprocess.run([str(script_path), "host"], capture_output=True, text=True)
    assert result.returncode == 1
    assert "Usage: bin/anvil host <subcommand...> [args...]" in result.stdout or (
        "Usage: bin/anvil host <subcommand...> [args...]" in result.stderr
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
if [[ "$*" == *"-e"* ]] || [[ "$*" == *"-f"* ]] || [[ "$*" == *"-d"* ]]; then
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
    env["REMOTE_USER"] = "ubuntu"
    env["REMOTE_DIR"] = "/workspace/firmware"

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

    # Test 4: bin/anvil cli
    res_cli = subprocess.run(
        [str(script_path), "cli", "motor", "speed", "100"], env=env, capture_output=True, text=True
    )
    assert res_cli.returncode == 0
    assert "Reverse proxying host CLI to local daemon" in res_cli.stdout

    # Test 5: bin/anvil host
    res_host = subprocess.run([str(script_path), "host", "status"], env=env, capture_output=True, text=True)
    assert res_host.returncode == 0
    assert "Reverse proxying host command to local daemon" in res_host.stdout

    # Test 6: bin/anvil stop
    res_stop = subprocess.run([str(script_path), "stop"], env=env, capture_output=True, text=True)
    assert res_stop.returncode == 0
    assert "Remote processes cleaned up" in res_stop.stdout

    # Validate calls logged
    calls = log_file.read_text(encoding="utf-8")
    assert "MOCK_RSYNC:" in calls
    assert "--exclude target" in calls
    assert "--exclude .git" in calls
    assert "--exclude .anvil.env*" in calls
    assert "MOCK_SSH:" in calls
    assert "pytest" in calls
    assert "verify.sh" in calls
    assert "-R 8899:127.0.0.1:8899" in calls
    assert "python tools/host.py cli motor speed 100" in calls
    assert "python tools/host.py status" in calls
    assert "pkill" in calls


def test_anvil_runner_vast_dynamic_port_and_rsync(tmp_path: Path) -> None:
    """Verify bin/anvil handles dynamic Vast.ai port (-p 34567) and -e 'ssh ... -p 34567' for rsync."""
    repo_root = get_repo_root()
    script_path = repo_root / "bin" / "anvil"

    mock_bin_dir = tmp_path / "bin"
    mock_bin_dir.mkdir(parents=True, exist_ok=True)
    log_file = tmp_path / "calls.log"

    mock_ssh = mock_bin_dir / "ssh"
    mock_ssh.write_text(
        f"""#!/usr/bin/env bash
echo "MOCK_SSH: $@" >> "{log_file}"
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
    env["REMOTE_HOST"] = "ssh5.vast.ai"
    env["REMOTE_PORT"] = "34567"
    env["REMOTE_USER"] = "root"
    env["REMOTE_DIR"] = "/workspace/firmware"

    res = subprocess.run([str(script_path), "sync"], env=env, capture_output=True, text=True)
    assert res.returncode == 0

    calls = log_file.read_text(encoding="utf-8")
    assert "-p 34567" in calls
    assert "root@ssh5.vast.ai" in calls
    assert (
        "-e ssh -o ServerAliveInterval=30 -o ServerAliveCountMax=120 -o TCPKeepAlive=yes -o StrictHostKeyChecking=accept-new -p 34567"
        in calls
    )


def test_vast_manager_env_and_lock_lifecycle(tmp_path: Path) -> None:
    """Verify vast_manager environment file updating, lock registration, and pruning."""
    env_file = tmp_path / ".anvil.env"
    lock_file = tmp_path / ".anvil.lock"

    # 1. Write environment
    write_env_file(
        env_file,
        {
            "VAST_INSTANCE_ID": "987654",
            "REMOTE_HOST": "ssh7.vast.ai",
            "REMOTE_PORT": "45678",
            "REMOTE_USER": "root",
        },
    )
    loaded = load_env_file(env_file)
    assert loaded["VAST_INSTANCE_ID"] == "987654"
    assert loaded["REMOTE_HOST"] == "ssh7.vast.ai"
    assert loaded["REMOTE_PORT"] == "45678"

    # 2. Lock registration
    my_pid = os.getpid()
    record_lock("pytest -k test_foo", "firmware", pid=my_pid, lock_file=lock_file)
    assert lock_file.exists()

    # Active lock should identify current process
    active = prune_locks(lock_file)
    assert len(active) == 1
    assert active[0]["pid"] == my_pid
    assert active[0]["repo"] == "firmware"
    assert "test_foo" in active[0]["command"]

    # 3. Unlock
    remove_lock(pid=my_pid, lock_file=lock_file)
    active_after = prune_locks(lock_file)
    assert len(active_after) == 0


def test_vast_manager_bootstrap_script_content() -> None:
    """Verify bootstrap script contains required compiler toolchains, Rust, and Conda commands."""
    script = build_bootstrap_script(target="firmware")
    assert "build-essential" in script
    assert "cmake" in script
    assert "clang" in script
    assert "rustup.rs" in script
    assert "cargo-nextest" in script
    assert "Miniforge" in script
    assert "/workspace/miniforge3" in script
    assert "firmware-env" in script
