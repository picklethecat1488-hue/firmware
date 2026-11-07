"""Unit tests for tools/anvil remote execution, synchronization runner, and Vast.ai lifecycle."""

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
    DEFAULT_MAX_COST,
    DEFAULT_MIN_TOPS,
    build_bootstrap_script,
    load_env_file,
    prune_locks,
    record_lock,
    remove_lock,
    search_offers,
    provision_instance,
    write_env_file,
)


def get_repo_root() -> Path:
    """Return repository root path."""
    return Path(__file__).resolve().parents[2]


def test_anvil_runner_executable_and_syntax() -> None:
    """Verify tools/anvil exists, has executable permissions, and passes bash syntax validation."""
    repo_root = get_repo_root()
    script_path = repo_root / "tools" / "anvil"
    assert script_path.exists(), "tools/anvil runner script must exist"

    file_stat = script_path.stat()
    assert bool(file_stat.st_mode & stat.S_IXUSR), "tools/anvil must be executable"

    # Bash syntax check (bash -n tools/anvil)
    result = subprocess.run(["bash", "-n", str(script_path)], capture_output=True, text=True)
    assert result.returncode == 0, f"tools/anvil syntax error: {result.stderr}"


def test_anvil_runner_usage_without_args() -> None:
    """Verify tools/anvil without arguments prints usage instructions and exits with code 1."""
    repo_root = get_repo_root()
    script_path = repo_root / "tools" / "anvil"

    result = subprocess.run([str(script_path)], capture_output=True, text=True)
    assert result.returncode == 1
    assert "Usage: tools/anvil" in result.stdout or "Usage: tools/anvil" in result.stderr
    assert "provision" in result.stdout
    assert "bootstrap" in result.stdout
    assert "release" in result.stdout
    assert "auto-release" in result.stdout
    assert "status" in result.stdout
    assert "--min-tops" in result.stdout
    assert "--max-cost" in result.stdout


def test_anvil_runner_pull_requires_path() -> None:
    """Verify tools/anvil pull requires a remote path argument."""
    repo_root = get_repo_root()
    script_path = repo_root / "tools" / "anvil"

    result = subprocess.run([str(script_path), "pull"], capture_output=True, text=True)
    assert result.returncode == 1
    assert "Usage: tools/anvil pull <remote_path> [local_dest]" in result.stdout or (
        "Usage: tools/anvil pull <remote_path> [local_dest]" in result.stderr
    )


def test_anvil_runner_cli_requires_command() -> None:
    """Verify tools/anvil cli requires a command argument."""
    repo_root = get_repo_root()
    script_path = repo_root / "tools" / "anvil"

    result = subprocess.run([str(script_path), "cli"], capture_output=True, text=True)
    assert result.returncode == 1
    assert "Usage: tools/anvil cli <command...>" in result.stdout or (
        "Usage: tools/anvil cli <command...>" in result.stderr
    )


def test_anvil_runner_host_requires_subcommand() -> None:
    """Verify tools/anvil host requires a subcommand argument."""
    repo_root = get_repo_root()
    script_path = repo_root / "tools" / "anvil"

    result = subprocess.run([str(script_path), "host"], capture_output=True, text=True)
    assert result.returncode == 1
    assert "Usage: tools/anvil host <subcommand...> [args...]" in result.stdout or (
        "Usage: tools/anvil host <subcommand...> [args...]" in result.stderr
    )


