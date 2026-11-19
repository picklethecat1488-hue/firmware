"""Asynchronous HostSessionClient and log-watcher primitives built on top of tools/host.py."""

import asyncio
from dataclasses import dataclass, field
import json
from pathlib import Path
import socket
import subprocess
import sys
import time
from typing import Any, Callable, Dict, List, Optional, Sequence, Tuple, Union
import urllib.error
import urllib.request

from device_tests.model import (
    DetectedBoard,
    TargetBinarySpec,
    check_regex,
    parse_query_metadata_output,
    resolve_target_binary,
)

DEFAULT_DAEMON_HOST: str = "127.0.0.1"
DEFAULT_DAEMON_PORT: int = 8899
DEFAULT_HTTP_TIMEOUT_SEC: float = 60.0
DEFAULT_CLI_TIMEOUT_SEC: float = 5.0
DEFAULT_LOG_WAIT_TIMEOUT_SEC: float = 10.0
DEFAULT_SYNC_TIMEOUT_SEC: float = 180.0
LOG_POLL_INTERVAL_SEC: float = 0.04
SYNC_POLL_INTERVAL_SEC: float = 0.10
DAEMON_BOOT_TIMEOUT_SEC: float = 8.0
DAEMON_STOP_WAIT_TIMEOUT_SEC: float = 4.0
DAEMON_TERMINATE_WAIT_TIMEOUT_SEC: float = 2.0
POST_REBOOT_SETTLE_SEC: float = 0.25


@dataclass
class SyncOutcome:
    """Result of ensuring a target firmware binary is compiled, flashed, and connected."""

    synced: bool
    reused_existing: bool
    package: str
    bin_name: str
    arm_target: str
    profile: str
    boot_log_cursor: int = 0


@dataclass
class LogStreamWatcher:
    """Asynchronous watcher capturing daemon/target logs emitted after context entry."""

    patterns: Tuple[str, ...]
    start_log_id: int
    timeout: float
    channel: Optional[str]
    matched_lines: List[str] = field(default_factory=list)
    entries: List[Dict[str, Any]] = field(default_factory=list)
    _task: Optional[asyncio.Task[List[str]]] = field(default=None, init=False, repr=False)
    _fetch_logs_fn: Optional[Callable[[], Any]] = field(default=None, init=False, repr=False)

    async def _poll_until_matched(self) -> List[str]:
        """Poll log stream from `start_log_id` until all patterns match or `timeout` elapses."""
        assert self._fetch_logs_fn is not None
        deadline = time.monotonic() + max(self.timeout, LOG_POLL_INTERVAL_SEC)
        unmatched: List[str] = list(self.patterns)
        found: Dict[str, str] = {}

        while time.monotonic() < deadline:
            raw_logs = await self._fetch_logs_fn()
            filtered = [
                entry
                for entry in raw_logs
                if int(entry.get("id", 0)) > self.start_log_id
                and (self.channel is None or entry.get("channel") == self.channel)
            ]
            self.entries = filtered
            combined = "\n".join(str(entry.get("message", "")) for entry in filtered)

            still_missing: List[str] = []
            for pat in unmatched:
                ok, line = check_regex(combined, pat)
                if ok:
                    found[pat] = line
                else:
                    still_missing.append(pat)

            unmatched = still_missing
            if not unmatched:
                self.matched_lines = [found[p] for p in self.patterns if p in found]
                return list(self.matched_lines)

            await asyncio.sleep(LOG_POLL_INTERVAL_SEC)

        recent_preview = "\n".join(str(e.get("message", "")) for e in self.entries[-20:])
        raise AssertionError(
            f"Timed out after {self.timeout:.1f}s waiting for target logs matching {unmatched}. "
            f"Captured {len(self.entries)} log entries since id={self.start_log_id}:\n{recent_preview}"
        )

    async def __aenter__(self) -> "LogStreamWatcher":
        """Start background polling task before entering the command block."""
        self._task = asyncio.create_task(self._poll_until_matched())
        await asyncio.sleep(0)
        return self

    async def __aexit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Await log matcher completion unless the body itself raised an exception."""
        if self._task is None:
            return
        if exc_type is not None:
            self._task.cancel()
            try:
                await self._task
            except (asyncio.CancelledError, AssertionError):
                pass
            return
        await self._task

    async def matched(self) -> List[str]:
        """Return matched log lines once the watcher task completes."""
        if self._task is not None and not self._task.done():
            await self._task
        return list(self.matched_lines)


def _find_free_local_port(host: str = DEFAULT_DAEMON_HOST) -> int:
    """Bind an ephemeral TCP socket to select an available loopback port."""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.bind((host, 0))
        return int(sock.getsockname()[1])


def _http_json_sync(
    url: str,
    payload: Optional[Dict[str, Any]] = None,
    timeout: float = DEFAULT_HTTP_TIMEOUT_SEC,
) -> Any:
    """Perform a synchronous JSON GET or POST request against the local host.py server."""
    headers = {"Content-Type": "application/json"}
    data = json.dumps(payload).encode("utf-8") if payload is not None else None
    req = urllib.request.Request(
        url,
        data=data,
        headers=headers,
        method="POST" if data is not None else "GET",
    )
    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            raw = resp.read().decode("utf-8")
            return json.loads(raw) if raw else {}
    except urllib.error.HTTPError as err:
        err_body = err.read().decode("utf-8", errors="replace")
        raise RuntimeError(f"HTTP {err.code} from {url}: {err_body}") from err


def query_metadata_standalone(repo_root: Path) -> Optional[DetectedBoard]:
    """Query `host_fs query_metadata` via `host.py`'s ELF probe helper when no daemon is running."""
    from host_daemon.model.metadata import probe_attached_target_metadata

    res = probe_attached_target_metadata(repo_root)
    if not res.get("connected"):
        return None
    return DetectedBoard.from_metadata_response(res)


