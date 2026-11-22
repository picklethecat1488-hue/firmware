"""Host daemon package for target synchronization, RTT logging, CLI, and FS workstation."""

from typing import Any

__all__ = [
    "DaemonRunner",
    "HostDaemonRequestHandler",
    "HostDaemonServer",
    "ManualPromptState",
]


def __getattr__(name: str) -> Any:
    """Lazily resolve heavy daemon runtime classes so lightweight CLI subcommands remain stdlib-only."""
    if name == "ManualPromptState":
        from host_daemon.model.manual_prompt import ManualPromptState

        return ManualPromptState
    if name == "DaemonRunner":
        from host_daemon.provider.daemon.runner import DaemonRunner

        return DaemonRunner
    if name in ("HostDaemonRequestHandler", "HostDaemonServer"):
        from host_daemon.provider.daemon import server as _srv

        return getattr(_srv, name)
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")
