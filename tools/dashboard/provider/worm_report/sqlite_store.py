"""SQLite backing store for worm report database.

Provides robust, concurrent, atomic persistence for worm reports,
reproduction steps, attachments, and tracker metadata using SQLite.
"""

from collections.abc import Collection
import contextlib
import json
from pathlib import Path
import re
import sqlite3
from typing import Any, Dict, Generator, List, Optional
import uuid as uuid_pkg

from model.id_generator import generate_docker_pattern_id
from model.worm_report import (
    WormAttachmentModel,
    WormCategory,
    WormDatabaseModel,
    WormReportModel,
    WormReviewTaskModel,
    WormReviewTaskStatus,
    WormSectionModel,
    WormSeverity,
    WormStatus,
)
from provider.sanitizer import elide_personal_info


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
            return
        for col_name, col_def in [
            ("commit_ids", "TEXT NOT NULL DEFAULT '[]'"),
            ("review_tasks", "TEXT NOT NULL DEFAULT '[]'"),
            ("rejection_notes", "TEXT NOT NULL DEFAULT ''"),
            ("accepted_at", "TEXT"),
            ("rejected_at", "TEXT"),
            ("custom_sections", "TEXT NOT NULL DEFAULT '[]'"),
        ]:
            if col_name not in cols:
                conn.execute(f"ALTER TABLE worms ADD COLUMN {col_name} {col_def}")
        cursor = conn.execute("SELECT 1 FROM sqlite_master WHERE type='table' AND name='file_mtimes'")
        if not cursor.fetchone():
            conn.execute("CREATE TABLE IF NOT EXISTS file_mtimes (filename TEXT PRIMARY KEY, mtime REAL NOT NULL)")

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
                commit_ids TEXT NOT NULL DEFAULT '[]',
                review_tasks TEXT NOT NULL DEFAULT '[]',
                created_at TEXT NOT NULL DEFAULT '',
                updated_at TEXT NOT NULL DEFAULT '',
                resolved_at TEXT,
                resolution_notes TEXT NOT NULL DEFAULT '',
                rejection_notes TEXT NOT NULL DEFAULT '',
                accepted_at TEXT,
                rejected_at TEXT,
                custom_sections TEXT NOT NULL DEFAULT '[]'
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

            CREATE TABLE IF NOT EXISTS file_mtimes (
                filename TEXT PRIMARY KEY,
                mtime REAL NOT NULL
            );
            """
        )
        # Ensure uuid column exists on preexisting databases
        cursor = conn.execute("PRAGMA table_info(worms)")
        cols = [row["name"] for row in cursor.fetchall()]
        if "uuid" not in cols:
            conn.execute("ALTER TABLE worms ADD COLUMN uuid TEXT")
        if "commit_ids" not in cols:
            conn.execute("ALTER TABLE worms ADD COLUMN commit_ids TEXT DEFAULT '[]'")
        if "review_tasks" not in cols:
            conn.execute("ALTER TABLE worms ADD COLUMN review_tasks TEXT DEFAULT '[]'")
        if "rejection_notes" not in cols:
            conn.execute("ALTER TABLE worms ADD COLUMN rejection_notes TEXT DEFAULT ''")
        if "accepted_at" not in cols:
            conn.execute("ALTER TABLE worms ADD COLUMN accepted_at TEXT")
        if "rejected_at" not in cols:
            conn.execute("ALTER TABLE worms ADD COLUMN rejected_at TEXT")

        # Backfill any missing UUIDs
        null_uuid_rows = conn.execute("SELECT id FROM worms WHERE uuid IS NULL OR uuid = ''").fetchall()
        for r in null_uuid_rows:
            conn.execute("UPDATE worms SET uuid = ? WHERE id = ?", (str(uuid_pkg.uuid4()), r["id"]))

        conn.execute("CREATE UNIQUE INDEX IF NOT EXISTS idx_worms_uuid ON worms(uuid)")

        # Check if legacy resolved worms have been migrated to CLOSED
        cursor = conn.execute("SELECT value FROM metadata WHERE key = 'legacy_resolved_migrated'")
        migrated_row = cursor.fetchone()
        if not migrated_row or migrated_row["value"] != "1":
            conn.execute(
                """
                UPDATE worms
                SET status = 'CLOSED',
                    accepted_at = COALESCE(NULLIF(resolved_at, ''), NULLIF(updated_at, ''), NULLIF(created_at, ''), CURRENT_TIMESTAMP)
                WHERE status = 'RESOLVED'
                  AND (accepted_at IS NULL OR accepted_at = '')
                  AND (resolved_at IS NULL OR resolved_at = '' OR resolved_at < '2026-11-05 14:00:00 UTC')
                """
            )
            conn.execute(
                "INSERT INTO metadata (key, value) VALUES ('legacy_resolved_migrated', '1') "
                "ON CONFLICT(key) DO UPDATE SET value = '1'"
            )

    def migrate_legacy_resolved(
        self,
        before_timestamp: Optional[str] = "2026-11-05 14:00:00 UTC",
        migrate_all: bool = False,
        exclude_ids: Optional[Collection[str]] = None,
        dry_run: bool = False,
    ) -> List[str]:
        """Migrate legacy resolved worms to CLOSED status with accepted_at populated.

        Args:
            before_timestamp: Cutoff timestamp (inclusive). Worms resolved before this are migrated.
                              Ignored if migrate_all is True.
            migrate_all: If True, migrate all unaccepted RESOLVED worms regardless of timestamp.
            exclude_ids: Optional collection of worm IDs to skip.
            dry_run: If True, return candidate IDs without modifying SQLite.

        Returns:
            List of migrated worm IDs.
        """
        exclude_set = {x.upper().strip() for x in (exclude_ids or []) if x.strip()}
        with self._get_connection() as conn:
            cursor = conn.execute(
                "SELECT id, resolved_at, updated_at, created_at FROM worms "
                "WHERE status = 'RESOLVED' AND (accepted_at IS NULL OR accepted_at = '')"
            )
            candidates = []
            for row in cursor.fetchall():
                w_id = row["id"]
                if w_id.upper() in exclude_set:
                    continue
                res_at = row["resolved_at"]
                if not migrate_all and before_timestamp:
                    check_ts = res_at or row["updated_at"] or row["created_at"]
                    if check_ts and check_ts > before_timestamp:
                        continue
                candidates.append((w_id, res_at or row["updated_at"] or row["created_at"]))

            if dry_run or not candidates:
                return [c[0] for c in candidates]

            for w_id, acc_at in candidates:
                conn.execute(
                    "UPDATE worms SET status = 'CLOSED', accepted_at = COALESCE(?, CURRENT_TIMESTAMP), "
                    "updated_at = COALESCE(updated_at, CURRENT_TIMESTAMP) WHERE id = ?",
                    (acc_at, w_id),
                )
            conn.execute(
                "INSERT INTO metadata (key, value) VALUES ('legacy_resolved_migrated', '1') "
                "ON CONFLICT(key) DO UPDATE SET value = '1'"
            )
            return [c[0] for c in candidates]

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
            # Load worms
            bug_rows = conn.execute(
                "SELECT id, uuid, title, status, severity, category, component, description, "
                "reproduction_steps, expected_behavior, actual_behavior, logs, commit_ids, review_tasks, "
                "created_at, updated_at, resolved_at, resolution_notes, rejection_notes, accepted_at, rejected_at, "
                "custom_sections FROM worms"
            ).fetchall()

            def _sort_key(r: sqlite3.Row) -> tuple[str, int, str]:
                created = r["created_at"] or ""
                m = re.search(r"\d+", r["id"] or "")
                num = int(m.group(0)) if m else 999999
                return (created if created else f"0000-{num:06d}", num, r["id"] or "")

            sorted_bug_rows = sorted(bug_rows, key=_sort_key)

            worms: List[WormReportModel] = []
            for row in sorted_bug_rows:
                steps_raw = row["reproduction_steps"]
                try:
                    steps = json.loads(steps_raw) if steps_raw else []
                except (json.JSONDecodeError, TypeError):
                    steps = []

                commit_ids_raw = row["commit_ids"] if "commit_ids" in row.keys() else "[]"
                try:
                    commit_ids = json.loads(commit_ids_raw) if commit_ids_raw else []
                except (json.JSONDecodeError, TypeError):
                    commit_ids = []

                tasks_raw = row["review_tasks"] if "review_tasks" in row.keys() else "[]"
                try:
                    tasks_data = json.loads(tasks_raw) if tasks_raw else []
                    review_tasks = [WormReviewTaskModel.model_validate(t) for t in tasks_data]
                except Exception:
                    review_tasks = []

                rej_notes = row["rejection_notes"] if "rejection_notes" in row.keys() and row["rejection_notes"] else ""
                acc_at = row["accepted_at"] if "accepted_at" in row.keys() else None
                rej_at = row["rejected_at"] if "rejected_at" in row.keys() else None

                custom_raw = (
                    row["custom_sections"] if ("custom_sections" in row.keys() and row["custom_sections"]) else "[]"
                )
                try:
                    custom_list = json.loads(custom_raw) if custom_raw else []
                    custom_sections = [WormSectionModel.model_validate(c) for c in custom_list]
                except Exception:
                    custom_sections = []

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
                    custom_sections=custom_sections,
                    commit_ids=commit_ids,
                    review_tasks=review_tasks,
                    created_at=row["created_at"] or "",
                    updated_at=row["updated_at"] or "",
                    resolved_at=row["resolved_at"],
                    resolution_notes=row["resolution_notes"] or "",
                    rejection_notes=rej_notes,
                    accepted_at=acc_at,
                    rejected_at=rej_at,
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
        desc_clean = elide_personal_info(bug.description)
        expected_clean = elide_personal_info(bug.expected_behavior)
        actual_clean = elide_personal_info(bug.actual_behavior)
        logs_clean = elide_personal_info(bug.logs)
        notes_clean = elide_personal_info(bug.resolution_notes)
        rejection_clean = elide_personal_info(bug.rejection_notes or "")
        steps_clean = [elide_personal_info(s) for s in bug.reproduction_steps]
        steps_json = json.dumps(steps_clean)
        commit_ids_json = json.dumps(bug.commit_ids)
        review_tasks_json = json.dumps([t.model_dump(mode="json") for t in bug.review_tasks])
        custom_clean = [
            {"title": elide_personal_info(s.title), "content": elide_personal_info(s.content)}
            for s in bug.custom_sections
        ]
        custom_json = json.dumps(custom_clean)
        b_uuid = bug.uuid or str(uuid_pkg.uuid4())
        existing_with_uuid = conn.execute(
            "SELECT id FROM worms WHERE uuid = ? AND id != ?", (b_uuid, bug.id)
        ).fetchone()
        if existing_with_uuid:
            b_uuid = str(uuid_pkg.uuid4())
            bug.uuid = b_uuid
        conn.execute(
            """
            INSERT INTO worms (
                id, uuid, title, status, severity, category, component, description,
                reproduction_steps, expected_behavior, actual_behavior, logs,
                commit_ids, review_tasks, created_at, updated_at, resolved_at,
                resolution_notes, rejection_notes, accepted_at, rejected_at, custom_sections
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
                commit_ids=excluded.commit_ids,
                review_tasks=excluded.review_tasks,
                created_at=excluded.created_at,
                updated_at=excluded.updated_at,
                resolved_at=excluded.resolved_at,
                resolution_notes=excluded.resolution_notes,
                rejection_notes=excluded.rejection_notes,
                accepted_at=excluded.accepted_at,
                rejected_at=excluded.rejected_at,
                custom_sections=excluded.custom_sections
            """,
            (
                bug.id,
                b_uuid,
                bug.title,
                bug.status.value,
                bug.severity.value,
                bug.category.value,
                bug.component,
                desc_clean,
                steps_json,
                expected_clean,
                actual_clean,
                logs_clean,
                commit_ids_json,
                review_tasks_json,
                bug.created_at,
                bug.updated_at,
                bug.resolved_at,
                notes_clean,
                rejection_clean,
                bug.accepted_at,
                bug.rejected_at,
                custom_json,
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
        """Generate next unique global Docker-pattern worm ID using CRNG (secrets/os.urandom)."""
        with self._get_connection() as conn:
            rows = conn.execute("SELECT id FROM worms").fetchall()
            existing_ids = {r["id"] for r in rows}
            return generate_docker_pattern_id(prefix="WORM", existing_ids=existing_ids)

    def get_worm_by_uuid(self, worm_uuid: str) -> Optional[WormReportModel]:
        """Find a worm in SQLite by its unique UUID."""
        with self._get_connection() as conn:
            row = conn.execute("SELECT id FROM worms WHERE uuid = ?", (worm_uuid,)).fetchone()
            if not row:
                return None
            return self.get_worm(row["id"])

    def get_worm(self, worm_id: str) -> Optional[WormReportModel]:
        """Fetch a single worm report by human-readable worm ID."""
        with self._get_connection() as conn:
            row = conn.execute("SELECT * FROM worms WHERE id = ?", (worm_id,)).fetchone()
            if not row:
                return None
            att_rows = conn.execute(
                "SELECT id, filename, file_type, file_path, size_bytes, description, created_at "
                "FROM worm_attachments WHERE worm_id = ? ORDER BY created_at ASC",
                (worm_id,),
            ).fetchall()
            attachments = [
                WormAttachmentModel(
                    id=ar["id"],
                    filename=ar["filename"],
                    file_type=ar["file_type"],
                    file_path=ar["file_path"],
                    size_bytes=ar["size_bytes"] or 0,
                    description=ar["description"] or "",
                    created_at=ar["created_at"] or "",
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

            commit_ids_raw = row["commit_ids"] if "commit_ids" in row.keys() else "[]"
            try:
                commit_ids = json.loads(commit_ids_raw) if commit_ids_raw else []
            except (json.JSONDecodeError, TypeError):
                commit_ids = []

            tasks_raw = row["review_tasks"] if "review_tasks" in row.keys() else "[]"
            try:
                tasks_data = json.loads(tasks_raw) if tasks_raw else []
                review_tasks = [WormReviewTaskModel.model_validate(t) for t in tasks_data]
            except Exception:
                review_tasks = []

            rej_notes = row["rejection_notes"] if "rejection_notes" in row.keys() and row["rejection_notes"] else None
            acc_at = row["accepted_at"] if "accepted_at" in row.keys() else None
            rej_at = row["rejected_at"] if "rejected_at" in row.keys() else None
            w_uuid = row["uuid"] if ("uuid" in row.keys() and row["uuid"]) else str(uuid_pkg.uuid4())

            return WormReportModel(
                id=row["id"],
                uuid=w_uuid,
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
                commit_ids=commit_ids,
                review_tasks=review_tasks,
                created_at=row["created_at"] or "",
                updated_at=row["updated_at"] or "",
                resolved_at=row["resolved_at"],
                resolution_notes=row["resolution_notes"] or "",
                rejection_notes=rej_notes,
                accepted_at=acc_at,
                rejected_at=rej_at,
            )

    get_bug = get_worm

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
            existing_target = conn.execute("SELECT uuid FROM worms WHERE id = ?", (new_id,)).fetchone()
            if existing_target and existing_target["uuid"] != worm_uuid:
                # Target ID already occupied in SQLite; do not crash on unique constraint
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

            all_ids = {r["id"] for r in conn.execute("SELECT id FROM worms").fetchall()}
            reassigned: Dict[str, str] = {}
            for dup in dups:
                dup_id = dup["id"]
                rows = conn.execute(
                    "SELECT uuid, id, created_at FROM worms WHERE id = ? ORDER BY created_at ASC",
                    (dup_id,),
                ).fetchall()
                # Retain the first entry, reassign subsequent duplicates
                for row in rows[1:]:
                    new_id = generate_docker_pattern_id(prefix="WORM", existing_ids=all_ids)
                    all_ids.add(new_id)
                    b_uuid = row["uuid"]
                    conn.execute("PRAGMA foreign_keys = OFF")
                    conn.execute("UPDATE worms SET id = ? WHERE uuid = ?", (new_id, b_uuid))
                    conn.execute("UPDATE worm_attachments SET worm_id = ? WHERE worm_id = ?", (new_id, dup_id))
                    conn.execute("PRAGMA foreign_keys = ON")
                    reassigned[b_uuid] = new_id

            return reassigned

    def get_file_mtimes(self) -> Dict[str, float]:
        """Retrieve cached modification times of scanned feedback files."""
        with self._get_connection() as conn:
            cursor = conn.execute("SELECT filename, mtime FROM file_mtimes")
            return {row["filename"]: float(row["mtime"]) for row in cursor.fetchall()}

    def update_file_mtimes(self, mtimes: Dict[str, float]) -> None:
        """Update cached modification times of scanned feedback files."""
        if not mtimes:
            return
        with self._get_connection() as conn:
            conn.executemany(
                "INSERT INTO file_mtimes (filename, mtime) VALUES (?, ?) ON CONFLICT(filename) DO UPDATE SET mtime=excluded.mtime",
                list(mtimes.items()),
            )

    def delete_file_mtime(self, filename: str) -> None:
        """Remove a deleted file from cached modification times."""
        with self._get_connection() as conn:
            conn.execute("DELETE FROM file_mtimes WHERE filename = ?", (filename,))