class AsyncHostSessionClient:
    """Asynchronous client driving firmware sync, CLI, host_fs, reboot, and logs via host.py."""

    def __init__(
        self,
        repo_root: Path,
        host: str = DEFAULT_DAEMON_HOST,
        port: int = DEFAULT_DAEMON_PORT,
        owns_daemon: bool = False,
        manual_enabled: bool = False,
        runner: Any = None,
        prior_target_config: Optional[Dict[str, Any]] = None,
        spawned_proc: Optional[subprocess.Popen[Any]] = None,
    ) -> None:
        """Initialize client state for either HTTP IPC or direct in-process DaemonRunner."""
        self.repo_root = repo_root
        self.host = host
        self.port = port
        self.base_url = f"http://{host}:{port}"
        self.owns_daemon = owns_daemon
        self.manual_enabled = manual_enabled
        self._runner = runner
        self._prior_target_config = prior_target_config
        self._spawned_proc = spawned_proc
        self._reconfigured_existing_daemon = False
        self._cached_board: Optional[DetectedBoard] = None
        self._board_queried = False
        self._verified_binary_key: Optional[Tuple[str, str, str]] = None
        self.last_boot_log_cursor: int = 0
        self.last_output_log: str = ""

    @classmethod
    def from_runner(
        cls,
        runner: Any,
        *,
        manual_enabled: bool = True,
    ) -> "AsyncHostSessionClient":
        """Construct an in-process client bound directly to an active `DaemonRunner`."""
        return cls(
            repo_root=runner.repo_root,
            owns_daemon=False,
            manual_enabled=manual_enabled,
            runner=runner,
        )

    @classmethod
    async def connect(
        cls,
        repo_root: Path,
        *,
        host: str = DEFAULT_DAEMON_HOST,
        port: Optional[int] = None,
        manual_enabled: bool = False,
        spawn_if_needed: bool = True,
    ) -> "AsyncHostSessionClient":
        """Attach to a running host.py daemon (never stopping it) or spawn a temporary headless one."""
        from host_daemon.cli import get_lock_data

        candidate_ports: List[int] = []
        if port is not None:
            candidate_ports.append(port)
        else:
            lock_data = get_lock_data()
            if lock_data and lock_data.get("port"):
                candidate_ports.append(int(lock_data["port"]))
            if DEFAULT_DAEMON_PORT not in candidate_ports:
                candidate_ports.append(DEFAULT_DAEMON_PORT)

        for candidate_port in candidate_ports:
            try:
                status = await asyncio.to_thread(
                    _http_json_sync,
                    f"http://{host}:{candidate_port}/api/status",
                    None,
                    2.0,
                )
                if isinstance(status, dict) and status.get("status") == "running":
                    prior_cfg: Optional[Dict[str, Any]] = None
                    if bool(status.get("flashed")):
                        targets_info = await asyncio.to_thread(
                            _http_json_sync,
                            f"http://{host}:{candidate_port}/api/targets",
                            None,
                            3.0,
                        )
                        if isinstance(targets_info, dict):
                            prior_cfg = dict(targets_info.get("current", {}))
                    return cls(
                        repo_root=repo_root,
                        host=host,
                        port=candidate_port,
                        owns_daemon=False,
                        manual_enabled=manual_enabled,
                        prior_target_config=prior_cfg,
                    )
            except (OSError, RuntimeError):
                continue

        if not spawn_if_needed:
            return cls(
                repo_root=repo_root,
                host=host,
                port=port or DEFAULT_DAEMON_PORT,
                owns_daemon=False,
                manual_enabled=manual_enabled,
            )

        bound_port = port or _find_free_local_port(host)
        host_py = repo_root / "tools" / "host.py"
        proc = subprocess.Popen(
            [
                sys.executable,
                str(host_py),
                "--no-detach",
                "--no-browser",
                "--host",
                host,
                "--port",
                str(bound_port),
            ],
            cwd=str(repo_root),
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
        )

        deadline = time.monotonic() + DAEMON_BOOT_TIMEOUT_SEC
        while time.monotonic() < deadline:
            if proc.poll() is not None:
                out_text = proc.stdout.read() if proc.stdout else ""
                raise RuntimeError(
                    f"Spawned host.py daemon exited prematurely with code {proc.returncode} "
                    f"on {host}:{bound_port}:\n{out_text}"
                )
            try:
                st = await asyncio.to_thread(
                    _http_json_sync,
                    f"http://{host}:{bound_port}/api/status",
                    None,
                    1.5,
                )
                if isinstance(st, dict) and st.get("status") == "running":
                    return cls(
                        repo_root=repo_root,
                        host=host,
                        port=bound_port,
                        owns_daemon=True,
                        manual_enabled=manual_enabled,
                        spawned_proc=proc,
                    )
            except (OSError, RuntimeError):
                await asyncio.sleep(0.10)

        proc.terminate()
        raise RuntimeError(f"Timed out waiting for spawned host.py daemon on {host}:{bound_port}")

    async def _request(
        self,
        endpoint: str,
        payload: Optional[Dict[str, Any]] = None,
        timeout: float = DEFAULT_HTTP_TIMEOUT_SEC,
    ) -> Any:
        """Issue an async non-blocking JSON request to the bound host.py HTTP server."""
        url = f"{self.base_url}{endpoint}"
        return await asyncio.to_thread(_http_json_sync, url, payload, timeout)

    async def get_status(self) -> Dict[str, Any]:
        """Fetch live daemon status dictionary."""
        if self._runner is not None:
            return dict(self._runner.get_status())
        res = await self._request("/api/status", None, timeout=5.0)
        return dict(res) if isinstance(res, dict) else {}

    async def get_logs(self) -> List[Dict[str, Any]]:
        """Return buffered daemon and target RTT log entries."""
        if self._runner is not None:
            with self._runner._lock:
                return list(self._runner.logs)
        res = await self._request("/api/logs", None, timeout=5.0)
        return list(res) if isinstance(res, list) else []

    async def current_log_cursor(self) -> int:
        """Return the highest monotonic `id` currently present in `/api/logs`."""
        logs = await self.get_logs()
        if not logs:
            return 0
        return max(int(entry.get("id", 0)) for entry in logs)

    async def query_metadata(self, *, force_refresh: bool = False) -> Optional[DetectedBoard]:
        """Query on-device `.program_metadata` and target ELF match validation from `host.py` (`/api/metadata`)."""
        if self._board_queried and not force_refresh:
            return self._cached_board

        if self._runner is not None and hasattr(self._runner, "query_target_metadata"):
            meta_res = await asyncio.to_thread(self._runner.query_target_metadata)
            self._cached_board = (
                DetectedBoard.from_metadata_response(meta_res)
                if isinstance(meta_res, dict) and meta_res.get("connected")
                else None
            )
            self._board_queried = True
            return self._cached_board

        if self._runner is not None:
            raw_out = await asyncio.to_thread(self._runner.execute_fs, "query_metadata", ["--json"])
            self._cached_board = parse_query_metadata_output(raw_out)
            self._board_queried = True
            return self._cached_board

        try:
            meta_res = await self._request("/api/metadata", None, timeout=20.0)
            if isinstance(meta_res, dict):
                self._cached_board = (
                    DetectedBoard.from_metadata_response(meta_res) if meta_res.get("connected") else None
                )
                self._board_queried = True
                return self._cached_board
        except (OSError, RuntimeError):
            self._cached_board = await asyncio.to_thread(
                query_metadata_standalone,
                self.repo_root,
            )
            self._board_queried = True
            return self._cached_board

        self._cached_board = None
        self._board_queried = True
        return None

    async def ensure_target_binary(
        self,
        target_binary: Union[TargetBinarySpec, str, None],
        *,
        force: bool = False,
        timeout: float = DEFAULT_SYNC_TIMEOUT_SEC,
    ) -> Optional[SyncOutcome]:
        """Ensure `target_binary` is compiled, flashed, and matches target ELF metadata on `host.py`."""
        spec = resolve_target_binary(target_binary, repo_root=self.repo_root)
        if spec is None:
            return None

        status = await self.get_status()
        desired_profile = "release" if spec.release else "debug"
        bin_key = (spec.package, spec.bin_name, desired_profile)
        already_active = (
            not force
            and status.get("package") == spec.package
            and status.get("bin") == spec.bin_name
            and status.get("profile") == desired_profile
            and bool(status.get("flashed"))
            and bool(status.get("connected"))
        )
        if already_active:
            if self._verified_binary_key == bin_key:
                return SyncOutcome(
                    synced=False,
                    reused_existing=True,
                    package=spec.package,
                    bin_name=spec.bin_name,
                    arm_target=spec.arm_target,
                    profile=desired_profile,
                    boot_log_cursor=self.last_boot_log_cursor,
                )
            board_check = await self.query_metadata(force_refresh=True)
            if board_check is None or board_check.matches_target:
                self._verified_binary_key = bin_key
                return SyncOutcome(
                    synced=False,
                    reused_existing=True,
                    package=spec.package,
                    bin_name=spec.bin_name,
                    arm_target=spec.arm_target,
                    profile=desired_profile,
                    boot_log_cursor=self.last_boot_log_cursor,
                )

        self._verified_binary_key = None
        pre_sync_cursor = await self.current_log_cursor()
        if not self.owns_daemon:
            self._reconfigured_existing_daemon = True

        if self._runner is not None:
            self._runner.update_config(
                target=spec.arm_target,
                package=spec.package,
                bin_name=spec.bin_name,
                release=spec.release,
                chip=spec.chip,
            )
            sync_state = await asyncio.to_thread(self._runner.sync)
        else:
            await self._request(
                "/api/target/configure",
                {
                    "package": spec.package,
                    "bin": spec.bin_name,
                    "target": spec.arm_target,
                    "chip": spec.chip,
                    "release": spec.release,
                },
                timeout=15.0,
            )
            deadline = time.monotonic() + timeout
            sync_state = {}
            while time.monotonic() < deadline:
                sync_state = await self._request("/api/sync/status", None, timeout=5.0)
                if isinstance(sync_state, dict) and not sync_state.get("active", False):
                    if sync_state.get("step") in ("complete", "failed", "cancelled"):
                        break
                await asyncio.sleep(SYNC_POLL_INTERVAL_SEC)

        if not isinstance(sync_state, dict) or sync_state.get("step") != "complete":
            err_detail = (
                sync_state.get("error") or sync_state.get("message")
                if isinstance(sync_state, dict)
                else "unknown sync state"
            )
            raise AssertionError(
                f"Failed to synchronize target binary '{spec.bin_name}' ({spec.package}): {err_detail}"
            )

        self._board_queried = False
        self._verified_binary_key = bin_key
        self.last_boot_log_cursor = pre_sync_cursor
        return SyncOutcome(
            synced=True,
            reused_existing=False,
            package=spec.package,
            bin_name=spec.bin_name,
            arm_target=spec.arm_target,
            profile=desired_profile,
            boot_log_cursor=pre_sync_cursor,
        )

    async def cli(
        self,
        command: str,
        *,
        timeout: float = DEFAULT_CLI_TIMEOUT_SEC,
    ) -> str:
        """Execute a target shell command via `host.py` and return its captured stdout string."""
        if self._runner is not None:
            output = await asyncio.to_thread(
                self._runner.execute_cli,
                command,
                True,
                timeout,
            )
        else:
            res = await self._request(
                "/api/cli/execute",
                {"command": command, "wait_for_output": True, "timeout": timeout},
                timeout=max(timeout + 5.0, DEFAULT_HTTP_TIMEOUT_SEC),
            )
            output = str(res.get("output", "")) if isinstance(res, dict) else ""

        self.last_output_log = output
        return output

    async def fs(
        self,
        subcommand: str,
        args: Optional[Sequence[str]] = None,
    ) -> str:
        """Execute a `host_fs` subcommand via `host.py` and return its captured stdout string."""
        arg_list = [str(a) for a in (args or ())]
        full_cmd = f"{subcommand} {' '.join(arg_list)}".strip()
        if self._runner is not None:
            output = await asyncio.to_thread(self._runner.execute_fs, subcommand, arg_list)
        else:
            res = await self._request(
                "/api/fs/execute",
                {"command": full_cmd, "subcommand": subcommand, "args": arg_list},
                timeout=DEFAULT_HTTP_TIMEOUT_SEC,
            )
            output = str(res.get("output", "")) if isinstance(res, dict) else ""

        self.last_output_log = output
        return output

    async def reboot(self) -> Dict[str, Any]:
        """Reboot the attached target MCU via `host.py` (`/api/reboot`) and wait for RTT re-attachment."""
        if self._runner is not None:
            fn = getattr(self._runner, "reboot_target", self._runner.reset_target)
            res = await asyncio.to_thread(fn)
        else:
            res = await self._request("/api/reboot", {}, timeout=20.0)
        await asyncio.sleep(POST_REBOOT_SETTLE_SEC)
        return dict(res) if isinstance(res, dict) else {}

    def watch_logs(
        self,
        *patterns: str,
        start_log_id: Optional[int] = None,
        timeout: float = DEFAULT_LOG_WAIT_TIMEOUT_SEC,
        channel: Optional[str] = "target",
        since_boot: bool = False,
    ) -> "_DeferredLogStreamContext":
        """Return an `async with host.watch_logs(...)` context manager that snapshots cursor on entry."""
        return _DeferredLogStreamContext(
            client=self,
            patterns=tuple(patterns),
            explicit_start_id=start_log_id,
            timeout=timeout,
            channel=channel,
            since_boot=since_boot,
        )

    async def publish_manual_prompt(
        self,
        *,
        prompt_id: str = "",
        kind: str = "yes_no",
        unit: str = "",
        test_name: str,
        project: str,
        procedure: str,
        prompt: str,
        active: bool = True,
    ) -> Dict[str, Any]:
        """Send periodic manual test procedure & prompt context to `host.py` and return prompt/response state."""
        payload = {
            "prompt_id": prompt_id,
            "kind": kind,
            "unit": unit,
            "test_name": test_name,
            "project": project,
            "procedure": procedure,
            "prompt": prompt,
            "active": active,
        }
        if self._runner is not None and hasattr(self._runner, "update_manual_prompt"):
            res = await asyncio.to_thread(self._runner.update_manual_prompt, payload)
            return dict(res) if isinstance(res, dict) else {}
        try:
            res = await self._request("/api/manual_prompt", payload, timeout=3.0)
            if isinstance(res, dict):
                prompt_state = res.get("manual_prompt")
                return dict(prompt_state) if isinstance(prompt_state, dict) else dict(res)
        except (OSError, RuntimeError):
            pass
        return {}

    async def close(self) -> None:
        """Tear down owned temporary daemon or restore prior target config on an existing daemon."""
        if self._runner is not None:
            return
        if self.owns_daemon:
            stop_requested = False
            try:
                await self._request("/api/stop", {}, timeout=5.0)
                stop_requested = True
            except (OSError, RuntimeError):
                pass
            if self._spawned_proc is not None and self._spawned_proc.poll() is None:
                if stop_requested:
                    try:
                        await asyncio.to_thread(self._spawned_proc.wait, DAEMON_STOP_WAIT_TIMEOUT_SEC)
                    except subprocess.TimeoutExpired:
                        pass
                if self._spawned_proc.poll() is None:
                    self._spawned_proc.terminate()
                    try:
                        await asyncio.to_thread(self._spawned_proc.wait, DAEMON_TERMINATE_WAIT_TIMEOUT_SEC)
                    except subprocess.TimeoutExpired:
                        self._spawned_proc.kill()
            await asyncio.sleep(POST_REBOOT_SETTLE_SEC)
            return

        if self._reconfigured_existing_daemon and self._prior_target_config:
            prior_pkg = self._prior_target_config.get("package")
            prior_bin = self._prior_target_config.get("bin")
            if prior_pkg and prior_bin:
                try:
                    prior_target = self._prior_target_config.get("target")
                    prior_chip = self._prior_target_config.get("chip")
                    prior_release = bool(self._prior_target_config.get("release", False))
                    if prior_target and prior_chip:
                        restore_spec = TargetBinarySpec(
                            package=str(prior_pkg),
                            bin_name=str(prior_bin),
                            arm_target=str(prior_target),
                            chip=str(prior_chip),
                            release=prior_release,
                        )
                    else:
                        inferred = resolve_target_binary(str(prior_bin), repo_root=self.repo_root)
                        if inferred is None:
                            return
                        restore_spec = TargetBinarySpec(
                            package=inferred.package,
                            bin_name=inferred.bin_name,
                            arm_target=inferred.arm_target,
                            chip=inferred.chip,
                            release=prior_release,
                        )
                    await self.ensure_target_binary(restore_spec)
                except (OSError, RuntimeError, ValueError, AssertionError):
                    pass


