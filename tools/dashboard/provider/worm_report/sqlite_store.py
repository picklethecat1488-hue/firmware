"""SQLite backing store for worm report database.

Provides robust, concurrent, atomic persistence for worm reports,
reproduction steps, attachments, and tracker metadata using SQLite.
"""

import contextlib
import json
from pathlib import Path
import re
import sqlite3
from typing import Any, Dict, Generator, List, Optional
import uuid as uuid_pkg

from model.worm_report import (
    WormAttachmentModel,
    WormCategory,
    WormDatabaseModel,
    WormReportModel,
    WormSeverity,
    WormStatus,
)


class SQLiteWormStore:
    """SQLite persistence engine for the worm workstation."""

    def __init__(self, db_path: Path) -> None:
        """Initialize SQLite worm store.

        Args:
            db_path: Path to the SQLite database file.
        """
        self.db_path = Path(db_path)
        self.db_path.parent.mkdir(parents=True, exist_ok=True)
        self._init_schema()

    @contextlib.contextmanager
    def _get_connection(self) -> Generator[sqlite3.Connection, None, None]:
        """Create, configure, and safely close a SQLite connection with foreign keys enabled."""
        self.db_path.parent.mkdir(parents=True, exist_ok=True)
        conn = sqlite3.connect(str(self.db_path), timeout=30.0)
        conn.row_factory = sqlite3.Row
        conn.execute("PRAGMA foreign_keys = ON")
        conn.execute("PRAGMA journal_mode = WAL")
        self._ensure_schema(conn)
        try:
            with conn:
                yield conn
        finally:
            conn.close()

    def _ensure_schema(self, conn: sqlite3.Connection) -> None:
        """Ensure schema tables and indices exist in connection."""
        cursor = conn.execute("SELECT 1 FROM sqlite_master WHERE type='table' AND name='worm_attachments'")
        if not cursor.fetchone():
            self._init_schema_with_conn(conn)
            return
        cursor = conn.execute("PRAGMA table_info(worms)")
        cols = [row["name"] for row in cursor.fetchall()]
        if "uuid" not in cols:
            self._init_schema_with_conn(conn)

    def _init_schema(self) -> None:
        """Initialize tables and indices if they do not already exist."""
        with self._get_connection():
            pass

    def _init_schema_with_conn(self, conn: sqlite3.Connection) -> None:
        """Execute table creation and migrations on connection."""
        conn.executescript(
            """
            CREATE TABLE IF NOT EXISTS metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS worms (
                id TEXT PRIMARY KEY,
                uuid TEXT,
                title TEXT NOT NULL,
                status TEXT NOT NULL,
                severity TEXT NOT NULL,
                category TEXT NOT NULL,
                component TEXT NOT NULL DEFAULT '',
                description TEXT NOT NULL DEFAULT '',
                reproduction_steps TEXT NOT NULL DEFAULT '[]',
                expected_behavior TEXT NOT NULL DEFAULT '',
                actual_behavior TEXT NOT NULL DEFAULT '',
                logs TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT '',
                updated_at TEXT NOT NULL DEFAULT '',
                resolved_at TEXT,
                resolution_notes TEXT NOT NULL DEFAULT ''
            );

            CREATE TABLE IF NOT EXISTS worm_attachments (
                id TEXT PRIMARY KEY,
                worm_id TEXT NOT NULL,
                filename TEXT NOT NULL,
                file_type TEXT NOT NULL DEFAULT 'reference',
                file_path TEXT NOT NULL,
                size_bytes INTEGER NOT NULL DEFAULT 0,
                description TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT '',
                FOREIGN KEY (worm_id) REFERENCES worms(id) ON DELETE CASCADE ON UPDATE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_worm_attachments_worm_id ON worm_attachments(worm_id);
            CREATE INDEX IF NOT EXISTS idx_worms_status ON worms(status);
            CREATE INDEX IF NOT EXISTS idx_worms_severity ON worms(severity);
            """
        )
        # Ensure uuid column exists on preexisting databases
        cursor = conn.execute("PRAGMA table_info(worms)")
        cols = [row["name"] for row in cursor.fetchall()]
        if "uuid" not in cols:
            conn.execute("ALTER TABLE worms ADD COLUMN uuid TEXT")

        # Backfill any missing UUIDs
        null_uuid_rows = conn.execute("SELECT id FROM worms WHERE uuid IS NULL OR uuid = ''").fetchall()
        for r in null_uuid_rows:
            conn.execute("UPDATE worms SET uuid = ? WHERE id = ?", (str(uuid_pkg.uuid4()), r["id"]))

        conn.execute("CREATE UNIQUE INDEX IF NOT EXISTS idx_worms_uuid ON worms(uuid)")

    def load_database(self) -> WormDatabaseModel:
        """Load full bug database model from SQLite.

        Returns:
            WormDatabaseModel populated with all bugs, attachments, and metadata.
        """
        with self._get_connection() as conn:
            # Load metadata
            meta_rows = conn.execute("SELECT key, value FROM metadata").fetchall()
            meta: Dict[str, str] = {row["key"]: row["value"] for row in meta_rows}
            title = meta.get("title", "Firmware Engineering Worm Tracker")
            summary = meta.get("summary", "")
            updated_at = meta.get("updated_at", "")

            # Load attachments grouped by worm_id
            att_rows = conn.execute(
                "SELECT id, worm_id, filename, file_type, file_path, size_bytes, description, created_at "
                "FROM worm_attachments ORDER BY created_at ASC"
            ).fetchall()
            attachments_by_bug: Dict[str, List[WormAttachmentModel]] = {}
            for row in att_rows:
                att = WormAttachmentModel(
                    id=row["id"],
                    filename=row["filename"],
                    file_type=row["file_type"],
                    file_path=row["file_path"],
                    size_bytes=row["size_bytes"],
                    description=row["description"],
                    created_at=row["created_at"],
                )
                attachments_by_bug.setdefault(row["worm_id"], []).append(att)

            # Load bugs
            bug_rows = conn.execute(
                "SELECT id, uuid, title, status, severity, category, component, description, "
                "reproduction_steps, expected_behavior, actual_behavior, logs, created_at, "
                "updated_at, resolved_at, resolution_notes FROM worms"
            ).fetchall()

            def _sort_key(r: sqlite3.Row) -> int:
                m = re.search(r"\d+", r["id"] or "")
                return int(m.group(0)) if m else 999999

            sorted_bug_rows = sorted(bug_rows, key=_sort_key)

            worms: List[WormReportModel] = []
            for row in sorted_bug_rows:
                steps_raw = row["reproduction_steps"]
                try:
                    steps = json.loads(steps_raw) if steps_raw else []
                except (json.JSONDecodeError, TypeError):
                    steps = []

                b_uuid = row["uuid"] if ("uuid" in row.keys() and row["uuid"]) else str(uuid_pkg.uuid4())
                try:
                    category_val = WormCategory(row["category"])
                except ValueError:
                    category_val = WormCategory.GENERAL
                bug = WormReportModel(
                    id=row["id"],
                    uuid=b_uuid,
                    title=row["title"],
                    status=WormStatus(row["status"]),
                    severity=WormSeverity(row["severity"]),
                    category=category_val,
                    component=row["component"] or "",
                    description=row["description"] or "",
                    reproduction_steps=steps,
                    expected_behavior=row["expected_behavior"] or "",
                    actual_behavior=row["actual_behavior"] or "",
                    logs=row["logs"] or "",
                    attachments=attachments_by_bug.get(row["id"], []),
                    created_at=row["created_at"] or "",
                    updated_at=row["updated_at"] or "",
                    resolved_at=row["resolved_at"],
                    resolution_notes=row["resolution_notes"] or "",
                )
                worms.append(bug)

            return WormDatabaseModel(
                title=title,
                summary=summary,
                worms=worms,
                updated_at=updated_at,
            )

    def save_database(self, database: WormDatabaseModel) -> None:
        """Atomically persist entire WormDatabaseModel into SQLite.

        Args:
            database: The bug database model to store.
        """
        with self._get_connection() as conn:
            # 1. Update metadata
            conn.execute(
                "INSERT INTO metadata (key, value) VALUES ('title', ?) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                (database.title,),
            )
            conn.execute(
                "INSERT INTO metadata (key, value) VALUES ('summary', ?) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                (database.summary,),
            )
            conn.execute(
                "INSERT INTO metadata (key, value) VALUES ('updated_at', ?) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                (database.updated_at,),
            )

            # 2. Track existing bug IDs to purge removed bugs
            current_ids = {b.id for b in database.worms}
            placeholders = ",".join("?" for _ in current_ids) if current_ids else "''"
            conn.execute(f"DELETE FROM worms WHERE id NOT IN ({placeholders})", list(current_ids))

            # 3. Upsert bugs and attachments
            for bug in database.worms:
                self._upsert_worm_in_conn(conn, bug)

    def save_worm(self, bug: WormReportModel) -> None:
        """Upsert a single worm report and its attachments.

        Args:
            bug: The worm report model to save.
        """
        with self._get_connection() as conn:
            self._upsert_worm_in_conn(conn, bug)

    def _upsert_worm_in_conn(self, conn: sqlite3.Connection, bug: WormReportModel) -> None:
        """Upsert a single bug in an active database connection."""
        steps_json = json.dumps(bug.reproduction_steps)
        b_uuid = bug.uuid or str(uuid_pkg.uuid4())
        conn.execute(
            """
            INSERT INTO worms (
                id, uuid, title, status, severity, category, component, description,
                reproduction_steps, expected_behavior, actual_behavior, logs,
                created_at, updated_at, resolved_at, resolution_notes
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                uuid=COALESCE(excluded.uuid, worms.uuid),
                title=excluded.title,
                status=excluded.status,
                severity=excluded.severity,
                category=excluded.category,
                component=excluded.component,
                description=excluded.description,
                reproduction_steps=excluded.reproduction_steps,
                expected_behavior=excluded.expected_behavior,
                actual_behavior=excluded.actual_behavior,
                logs=excluded.logs,
                created_at=excluded.created_at,
                updated_at=excluded.updated_at,
                resolved_at=excluded.resolved_at,
                resolution_notes=excluded.resolution_notes
            """,
            (
                bug.id,
                b_uuid,
                bug.title,
                bug.status.value,
                bug.severity.value,
                bug.category.value,
                bug.component,
                bug.description,
                steps_json,
                bug.expected_behavior,
                bug.actual_behavior,
                bug.logs,
                bug.created_at,
                bug.updated_at,
                bug.resolved_at,
                bug.resolution_notes,
            ),
        )

        # Sync attachments
        conn.execute("DELETE FROM worm_attachments WHERE worm_id = ?", (bug.id,))
        for att in bug.attachments:
            conn.execute(
                """
                INSERT INTO worm_attachments (
                    id, worm_id, filename, file_type, file_path, size_bytes, description, created_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT(id) DO UPDATE SET
                    worm_id=excluded.worm_id,
                    filename=excluded.filename,
                    file_type=excluded.file_type,
                    file_path=excluded.file_path,
                    size_bytes=excluded.size_bytes,
                    description=excluded.description,
                    created_at=excluded.created_at
                """,
                (
                    att.id,
                    bug.id,
                    att.filename,
                    att.file_type,
                    att.file_path,
                    att.size_bytes,
                    att.description,
                    att.created_at,
                ),
            )

    def delete_worm(self, worm_id: str) -> None:
        """Delete a bug and its associated attachments.

        Args:
            worm_id: Unique bug identifier.
        """
        with self._get_connection() as conn:
            conn.execute("DELETE FROM worms WHERE id = ?", (worm_id,))

    def import_from_json(self, json_path: Path) -> WormDatabaseModel:
        """Import database from a JSON state file into SQLite.

        Args:
            json_path: Path to the JSON state file.

        Returns:
            The loaded and imported WormDatabaseModel.
        """
        if not json_path.exists():
            raise FileNotFoundError(f"State file not found at {json_path}")
        with open(json_path, "r", encoding="utf-8") as f:
            data = json.load(f)
        db = WormDatabaseModel.model_validate(data)
        self.save_database(db)
        return db

    def export_to_json(self, json_path: Path) -> None:
        """Export SQLite contents to a JSON state file.

        Args:
            json_path: Path to write the JSON state file.
        """
        db = self.load_database()
        json_path.parent.mkdir(parents=True, exist_ok=True)
        with open(json_path, "w", encoding="utf-8") as f:
            json.dump(db.model_dump(mode="json"), f, indent=2)

    def generate_next_worm_id(self) -> str:
        """Query maximum bug ID directly from SQLite database and return next unique global ID."""
        with self._get_connection() as conn:
            rows = conn.execute("SELECT id FROM worms WHERE id LIKE 'WORM-%'").fetchall()
            max_idx = 0
            for r in rows:
                try:
                    idx = int(r["id"][4:])
                    if idx > max_idx:
                        max_idx = idx
                except ValueError:
                    pass
            return f"WORM-{max_idx + 1:03d}"

    def get_worm_by_uuid(self, worm_uuid: str) -> Optional[WormReportModel]:
        """Find a bug in SQLite by its unique UUID."""
        with self._get_connection() as conn:
            row = conn.execute(
                "SELECT id, uuid, title, status, severity, category, component, description, "
                "reproduction_steps, expected_behavior, actual_behavior, logs, created_at, "
                "updated_at, resolved_at, resolution_notes FROM worms WHERE uuid = ?",
                (worm_uuid,),
            ).fetchone()
            if not row:
                return None
            att_rows = conn.execute(
                "SELECT id, filename, file_type, file_path, size_bytes, description, created_at "
                "FROM worm_attachments WHERE worm_id = ? ORDER BY created_at ASC",
                (row["id"],),
            ).fetchall()
            attachments = [
                WormAttachmentModel(
                    id=ar["id"],
                    filename=ar["filename"],
                    file_type=ar["file_type"],
                    file_path=ar["file_path"],
                    size_bytes=ar["size_bytes"],
                    description=ar["description"],
                    created_at=ar["created_at"],
                )
                for ar in att_rows
            ]
            steps_raw = row["reproduction_steps"]
            try:
                steps = json.loads(steps_raw) if steps_raw else []
            except (json.JSONDecodeError, TypeError):
                steps = []
            try:
                category_val = WormCategory(row["category"])
            except ValueError:
                category_val = WormCategory.GENERAL
            return WormReportModel(
                id=row["id"],
                uuid=row["uuid"] or worm_uuid,
                title=row["title"],
                status=WormStatus(row["status"]),
                severity=WormSeverity(row["severity"]),
                category=category_val,
                component=row["component"] or "",
                description=row["description"] or "",
                reproduction_steps=steps,
                expected_behavior=row["expected_behavior"] or "",
                actual_behavior=row["actual_behavior"] or "",
                logs=row["logs"] or "",
                attachments=attachments,
                created_at=row["created_at"] or "",
                updated_at=row["updated_at"] or "",
                resolved_at=row["resolved_at"],
                resolution_notes=row["resolution_notes"] or "",
            )

    def update_worm_id(self, worm_uuid: str, new_id: str) -> None:
        """Update bug ID for an existing bug matching its unique UUID.

        Args:
            worm_uuid: Unique identifier of the bug.
            new_id: New bug ID to assign (e.g. WORM-199).
        """
        with self._get_connection() as conn:
            row = conn.execute("SELECT id FROM worms WHERE uuid = ?", (worm_uuid,)).fetchone()
            if not row:
                return
            old_id = row["id"]
            if old_id == new_id:
                return
            conn.execute("PRAGMA foreign_keys = OFF")
            conn.execute("UPDATE worms SET id = ? WHERE uuid = ?", (new_id, worm_uuid))
            conn.execute("UPDATE worm_attachments SET worm_id = ? WHERE worm_id = ?", (new_id, old_id))
            conn.execute("PRAGMA foreign_keys = ON")

    def resolve_duplicate_ids(self) -> Dict[str, str]:
        """Detect and automatically update duplicate bug IDs for different bugs in SQLite.

        Returns:
            Dictionary mapping bug UUID to new renumbered bug ID.
        """
        with self._get_connection() as conn:
            dups = conn.execute("SELECT id, COUNT(*) as cnt FROM worms GROUP BY id HAVING cnt > 1").fetchall()
            if not dups:
                return {}

            max_idx = 0
            all_ids = conn.execute("SELECT id FROM worms").fetchall()
            for r in all_ids:
                b_id = r["id"]
                if b_id.startswith("WORM-"):
                    try:
                        val = int(b_id[4:])
                        if val > max_idx:
                            max_idx = val
                    except ValueError:
                        pass

            reassigned: Dict[str, str] = {}
            for dup in dups:
                dup_id = dup["id"]
                rows = conn.execute(
                    "SELECT uuid, id, created_at FROM worms WHERE id = ? ORDER BY created_at ASC",
                    (dup_id,),
                ).fetchall()
                # Retain the first entry, reassign subsequent duplicates
                for row in rows[1:]:
                    max_idx += 1
                    new_id = f"WORM-{max_idx:03d}"
                    b_uuid = row["uuid"]
                    conn.execute("PRAGMA foreign_keys = OFF")
                    conn.execute("UPDATE worms SET id = ? WHERE uuid = ?", (new_id, b_uuid))
                    conn.execute("UPDATE worm_attachments SET worm_id = ? WHERE worm_id = ?", (new_id, dup_id))
                    conn.execute("PRAGMA foreign_keys = ON")
                    reassigned[b_uuid] = new_id

            return reassigned