def test_anvil_runner_sync_and_pytest_mocked(tmp_path: Path) -> None:
    """Verify tools/anvil sync and pytest dispatch appropriate rsync and ssh commands with proper exclusions."""
    repo_root = get_repo_root()
    script_path = repo_root / "tools" / "anvil"

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

    # Test 1: tools/anvil sync
    res_sync = subprocess.run([str(script_path), "sync"], env=env, capture_output=True, text=True)
    assert res_sync.returncode == 0
    assert "Sync complete" in res_sync.stdout

    # Test 2: tools/anvil pytest
    res_pytest = subprocess.run(
        [str(script_path), "pytest", "-k", "test_mock"], env=env, capture_output=True, text=True
    )
    assert res_pytest.returncode == 0

    # Test 3: tools/anvil verify
    res_verify = subprocess.run([str(script_path), "verify"], env=env, capture_output=True, text=True)
    assert res_verify.returncode == 0

    # Test 4: tools/anvil cli
    res_cli = subprocess.run(
        [str(script_path), "cli", "motor", "speed", "100"], env=env, capture_output=True, text=True
    )
    assert res_cli.returncode == 0
    assert "Reverse proxying host CLI to local daemon" in res_cli.stdout

    # Test 5: tools/anvil host
    res_host = subprocess.run([str(script_path), "host", "status"], env=env, capture_output=True, text=True)
    assert res_host.returncode == 0
    assert "Reverse proxying host command to local daemon" in res_host.stdout

    # Test 6: tools/anvil stop
    res_stop = subprocess.run([str(script_path), "stop"], env=env, capture_output=True, text=True)
    assert res_stop.returncode == 0
    assert "Remote processes cleaned up" in res_stop.stdout

    # Validate calls logged
    calls = log_file.read_text(encoding="utf-8")
    assert "MOCK_RSYNC:" in calls
    assert "--exclude /target" in calls
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
    """Verify tools/anvil handles dynamic Vast.ai port (-p 34567) and -e 'ssh ... -p 34567' for rsync."""
    repo_root = get_repo_root()
    script_path = repo_root / "tools" / "anvil"

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


def test_search_offers_filters_min_tops_and_max_cost(monkeypatch) -> None:
    """Verify search_offers rejects offers below min_tops or above max_cost."""
    mock_offers = [
        {"id": 1, "gpu_name": "GTX 1080", "total_flops": 12.0, "dph_total": 0.15},  # Low TOPS (junk)
        {"id": 2, "gpu_name": "H100 SXM", "total_flops": 700.0, "dph_total": 4.50},  # Overpriced
        {"id": 3, "gpu_name": "RTX 4090", "total_flops": 82.5, "dph_total": 0.45},  # Ideal match
        {"id": 4, "gpu_name": "RTX 3090", "total_flops": 71.0, "dph_total": 0.28},  # Ideal match
    ]

    import tools.vast_manager as vm

    monkeypatch.setattr(vm, "api_request", lambda endpoint, **kwargs: {"offers": mock_offers})

    # Test with default constraints (min_tops=50.0, max_cost=1.50)
    results = search_offers(min_tops=50.0, max_cost=1.50)
    res_ids = [o["id"] for o in results]
    assert res_ids == [3, 4], f"Expected only offers 3 and 4, got {res_ids}"

    # Test with custom tighter constraints
    results_tight = search_offers(min_tops=80.0, max_cost=0.50)
    assert [o["id"] for o in results_tight] == [3]


def test_provision_instance_rejects_junk_and_overpriced_offers(monkeypatch, tmp_path: Path) -> None:
    """Verify provision_instance refuses to claim machines when only junk or overpriced hardware is available."""
    junk_offers = [
        {"id": 101, "gpu_name": "GTX 1060", "total_flops": 8.0, "dph_total": 0.10},
        {"id": 102, "gpu_name": "Tesla T4", "total_flops": 25.0, "dph_total": 0.35},
        {"id": 103, "gpu_name": "A100 80GB", "total_flops": 312.0, "dph_total": 3.80},
    ]

    import tools.vast_manager as vm

    monkeypatch.setattr(vm, "get_api_key", lambda: "mock_api_key_12345")
    monkeypatch.setattr(vm, "ensure_ssh_key_registered", lambda: True)
    monkeypatch.setattr(vm, "probe_ssh_connection", lambda h, p, **kw: True)
    monkeypatch.setattr(vm, "api_request", lambda endpoint, **kwargs: {"offers": junk_offers})

    env_file = tmp_path / ".anvil.env"
    res = provision_instance(min_tops=50.0, max_cost=1.50, env_file=env_file)
    assert "error" in res, "Provision must fail if no offers meet min_tops and max_cost"
    assert "No offers available" in res["error"] or "matching" in res["error"]


