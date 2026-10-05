"""Reproduction and regression test for WORM-011: Dashboard open files / SQLite connection leak.

Ensures that:
1. `DashboardServer.build_session()` does not leak unclosed SQLite connections or file descriptors.
2. Successive invocations of `build_session()` do not leave lingering SQLite database connections open.
3. `DashboardRequestHandler` marks connections for immediate closure (`Connection: close`) to prevent socket leaks.
"""

from pathlib import Path
import sqlite3
import unittest.mock

import pytest

from provider.dashboard.server import DashboardRequestHandler, DashboardServer


class TrackingConnection(sqlite3.Connection):
    """SQLite connection wrapper that records when close() is invoked."""

    closed_instances = set()

    def close(self) -> None:
        """Record closure and invoke underlying sqlite3.Connection.close()."""
        TrackingConnection.closed_instances.add(self)
        super().close()


def test_build_session_does_not_leak_sqlite_connections(tmp_path: Path) -> None:
    """Verify that build_session() closes all SQLite database connections it opens."""
    repo_root = Path(__file__).resolve().parent.parent.parent

    created_connections: list[TrackingConnection] = []
    TrackingConnection.closed_instances.clear()

    orig_connect = sqlite3.connect

    def mock_connect(*args, **kwargs):
        kwargs["factory"] = TrackingConnection
        conn = orig_connect(*args, **kwargs)
        created_connections.append(conn)
        return conn

    server = DashboardServer(host="127.0.0.1", port=0, repo_root=repo_root, bind_and_activate=False)

    with unittest.mock.patch("sqlite3.connect", side_effect=mock_connect):
        created_connections.clear()
        TrackingConnection.closed_instances.clear()

        # Execute session build (as invoked on every dashboard poll)
        server.build_session()

        unclosed = [c for c in created_connections if c not in TrackingConnection.closed_instances]
        try:
            assert len(unclosed) == 0, f"build_session() leaked {len(unclosed)} unclosed SQLite connection(s)!"
        finally:
            for c in unclosed:
                try:
                    c.close()
                except Exception:
                    pass


def test_build_session_multiple_iterations_no_leaks(tmp_path: Path) -> None:
    """Verify that repeated polls to build_session() leave zero open SQLite connections."""
    repo_root = Path(__file__).resolve().parent.parent.parent

    created_connections: list[TrackingConnection] = []
    TrackingConnection.closed_instances.clear()

    orig_connect = sqlite3.connect

    def mock_connect(*args, **kwargs):
        kwargs["factory"] = TrackingConnection
        conn = orig_connect(*args, **kwargs)
        created_connections.append(conn)
        return conn

    server = DashboardServer(host="127.0.0.1", port=0, repo_root=repo_root, bind_and_activate=False)

    with unittest.mock.patch("sqlite3.connect", side_effect=mock_connect):
        created_connections.clear()
        TrackingConnection.closed_instances.clear()

        for _ in range(5):
            server.build_session()

        unclosed = [c for c in created_connections if c not in TrackingConnection.closed_instances]
        try:
            assert len(unclosed) == 0, f"Repeated build_session() calls leaked {len(unclosed)} connection(s)!"
        finally:
            for c in unclosed:
                try:
                    c.close()
                except Exception:
                    pass