class _DeferredLogStreamContext:
    """Async context manager supporting `async with host.watch_logs(...) as watcher:` without double-await."""

    def __init__(
        self,
        *,
        client: AsyncHostSessionClient,
        patterns: Tuple[str, ...],
        explicit_start_id: Optional[int],
        timeout: float,
        channel: Optional[str],
        since_boot: bool,
    ) -> None:
        """Store deferred watcher configuration until `__aenter__` snapshots the log cursor."""
        self._client = client
        self._patterns = patterns
        self._explicit_start_id = explicit_start_id
        self._timeout = timeout
        self._channel = channel
        self._since_boot = since_boot
        self._watcher: Optional[LogStreamWatcher] = None

    async def __aenter__(self) -> LogStreamWatcher:
        """Snapshot the current log cursor and start the background polling task."""
        if self._explicit_start_id is not None:
            start_id = self._explicit_start_id
        elif self._since_boot:
            start_id = self._client.last_boot_log_cursor
        else:
            start_id = await self._client.current_log_cursor()
        self._watcher = LogStreamWatcher(
            patterns=self._patterns,
            start_log_id=start_id,
            timeout=self._timeout,
            channel=self._channel,
        )
        self._watcher._fetch_logs_fn = self._client.get_logs
        return await self._watcher.__aenter__()

    async def __aexit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Delegate exit handling and timeout assertions to the underlying `LogStreamWatcher`."""
        if self._watcher is not None:
            await self._watcher.__aexit__(exc_type, exc_val, exc_tb)