def test_provision_instance_respects_config_and_writes_hardware_limits(monkeypatch, tmp_path: Path) -> None:
    """Verify provision_instance resolves min_tops and max_cost from env_file and records them."""
    good_offers = [
        {"id": 201, "gpu_name": "RTX 4090", "total_flops": 82.5, "dph_total": 0.42},
    ]

    import tools.vast_manager as vm

    monkeypatch.setattr(vm, "get_api_key", lambda: "mock_api_key_12345")
    monkeypatch.setattr(vm, "ensure_ssh_key_registered", lambda: True)
    monkeypatch.setattr(vm, "probe_ssh_connection", lambda h, p, **kw: True)
    monkeypatch.setattr(
        vm,
        "api_request",
        lambda endpoint, method="GET", data=None, **kwargs: (
            {"new_contract": 99999}
            if method == "PUT"
            else (
                {
                    "instances": [
                        {
                            "id": 99999,
                            "actual_status": "running",
                            "ssh_host": "ssh8.vast.ai",
                            "ssh_port": 22222,
                            "gpu_name": "RTX 4090",
                            "total_flops": 82.5,
                            "dph_total": 0.42,
                        }
                    ]
                }
                if "instances" in endpoint
                else {"offers": good_offers}
            )
        ),
    )

    env_file = tmp_path / ".anvil.env"
    write_env_file(env_file, {"MIN_TOPS": "60.0", "MAX_COST": "1.20"})

    res = provision_instance(env_file=env_file)
    assert "error" not in res
    assert res["instance_id"] == 99999

    loaded = load_env_file(env_file)
    assert loaded["VAST_INSTANCE_ID"] == "99999"
    assert loaded["REMOTE_HOST"] == "ssh8.vast.ai"
    assert loaded["REMOTE_PORT"] == "22222"
    assert loaded["MIN_TOPS"] == "60.0"
    assert loaded["MAX_COST"] == "1.20"


def test_api_request_routes_instances_list_to_v1(monkeypatch) -> None:
    """Verify api_request routes instances/ to v1 API and other endpoints to v0 API."""
    import urllib.request
    import tools.vast_manager as vm

    captured_urls = []

    class MockResponse:
        def __init__(self, data=b"{}"):
            self.data = data

        def __enter__(self):
            return self

        def __exit__(self, exc_type, exc_val, exc_tb):
            pass

        def read(self):
            return self.data

    def mock_urlopen(req, timeout=30):
        captured_urls.append(req.full_url)
        return MockResponse(b'{"success": true, "instances": []}')

    monkeypatch.setattr(vm, "get_api_key", lambda: "mock_key")
    monkeypatch.setattr(urllib.request, "urlopen", mock_urlopen)

    # 1. Instances listing -> v1
    vm.api_request("instances/")
    assert any("api/v1/instances" in u for u in captured_urls)

    # 2. Single instance query -> v0
    captured_urls.clear()
    vm.api_request("instances/555/")
    assert any("api/v0/instances/555" in u for u in captured_urls)

    # 3. Bundles query -> v0
    captured_urls.clear()
    vm.api_request("bundles/?q=test")
    assert any("api/v0/bundles" in u for u in captured_urls)


def test_check_instance_failure_patterns() -> None:
    """Verify check_instance_failure detects Docker daemon timeouts and fatal container states."""
    from tools.vast_manager import check_instance_failure

    # 1. User reported Docker registry timeout
    user_err = (
        'Error response from daemon: Head "https://registry-1.docker.io/v2/library/ubuntu/manifests/22.04": '
        'Get "https://auth.docker.io/token?scope=repository%3Alibrary%2Fubuntu%3Apull&service=registry.docker.io": '
        "net/http: request canceled while waiting for connection (Client.Timeout exceeded while awaiting headers)"
    )
    assert check_instance_failure({"actual_status": "loading", "status_msg": user_err}) == user_err

    # 2. Fatal status
    assert "offline" in str(check_instance_failure({"actual_status": "offline", "status_msg": "Host down"}))
    assert "failed" in str(check_instance_failure({"actual_status": "failed", "status_msg": "Boot failed"}))

    # 3. Healthy instance returns None
    assert check_instance_failure({"actual_status": "running", "cur_state": "running", "status_msg": None}) is None


