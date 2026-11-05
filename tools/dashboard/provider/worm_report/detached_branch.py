"""Detached branch storage engine for repository worm tracking.

Decouples the bug database and markdown reviews from code feature branches
by committing directly to a dedicated git reference (refs/heads/worms) via headless
git plumbing without modifying the active working tree, index, or HEAD.
"""

from __future__ import annotations

import json
import os
from pathlib import Path
import subprocess
import tempfile
from typing import Any, Dict, List, Optional

from model.worm_report import WormDatabaseModel
from provider.sanitizer import elide_personal_info
from provider.worm_report.markdown_exporter import MarkdownWormExporter
from provider.worm_report.sqlite_store import SQLiteWormStore

DEFAULT_BRANCH_NAME: str = "worms"
FILE_MODE_STANDARD: str = "100644"


class DetachedBranchWormStore:
    """Manages worm tracking records stored on a dedicated, detached Git branch.

    Uses Git plumbing commands (hash-object, update-index, write-tree, commit-tree,
    update-ref) with an isolated temporary index file to read, write, and synchronize
    worm records completely out-of-tree without dirtying the user's working directory.
    """

    def __init__(self, repo_root: Path, branch_name: str = DEFAULT_BRANCH_NAME) -> None:
        """Initialize store for the specified repository and branch name.

        Args:
            repo_root: Root path of git repository.
            branch_name: Name of detached worm branch (defaults to 'worms').
        """
        self.repo_root = Path(repo_root).resolve()
        self.branch_name = branch_name
        self.ref_name = f"refs/heads/{branch_name}"
        self.remote_ref_name = f"refs/remotes/origin/{branch_name}"
        self.exporter = MarkdownWormExporter(self.repo_root)

    def _hash_object(self, data: bytes) -> str:
        """Create a git blob object directly in the git object database.

        Args:
            data: Raw byte content to hash and store.

        Returns:
            The SHA-1 object hash string.
        """
        proc = subprocess.run(
            ["git", "hash-object", "-w", "--stdin"],
            input=data,
            cwd=str(self.repo_root),
            capture_output=True,
            check=True,
        )
        return proc.stdout.decode("utf-8").strip()

    def ref_exists(self) -> bool:
        """Check if the local detached branch reference exists.

        Returns:
            True if the branch ref exists locally, False otherwise.
        """
        proc = subprocess.run(
            ["git", "rev-parse", "--verify", "--quiet", self.ref_name],
            cwd=str(self.repo_root),
            capture_output=True,
            check=False,
        )
        return proc.returncode == 0

    def get_ref_commit(self) -> Optional[str]:
        """Get the commit SHA of the local branch ref if it exists.

        Returns:
            Commit SHA string or None if reference does not exist.
        """
        proc = subprocess.run(
            ["git", "rev-parse", "--verify", "--quiet", self.ref_name],
            cwd=str(self.repo_root),
            capture_output=True,
            text=True,
            check=False,
        )
        if proc.returncode == 0 and proc.stdout.strip():
            return proc.stdout.strip()
        return None

    def get_remote_ref_commit(self) -> Optional[str]:
        """Get the commit SHA of the remote tracking branch ref if it exists.

        Returns:
            Commit SHA string or None if reference does not exist.
        """
        proc = subprocess.run(
            ["git", "rev-parse", "--verify", "--quiet", self.remote_ref_name],
            cwd=str(self.repo_root),
            capture_output=True,
            text=True,
            check=False,
        )
        if proc.returncode == 0 and proc.stdout.strip():
            return proc.stdout.strip()
        return None

    def load_database(self) -> Optional[WormDatabaseModel]:
        """Load WormDatabaseModel from the detached branch worms.json state file.

        Attempts to read worms.json from the local detached ref, then falls back
        to the remote tracking ref.

        Returns:
            WormDatabaseModel if successfully read and validated, else None.
        """
        for target_ref in [self.ref_name, self.remote_ref_name]:
            try:
                proc = subprocess.run(
                    ["git", "show", f"{target_ref}:{self.branch_name}.json"],
                    cwd=str(self.repo_root),
                    capture_output=True,
                    text=True,
                    check=False,
                )
                if proc.returncode == 0 and proc.stdout.strip():
                    data = json.loads(proc.stdout)
                    return WormDatabaseModel.model_validate(data)
            except Exception:
                continue
        return None

    def commit_database(
        self,
        database: WormDatabaseModel,
        commit_message: str = "Update worm tracker database",
        parents: Optional[List[str]] = None,
        extra_feedback_dir: Optional[Path] = None,
        sync_local_disk: bool = False,
    ) -> Optional[str]:
        """Write worms.json, WORMS.md, and individual WORM_*.md files to the detached branch.

        Args:
            database: Worm database model containing all worms to store.
            commit_message: Commit log message.
            parents: Optional list of parent commit hashes. If None, uses local ref commit.
            extra_feedback_dir: Optional directory to scan for additional feedback files (e.g. CR_*.md).
            sync_local_disk: If True, also writes exported files to local feedback/ directory.

        Returns:
            New commit SHA string or None if operation failed.
        """
        if parents is None:
            curr_parent = self.get_ref_commit()
            if not curr_parent:
                curr_parent = self.get_remote_ref_commit()
            parents = [curr_parent] if curr_parent else []

        git_dir = self.repo_root / ".git"
        if git_dir.is_dir():
            tmp_index_path = git_dir / f"idx_{self.branch_name}_{os.getpid()}"
            tmp_dir_obj = None
        else:
            tmp_dir_obj = tempfile.TemporaryDirectory(prefix="git_idx_dir_")
            tmp_index_path = Path(tmp_dir_obj.name) / "index"

        env = os.environ.copy()
        env["GIT_INDEX_FILE"] = str(tmp_index_path)
        env["GIT_OPTIONAL_LOCKS"] = "0"
        env["GIT_AUTHOR_NAME"] = "User"
        env["GIT_AUTHOR_EMAIL"] = "user@workspace"
        env["GIT_COMMITTER_NAME"] = "User"
        env["GIT_COMMITTER_EMAIL"] = "user@workspace"
        try:
            # If a parent commit exists, initialize the isolated index from its tree
            if parents and parents[0]:
                subprocess.run(
                    ["git", "read-tree", parents[0]],
                    cwd=str(self.repo_root),
                    env=env,
                    check=True,
                )

            # 1. Store worms.json
            data_json = database.model_dump_json(indent=2).encode("utf-8")
            blob_json = self._hash_object(data_json)
            subprocess.run(
                [
                    "git",
                    "update-index",
                    "--add",
                    "--cacheinfo",
                    FILE_MODE_STANDARD,
                    blob_json,
                    f"{self.branch_name}.json",
                ],
                cwd=str(self.repo_root),
                env=env,
                check=True,
            )

            # 2. Store WORMS.md
            data_md = self.exporter.render_markdown(database).encode("utf-8")
            blob_md = self._hash_object(data_md)
            subprocess.run(
                [
                    "git",
                    "update-index",
                    "--add",
                    "--cacheinfo",
                    FILE_MODE_STANDARD,
                    blob_md,
                    f"{self.branch_name.upper()}.md",
                ],
                cwd=str(self.repo_root),
                env=env,
                check=True,
            )

            # 3. Store individual WORM_<id>.md files in feedback/
            for bug in database.worms:
                bug_md = self.exporter.render_worm_markdown(bug).encode("utf-8")
                blob_bug = self._hash_object(bug_md)
                clean_id = bug.id.removeprefix("WORM-").removeprefix("WORM_").removeprefix("BUG-").removeprefix("BUG_")
                fn = f"feedback/WORM_{clean_id}.md"
                subprocess.run(
                    [
                        "git",
                        "update-index",
                        "--add",
                        "--cacheinfo",
                        FILE_MODE_STANDARD,
                        blob_bug,
                        fn,
                    ],
                    cwd=str(self.repo_root),
                    env=env,
                    check=True,
                )

            # 4. Include existing CR_*.md or review files if available
            feedback_path = extra_feedback_dir or (self.repo_root / "feedback")
            if feedback_path.exists() and feedback_path.is_dir():
                for cr_file in feedback_path.glob("CR_*.md"):
                    try:
                        cr_bytes = cr_file.read_bytes()
                        blob_cr = self._hash_object(cr_bytes)
                        subprocess.run(
                            [
                                "git",
                                "update-index",
                                "--add",
                                "--cacheinfo",
                                FILE_MODE_STANDARD,
                                blob_cr,
                                f"feedback/{cr_file.name}",
                            ],
                            cwd=str(self.repo_root),
                            env=env,
                            check=True,
                        )
                    except OSError:
                        continue

            # 5. Write index to tree object
            tree_proc = subprocess.run(
                ["git", "write-tree"],
                cwd=str(self.repo_root),
                env=env,
                capture_output=True,
                text=True,
                check=True,
            )
            tree_sha = tree_proc.stdout.strip()

            # 6. Commit tree object
            clean_message = elide_personal_info(commit_message)
            commit_cmd = ["git", "commit-tree", tree_sha, "-m", clean_message]
            for p in parents:
                if p:
                    commit_cmd.extend(["-p", p])
            commit_proc = subprocess.run(
                commit_cmd,
                cwd=str(self.repo_root),
                env=env,
                capture_output=True,
                text=True,
                check=True,
            )
            commit_sha = commit_proc.stdout.strip()

            # 7. Atomically update detached branch ref
            subprocess.run(
                ["git", "update-ref", self.ref_name, commit_sha],
                cwd=str(self.repo_root),
                check=True,
            )

            # 8. Optionally sync local feedback directory for local editor convenience
            if sync_local_disk:
                self.exporter.export_all_individual_worms(database, feedback_path)
                state_json_path = self.repo_root / "build" / f"{self.branch_name}.json"
                self.exporter.export_state_json(database, state_json_path)

            return commit_sha
        finally:
            if tmp_dir_obj is not None:
                try:
                    tmp_dir_obj.cleanup()
                except OSError:
                    pass
            else:
                if tmp_index_path.exists():
                    try:
                        tmp_index_path.unlink()
                    except OSError:
                        pass
                lock_p = tmp_index_path.with_name(tmp_index_path.name + ".lock")
                if lock_p.exists():
                    try:
                        lock_p.unlink()
                    except OSError:
                        pass

    def seed_from_workspace(
        self,
        sqlite_store: Optional[SQLiteWormStore] = None,
        force: bool = False,
    ) -> Optional[str]:
        """Seed detached branch from existing workspace database or feedback directory.

        Args:
            sqlite_store: Optional SQLiteWormStore to read existing bugs from.
            force: If True, re-seeds even if branch reference already exists.

        Returns:
            Commit SHA string if seeded, or existing commit SHA if already present.
        """
        if self.ref_exists() and not force:
            return self.get_ref_commit()

        db: Optional[WormDatabaseModel] = None
        if sqlite_store:
            try:
                db = sqlite_store.load_database()
            except Exception:
                db = None

        feedback_dir = self.repo_root / "feedback"
        if (not db or not db.worms) and feedback_dir.exists():
            db = WormDatabaseModel()
            self.exporter.scan_and_sync_feedback_dir(feedback_dir, db, sqlite_store)

        if db and db.worms:
            return self.commit_database(
                db,
                commit_message="Initialize detached worm tracking database",
                extra_feedback_dir=feedback_dir,
            )
        return None

    def sync_branch(self) -> Dict[str, Any]:
        """Reconcile detached worm branch with remote tracking ref.

        Fetches origin/worms, fast-forwards or merges when needed, and pushes
        updated commits back to origin.

        Returns:
            Dictionary with synchronization status, branch, and commit details.
        """
        has_remote = False
        try:
            remotes_proc = subprocess.run(
                ["git", "remote"],
                cwd=str(self.repo_root),
                capture_output=True,
                text=True,
                check=False,
            )
            remotes = [r.strip() for r in remotes_proc.stdout.splitlines()]
            has_remote = "origin" in remotes
        except Exception:
            has_remote = False

        if has_remote:
            subprocess.run(
                ["git", "fetch", "origin", f"{self.branch_name}:{self.remote_ref_name}"],
                cwd=str(self.repo_root),
                capture_output=True,
                check=False,
            )

        local_commit = self.get_ref_commit()
        remote_commit = self.get_remote_ref_commit()

        if not local_commit and remote_commit:
            subprocess.run(
                ["git", "update-ref", self.ref_name, remote_commit],
                cwd=str(self.repo_root),
                check=True,
            )
            local_commit = remote_commit
        elif local_commit and remote_commit and local_commit != remote_commit:
            is_ancestor = (
                subprocess.run(
                    ["git", "merge-base", "--is-ancestor", local_commit, remote_commit],
                    cwd=str(self.repo_root),
                    check=False,
                ).returncode
                == 0
            )
            if is_ancestor:
                subprocess.run(
                    ["git", "update-ref", self.ref_name, remote_commit],
                    cwd=str(self.repo_root),
                    check=True,
                )
                local_commit = remote_commit
            else:
                is_remote_ancestor = (
                    subprocess.run(
                        ["git", "merge-base", "--is-ancestor", remote_commit, local_commit],
                        cwd=str(self.repo_root),
                        check=False,
                    ).returncode
                    == 0
                )
                if not is_remote_ancestor:
                    # True divergence: merge databases
                    local_db = self.load_database()
                    remote_json_proc = subprocess.run(
                        ["git", "show", f"{remote_commit}:{self.branch_name}.json"],
                        cwd=str(self.repo_root),
                        capture_output=True,
                        text=True,
                        check=False,
                    )
                    if remote_json_proc.returncode == 0 and remote_json_proc.stdout.strip() and local_db:
                        remote_db = WormDatabaseModel.model_validate(json.loads(remote_json_proc.stdout))
                        merged_db = self.exporter.merge_databases(local_db, remote_db)
                        local_commit = self.commit_database(
                            merged_db,
                            commit_message="Merge remote worm database updates",
                            parents=[local_commit, remote_commit],
                        )

        pushed = False
        if has_remote and local_commit:
            push_proc = subprocess.run(
                ["git", "push", "origin", f"{self.ref_name}:{self.ref_name}"],
                cwd=str(self.repo_root),
                capture_output=True,
                text=True,
                check=False,
            )
            pushed = push_proc.returncode == 0

        return {
            "status": "ok",
            "branch": self.branch_name,
            "commit": local_commit,
            "remote_commit": remote_commit,
            "pushed": pushed,
        }
