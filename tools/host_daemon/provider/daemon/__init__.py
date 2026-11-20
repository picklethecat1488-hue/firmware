"""Host daemon runner and server provider package."""

from typing import Any

__all__ = ["DaemonRunner", "HostDaemonRequestHandler", "HostDaemonServer"]


def __getattr__(name: str) -> Any:
    """Lazily load `DaemonRunner` and `HostDaemonServer` on first attribute access."""
    if name == "DaemonRunner":
        from host_daemon.provider.daemon.runner import DaemonRunner

        return DaemonRunner
    if name in ("HostDaemonRequestHandler", "HostDaemonServer"):
        from host_daemon.provider.daemon import server as _srv

        return getattr(_srv, name)
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")