def test_provision_instance_retries_and_cleans_up_on_daemon_error(monkeypatch, tmp_path: Path) -> None:
    """Verify provision_instance auto-releases broken instances on daemon pull error and retries with next host."""
    import tools.vast_manager as vm

    offers = [
        {"id": 1001, "machine_id": 133175, "gpu_name": "RTX 4090", "total_flops": 80.0, "dph_total": 0.30},
        {"id": 1002, "machine_id": 144200, "gpu_name": "RTX 4090", "total_flops": 80.0, "dph_total": 0.35},
    ]

    deleted_instances = []
    created_offers = []

    def mock_api_request(endpoint, method="GET", data=None, **kwargs):
        if method == "PUT" and "asks/" in endpoint:
            oid = int(endpoint.split("/")[1])
            created_offers.append(oid)
            if oid == 1001:
                return {"new_contract": 59343680}
            else:
                return {"new_contract": 59343681}
        elif method == "DELETE" and "instances/" in endpoint:
            iid = int(endpoint.split("/")[1])
            deleted_instances.append(iid)
            return {"success": True}
        elif "instances" in endpoint:
            # If asking about instance 59343680, report daemon timeout
            # If asking about instance 59343681, report healthy running state
            return {
                "instances": [
                    {
                        "id": 59343680,
                        "machine_id": 133175,
                        "actual_status": "loading",
                        "status_msg": 'Error response from daemon: Head "https://registry-1.docker.io/v2/": Client.Timeout exceeded',
                    },
                    {
                        "id": 59343681,
                        "machine_id": 144200,
                        "actual_status": "running",
                        "status_msg": None,
                        "ssh_host": "ssh3.vast.ai",
                        "ssh_port": 22233,
                    },
                ]
            }
        elif "bundles/" in endpoint:
            return {"offers": offers}
        return {}

    monkeypatch.setattr(vm, "get_api_key", lambda: "mock_api_key")
    monkeypatch.setattr(vm, "ensure_ssh_key_registered", lambda: True)
    monkeypatch.setattr(vm, "probe_ssh_connection", lambda h, p, **kw: True)
    monkeypatch.setattr(vm, "api_request", mock_api_request)
    monkeypatch.setattr(time, "sleep", lambda s: None)

    env_file = tmp_path / ".anvil.env"
    res = provision_instance(min_tops=50.0, max_cost=1.50, env_file=env_file)

    assert "error" not in res
    assert res["instance_id"] == 59343681
    assert 59343680 in deleted_instances, "Broken instance must be destroyed immediately"
    assert created_offers == [1001, 1002], "Must try first offer, then retry with next offer"

    saved_env = vm.load_env_file(env_file)
    assert saved_env["VAST_INSTANCE_ID"] == "59343681"
    assert saved_env["REMOTE_HOST"] == "ssh3.vast.ai"
    assert saved_env["REMOTE_PORT"] == "22233"


def test_release_instance_cleans_unreferenced_instances(monkeypatch, tmp_path: Path) -> None:
    """Verify release_instance destroys orphaned account instances when ~/.anvil.env is unconfigured."""
    import tools.vast_manager as vm

    deleted_instances = []

    def mock_api_request(endpoint, method="GET", **kwargs):
        if method == "DELETE" and "instances/" in endpoint:
            iid = int(endpoint.split("/")[1])
            deleted_instances.append(iid)
            return {"success": True}
        elif "instances" in endpoint:
            return {
                "instances": [
                    {"id": 88888, "actual_status": "loading"},
                ]
            }
        return {}

    monkeypatch.setattr(vm, "get_api_key", lambda: "mock_key")
    monkeypatch.setattr(vm, "api_request", mock_api_request)

    env_file = tmp_path / ".anvil.env"
    assert not env_file.exists()

    success = vm.release_instance(env_file=env_file)
    assert success is True
    assert 88888 in deleted_instances


def test_probe_ssh_connection(monkeypatch) -> None:
    """Verify probe_ssh_connection executes non-interactive test and validates output."""
    import subprocess
    import tools.vast_manager as vm

    class MockResult:
        def __init__(self, code, out):
            self.returncode = code
            self.stdout = out

    # 1. Success case
    monkeypatch.setattr(subprocess, "run", lambda cmd, **kw: MockResult(0, "READY\n"))
    assert vm.probe_ssh_connection("ssh.vast.ai", 12345) is True

    # 2. Failure case
    monkeypatch.setattr(subprocess, "run", lambda cmd, **kw: MockResult(255, "Connection refused\n"))
    assert vm.probe_ssh_connection("ssh.vast.ai", 12345) is False
