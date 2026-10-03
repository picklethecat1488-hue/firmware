"""Markdown export engine for repository bug tracking.

Converts worm report database states, severities, attachments, and resolution notes
into a clean GitHub-flavored Markdown document (build/WORMS.md) with overview metrics,
action checklists, and embedded references.
"""

from datetime import datetime, timezone
import json
from pathlib import Path
import re
from typing import Any, Dict, List, Optional
import uuid as uuid_pkg

from model.worm_report import (
    WormAttachmentModel,
    WormCategory,
    WormDatabaseModel,
    WormReportModel,
    WormSeverity,
    WormStatus,
)


class MarkdownWormExporter:
    """Serializes worm tracker databases to GitHub-flavored Markdown and JSON."""

    def __init__(self, repo_root: Path) -> None:
        """Initialize exporter with repository base path.

        Args:
            repo_root: Base path of repository for file links.
        """
        self.repo_root = repo_root

    def export_markdown(
        self,
        database: WormDatabaseModel,
        output_path: Path,
        store: Optional[Any] = None,
        feedback_dir: Optional[Path] = None,
    ) -> Path:
        """Generate and save WORMS.md markdown document and individual WORM_<id>.md files.

        Args:
            database: Active worm database model.
            output_path: Destination path for WORMS.md file.
            store: Optional SQLiteWormStore to synchronize duplicate ID updates.
            feedback_dir: Optional destination directory for individual WORM_<id>.md files.

        Returns:
            Resolved Path where markdown was saved.
        """
        output_path.parent.mkdir(parents=True, exist_ok=True)

        # Detect and resolve duplicate IDs for different bugs (different UUIDs)
        seen_ids: Dict[str, str] = {}
        for bug in database.worms:
            if bug.id in seen_ids and seen_ids[bug.id] != bug.uuid:
                old_id = bug.id
                bug.id = database.generate_worm_id()
                if store and hasattr(store, "update_worm_id"):
                    store.update_worm_id(bug.uuid, bug.id)
            else:
                seen_ids[bug.id] = bug.uuid

        if store and hasattr(store, "resolve_duplicate_ids"):
            store.resolve_duplicate_ids()

        md_text = self.render_markdown(database)
        output_path.write_text(md_text, encoding="utf-8")

        # Export individual WORM_<id>.md files into feedback/ (or custom feedback_dir)
        target_feedback_dir = feedback_dir or (self.repo_root / "feedback")
        self.export_all_individual_worms(database, target_feedback_dir)

        return output_path

    def render_markdown(self, database: WormDatabaseModel) -> str:
        """Render worm database to structured GitHub-flavored Markdown.

        Args:
            database: Bug database model.

        Returns:
            Formatted Markdown document string.
        """
        now_str = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
        status_counts = database.count_by_status()
        severity_counts = database.count_by_severity()
        category_counts = database.count_by_category()

        total_worms = len(database.worms)
        open_worms = sum(1 for b in database.worms if b.status in (WormStatus.OPEN, WormStatus.IN_PROGRESS))
        resolved_worms = sum(1 for b in database.worms if b.status in (WormStatus.RESOLVED, WormStatus.CLOSED))
        resolved_pct = int((resolved_worms / total_worms) * 100) if total_worms > 0 else 100

        lines: List[str] = [
            f"# Worm Report Tracker: {database.title}",
            "",
            "> Automated bug tracking, triage, and issue registry generated via Firmware Bug Report Engine.",
            "",
            "## Tracker Overview",
            "",
            "| Metric | Details |",
            "| :--- | :--- |",
            f"| **Report Date** | `{now_str}` |",
            f"| **Total Issues** | `{total_worms}` |",
            f"| **Open Issues** | `{open_worms}` |",
            f"| **Resolved / Closed** | `{resolved_worms} ({resolved_pct}%)` |",
            "",
        ]

        if database.summary.strip():
            lines.extend(
                [
                    "## Executive Summary",
                    "",
                    database.summary.strip(),
                    "",
                ]
            )

        # Severity breakdown table
        lines.extend(
            [
                "## Issues by Severity",
                "",
                "| Severity | Count | Meaning |",
                "| :--- | :---: | :--- |",
                f"| **`[CRITICAL]`** | {severity_counts.get(WormSeverity.CRITICAL.value, 0)} | System crashes, build failures, blockages, or electrical shorts. |",
                f"| **`[HIGH]`** | {severity_counts.get(WormSeverity.HIGH.value, 0)} | Major functional defects, broken routing, DRC violations, or unphysical behavior. |",
                f"| **`[MEDIUM]`** | {severity_counts.get(WormSeverity.MEDIUM.value, 0)} | Silkscreen collisions, layout sub-optimality, or visual clipping. |",
                f"| **`[LOW]`** | {severity_counts.get(WormSeverity.LOW.value, 0)} | Minor aesthetic imperfections or documentation notes. |",
                "",
            ]
        )

        # Subsystem breakdown table
        lines.extend(
            [
                "## Issues by Category",
                "",
                "| Category | Count | Description |",
                "| :--- | :---: | :--- |",
            ]
        )
        category_descriptions = {
            WormCategory.FIRMWARE.value: "Core firmware logic, state machines, async tasks.",
            WormCategory.CONTROLLER.value: "Domain controllers, PID loops, event dispatch.",
            WormCategory.DRIVER.value: "Hardware peripheral drivers, embedded-hal.",
            WormCategory.PLATFORM.value: "Chip support, PAC, HAL, clock/power management.",
            WormCategory.BOARD.value: "Board support packages (BSP), pinmux, boards.",
            WormCategory.MODEL.value: "Domain data models, configurations, state definitions.",
            WormCategory.SHELL.value: "CLI interface, shell commands, debug console.",
            WormCategory.INFRASTRUCTURE.value: "Build tooling, compilers, test runners, headless tools.",
            WormCategory.UI.value: "Web dashboards, CLI viewers, review interfaces.",
            WormCategory.GENERAL.value: "Unclassified or cross-cutting firmware issues.",
        }
        for cat in WormCategory:
            desc = category_descriptions.get(cat.value, "Subsystem issues.")
            lines.append(f"| **`{cat.value}`** | {category_counts.get(cat.value, 0)} | {desc} |")
        lines.append("")

        # Issue checklist
        lines.extend(
            [
                "## Issue Checklist",
                "",
            ]
        )
        if not database.worms:
            lines.append("_No bugs registered in tracker._\n")
        else:
            for b in database.worms:
                chk = "x" if b.status in (WormStatus.RESOLVED, WormStatus.CLOSED) else " "
                comp_tag = f" `[{b.component}]`" if b.component else ""
                lines.append(
                    f"- [{chk}] **`[{b.severity.value}]`** [#{b.id}](#{b.id.lower()}): {b.title}{comp_tag} (`{b.status.value}`)"
                )
            lines.append("")

        # Detailed Issue Specifications
        lines.extend(
            [
                "## Detailed Issue Log",
                "",
            ]
        )

        for b in database.worms:
            status_icon = (
                "🟢"
                if b.status in (WormStatus.RESOLVED, WormStatus.CLOSED)
                else ("🟡" if b.status == WormStatus.IN_PROGRESS else "🔴")
            )
            lines.extend(
                [
                    f'### <a id="{b.id.lower()}"></a> {status_icon} `[{b.id}]` {b.title}',
                    "",
                    f"- **UUID**: `{b.uuid}`",
                    f"- **Status**: `{b.status.value}`",
                    f"- **Severity**: `{b.severity.value}`",
                    f"- **Category**: `{b.category.value}`",
                ]
            )
            if b.component:
                lines.append(f"- **Component**: `{b.component}`")
            if b.created_at:
                lines.append(f"- **Created**: `{b.created_at}`")
            if b.resolved_at:
                lines.append(f"- **Resolved**: `{b.resolved_at}`")
            lines.append("")

            if b.description.strip():
                lines.extend(
                    [
                        "#### Description",
                        "",
                        b.description.strip(),
                        "",
                    ]
                )

            if b.reproduction_steps:
                lines.extend(
                    [
                        "#### Reproduction Steps",
                        "",
                    ]
                )
                for step_idx, step in enumerate(b.reproduction_steps, 1):
                    lines.append(f"{step_idx}. {step}")
                lines.append("")

            if b.expected_behavior.strip() or b.actual_behavior.strip():
                lines.extend(
                    [
                        "#### Behavior Comparison",
                        "",
                        f"- **Expected**: {b.expected_behavior.strip() or '_Not specified_'}",
                        f"- **Actual**: {b.actual_behavior.strip() or '_Not specified_'}",
                        "",
                    ]
                )

            if b.logs.strip():
                lines.extend(
                    [
                        "#### Execution / Console Logs",
                        "",
                        "```text",
                        b.logs.strip(),
                        "```",
                        "",
                    ]
                )

            if b.attachments:
                lines.extend(
                    [
                        "#### Attachments & References",
                        "",
                        "| Type | Filename | Description |",
                        "| :--- | :--- | :--- |",
                    ]
                )
                for att in b.attachments:
                    lines.append(
                        f"| `{att.file_type}` | [{att.filename}]({att.file_path}) | {att.description or '_None_'} |"
                    )
                lines.append("")

            if b.resolution_notes.strip():
                lines.extend(
                    [
                        "#### Resolution Notes",
                        "",
                        b.resolution_notes.strip(),
                        "",
                    ]
                )

            lines.append("---\n")

        return "\n".join(lines).strip() + "\n"

    def export_state_json(self, database: WormDatabaseModel, state_path: Path) -> Path:
        """Persist worm database to JSON file.

        Args:
            database: Bug database model.
            state_path: Destination JSON file path.

        Returns:
            Resolved Path where JSON was saved.
        """
        state_path.parent.mkdir(parents=True, exist_ok=True)
        data = database.model_dump(mode="json")
        state_path.write_text(json.dumps(data, indent=2), encoding="utf-8")
        return state_path

    def load_state_json(self, state_path: Path) -> Optional[WormDatabaseModel]:
        """Load worm database from JSON file if it exists.

        Args:
            state_path: Path to JSON state file.

        Returns:
            WormDatabaseModel if file exists and is valid, else None.
        """
        if not state_path.exists():
            return None
        try:
            data = json.loads(state_path.read_text(encoding="utf-8"))
            return WormDatabaseModel.model_validate(data)
        except Exception:
            return None

    def parse_markdown(self, markdown_path: Path) -> Optional[WormDatabaseModel]:
        """Parse an existing WORMS.md file into a WormDatabaseModel.

        Args:
            markdown_path: Path to WORMS.md file.

        Returns:
            WormDatabaseModel if file exists and was parsed, else None.
        """
        if not markdown_path.exists():
            return None
        try:
            content = markdown_path.read_text(encoding="utf-8")
            return self.parse_markdown_text(content)
        except Exception:
            return None

    def parse_markdown_text(self, content: str) -> WormDatabaseModel:
        """Parse raw WORMS.md text content into a WormDatabaseModel.

        Args:
            content: Raw Markdown string.

        Returns:
            WormDatabaseModel populated with parsed issues.
        """
        title_match = re.search(r"^#\s+Worm Report Tracker:\s*(.*?)$", content, re.MULTILINE)
        title = title_match.group(1).strip() if title_match else "Firmware Bug Tracker"

        # Check list items for quick-action statuses: - [x] or - [ ]
        checklist_status: Dict[str, WormStatus] = {}
        for line in content.splitlines():
            chk_m = re.match(r"^-\s+\[([ xX])\]\s+.*?\b(WORM-\d+)\b", line)
            if chk_m:
                is_checked = chk_m.group(1).lower() == "x"
                chk_id = chk_m.group(2)
                checklist_status[chk_id] = WormStatus.RESOLVED if is_checked else WormStatus.OPEN

        db = WormDatabaseModel(title=title)

        # Split into bug sections by ### headers
        sections = re.split(r"\n(?=###\s+)", content)
        for sec in sections:
            sec_clean = sec.strip()
            if not sec_clean.startswith("###"):
                continue

            header_match = re.search(
                r'###\s+(?:<a id=".*?></a>\s*)?(?:[^\n\[]*?)?`\[(WORM-\d+)\]`\s*(.*?)$', sec_clean, re.MULTILINE
            )
            if not header_match:
                continue

            bug_id = header_match.group(1).strip()
            bug_title = header_match.group(2).strip()

            # Parse metadata lines
            uuid_val = ""
            m_uuid = re.search(r"-\s+\*\*UUID\*\*:\s*`?([0-9a-fA-F-]+)`?", sec_clean)
            if m_uuid:
                uuid_val = m_uuid.group(1).strip()

            status_val = WormStatus.OPEN
            m_status = re.search(r"-\s+\*\*Status\*\*:\s*`?([A-Za-z_]+)`?", sec_clean)
            if m_status:
                try:
                    status_val = WormStatus(m_status.group(1).strip().upper())
                except ValueError:
                    status_val = WormStatus.OPEN
            elif bug_id in checklist_status:
                status_val = checklist_status[bug_id]

            # If checklist explicitly checked [x], prefer RESOLVED unless CLOSED
            if checklist_status.get(bug_id) == WormStatus.RESOLVED and status_val == WormStatus.OPEN:
                status_val = WormStatus.RESOLVED

            severity_val = WormSeverity.MEDIUM
            m_sev = re.search(r"-\s+\*\*Severity\*\*:\s*`?([A-Za-z_]+)`?", sec_clean)
            if m_sev:
                try:
                    severity_val = WormSeverity(m_sev.group(1).strip().upper())
                except ValueError:
                    severity_val = WormSeverity.MEDIUM

            category_val = WormCategory.GENERAL
            m_cat = re.search(r"-\s+\*\*Category\*\*:\s*`?([A-Za-z_]+)`?", sec_clean)
            if m_cat:
                try:
                    category_val = WormCategory(m_cat.group(1).strip().upper())
                except ValueError:
                    category_val = WormCategory.GENERAL

            component_val = ""
            m_comp = re.search(r"-\s+\*\*Component\*\*:\s*`?([^\n`*]+)`?", sec_clean)
            if m_comp:
                component_val = m_comp.group(1).strip()

            created_val = ""
            m_created = re.search(r"-\s+\*\*Created\*\*:\s*`?([^\n`*]+)`?", sec_clean)
            if m_created:
                created_val = m_created.group(1).strip()

            resolved_val = None
            m_resolved = re.search(r"-\s+\*\*Resolved\*\*:\s*`?([^\n`*]+)`?", sec_clean)
            if m_resolved:
                resolved_val = m_resolved.group(1).strip()

            # Parse subsections: ####
            subsections = re.split(r"\n(?=####\s+)", sec_clean)
            desc = ""
            steps: List[str] = []
            expected = ""
            actual = ""
            logs = ""
            res_notes = ""
            attachments: List[WormAttachmentModel] = []

            for sub in subsections:
                sub_clean = sub.strip()
                if not sub_clean.startswith("####"):
                    continue

                sub_lines = sub_clean.splitlines()
                sub_title = sub_lines[0].replace("####", "").strip().lower()
                sub_body = "\n".join(sub_lines[1:]).strip()
                # Remove trailing divider --- if present
                if sub_body.endswith("---"):
                    sub_body = sub_body[:-3].strip()

                match sub_title:
                    case "description":
                        desc = sub_body
                    case "reproduction steps":
                        for step_line in sub_body.splitlines():
                            step_m = re.match(r"^\d+\.\s*(.*?)$", step_line.strip())
                            if step_m:
                                steps.append(step_m.group(1).strip())
                            elif step_line.strip().startswith("-"):
                                steps.append(step_line.strip().lstrip("-").strip())
                    case "expected behavior":
                        expected = sub_body
                    case "actual behavior":
                        actual = sub_body
                    case "execution / console logs":
                        cleaned_logs = sub_body
                        if cleaned_logs.startswith("```"):
                            first_nl = cleaned_logs.find("\n")
                            if first_nl != -1:
                                cleaned_logs = cleaned_logs[first_nl + 1 :]
                        if cleaned_logs.endswith("```"):
                            cleaned_logs = cleaned_logs[:-3]
                        logs = cleaned_logs.strip()
                    case "resolution notes" | "resolution":
                        res_notes = sub_body
                    case "attachments & references":
                        for att_line in sub_body.splitlines():
                            att_m = re.match(
                                r"^\|\s*`?(.*?)`?\s*\|\s*\[(.*?)\]\((.*?)\)\s*\|\s*(.*?)\s*\|$", att_line.strip()
                            )
                            if att_m:
                                ftype = att_m.group(1).strip()
                                fname = att_m.group(2).strip()
                                fpath = att_m.group(3).strip()
                                fdesc = att_m.group(4).strip()
                                if fdesc in ("_None_", "None"):
                                    fdesc = ""
                                attachments.append(
                                    WormAttachmentModel(
                                        id=f"att-{len(attachments) + 1}",
                                        filename=fname,
                                        file_type=ftype,
                                        file_path=fpath,
                                        description=fdesc,
                                    )
                                )

            bug = WormReportModel(
                id=bug_id,
                uuid=uuid_val or str(uuid_pkg.uuid4()),
                title=bug_title,
                status=status_val,
                severity=severity_val,
                category=category_val,
                component=component_val,
                created_at=created_val or datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC"),
                resolved_at=resolved_val,
                description=desc,
                reproduction_steps=steps,
                expected_behavior=expected,
                actual_behavior=actual,
                logs=logs,
                attachments=attachments,
                resolution_notes=res_notes,
            )
            db.add_or_update(bug)

        return db

    def merge_databases(self, base_db: WormDatabaseModel, md_db: WormDatabaseModel) -> WormDatabaseModel:
        """Merge markdown database changes into base database using Read-Modify-Write rules.

        Args:
            base_db: Current base database model (e.g. from SQLite).
            md_db: Database model parsed from WORMS.md.

        Returns:
            Updated base_db with all merged changes.
        """
        existing_map = {b.id: b for b in base_db.worms}

        for md_bug in md_db.worms:
            if md_bug.id not in existing_map:
                # Newly added bug in WORMS.md
                base_db.worms.append(md_bug)
                existing_map[md_bug.id] = md_bug
            else:
                existing = existing_map[md_bug.id]
                # Status update: protect resolved/closed bugs from being regressed to OPEN by stale markdown without notes
                if (
                    existing.status in (WormStatus.RESOLVED, WormStatus.CLOSED)
                    and md_bug.status == WormStatus.OPEN
                    and not md_bug.resolution_notes.strip()
                ):
                    pass
                elif md_bug.status != existing.status:
                    existing.status = md_bug.status
                    if md_bug.status in (WormStatus.RESOLVED, WormStatus.CLOSED):
                        if not existing.resolved_at:
                            existing.resolved_at = md_bug.resolved_at or datetime.now(timezone.utc).strftime(
                                "%Y-%m-%d %H:%M:%S UTC"
                            )
                    else:
                        existing.resolved_at = None

                # Non-empty title / severity / category / component updates
                if md_bug.title and md_bug.title != existing.title:
                    existing.title = md_bug.title
                if md_bug.severity != existing.severity:
                    existing.severity = md_bug.severity
                if md_bug.category != existing.category:
                    existing.category = md_bug.category
                if md_bug.component and md_bug.component != existing.component:
                    existing.component = md_bug.component

                # Description & notes
                if md_bug.description.strip() and md_bug.description.strip() != existing.description.strip():
                    existing.description = md_bug.description
                if (
                    md_bug.resolution_notes.strip()
                    and md_bug.resolution_notes.strip() != existing.resolution_notes.strip()
                ):
                    existing.resolution_notes = md_bug.resolution_notes

                # Reproduction steps
                if md_bug.reproduction_steps and md_bug.reproduction_steps != existing.reproduction_steps:
                    existing.reproduction_steps = md_bug.reproduction_steps

                # Behaviors & logs
                if (
                    md_bug.expected_behavior.strip()
                    and md_bug.expected_behavior.strip() != existing.expected_behavior.strip()
                ):
                    existing.expected_behavior = md_bug.expected_behavior
                if (
                    md_bug.actual_behavior.strip()
                    and md_bug.actual_behavior.strip() != existing.actual_behavior.strip()
                ):
                    existing.actual_behavior = md_bug.actual_behavior
                if md_bug.logs.strip() and md_bug.logs.strip() != existing.logs.strip():
                    existing.logs = md_bug.logs

                # Attachments: merge unique by file_path
                existing_paths = {a.file_path for a in existing.attachments}
                for att in md_bug.attachments:
                    if att.file_path not in existing_paths:
                        existing.attachments.append(att)
                        existing_paths.add(att.file_path)

        return base_db

    def render_worm_markdown(self, bug: WormReportModel) -> str:
        """Render a single worm report to Markdown.

        Args:
            bug: Bug report model to render.

        Returns:
            Formatted Markdown document string.
        """
        status_icon = (
            "🟢"
            if bug.status in (WormStatus.RESOLVED, WormStatus.CLOSED)
            else ("🟡" if bug.status == WormStatus.IN_PROGRESS else "🔴")
        )
        lines: List[str] = [
            f"# {status_icon} `[{bug.id}]` {bug.title}",
            "",
            f"- **UUID**: `{bug.uuid}`",
            f"- **ID**: `{bug.id}`",
            f"- **Status**: `{bug.status.value}`",
            f"- **Severity**: `{bug.severity.value}`",
            f"- **Category**: `{bug.category.value}`",
        ]
        if bug.component:
            lines.append(f"- **Component**: `{bug.component}`")
        if bug.created_at:
            lines.append(f"- **Created**: `{bug.created_at}`")
        if bug.resolved_at:
            lines.append(f"- **Resolved**: `{bug.resolved_at}`")
        lines.append("")

        if bug.description.strip():
            lines.extend(["#### Description", "", bug.description.strip(), ""])

        if bug.reproduction_steps:
            lines.extend(["#### Reproduction Steps", ""])
            for step_idx, step in enumerate(bug.reproduction_steps, 1):
                lines.append(f"{step_idx}. {step}")
            lines.append("")

        if bug.expected_behavior.strip() or bug.actual_behavior.strip():
            lines.extend(
                [
                    "#### Behavior Comparison",
                    "",
                    f"- **Expected**: {bug.expected_behavior.strip() or '_Not specified_'}",
                    f"- **Actual**: {bug.actual_behavior.strip() or '_Not specified_'}",
                    "",
                ]
            )

        if bug.logs.strip():
            lines.extend(["#### Execution / Console Logs", "", "```text", bug.logs.strip(), "```", ""])

        if bug.attachments:
            lines.extend(
                [
                    "#### Attachments & References",
                    "",
                    "| Type | Filename | Description |",
                    "| :--- | :--- | :--- |",
                ]
            )
            for att in bug.attachments:
                lines.append(
                    f"| `{att.file_type}` | [{att.filename}]({att.file_path}) | {att.description or '_None_'} |"
                )
            lines.append("")

        if bug.resolution_notes.strip():
            lines.extend(["#### Resolution Notes", "", bug.resolution_notes.strip(), ""])

        return "\n".join(lines).strip() + "\n"

    def export_individual_worm(self, bug: WormReportModel, feedback_dir: Path) -> Path:
        """Export a single bug to feedback/WORM_<id>.md.

        Args:
            bug: WormReportModel to export.
            feedback_dir: Directory where WORM_<id>.md is saved.

        Returns:
            Path to written markdown file.
        """
        feedback_dir.mkdir(parents=True, exist_ok=True)
        clean_id = bug.id.removeprefix("WORM-").removeprefix("WORM_")
        target_file = feedback_dir / f"WORM_{clean_id}.md"
        content = self.render_worm_markdown(bug)
        if target_file.exists():
            try:
                disk_content = target_file.read_text(encoding="utf-8")
                if disk_content == content:
                    return target_file
                # Protect resolved bug file on disk from being overwritten by un-noted OPEN bug
                if (
                    bug.status == WormStatus.OPEN
                    and not bug.resolution_notes.strip()
                    and "- **Status**: `RESOLVED`" in disk_content
                ):
                    return target_file
            except OSError:
                pass
        target_file.write_text(content, encoding="utf-8")
        return target_file

    def export_all_individual_worms(self, database: WormDatabaseModel, feedback_dir: Path) -> List[Path]:
        """Export all bugs in database to individual WORM_<id>.md files.

        Args:
            database: WormDatabaseModel with bugs to export.
            feedback_dir: Directory where files will be saved.

        Returns:
            List of Paths for exported bug markdown files.
        """
        feedback_dir.mkdir(parents=True, exist_ok=True)
        paths: List[Path] = []
        for bug in database.worms:
            paths.append(self.export_individual_worm(bug, feedback_dir))
        return paths

    def parse_individual_worm_file(self, file_path: Path) -> Optional[WormReportModel]:
        """Parse an individual WORM_<id>.md file into a WormReportModel.

        Args:
            file_path: Path to the bug markdown file.

        Returns:
            WormReportModel if valid, else None.
        """
        if not file_path.exists() or not file_path.is_file():
            return None
        try:
            content = file_path.read_text(encoding="utf-8")
            m_header = re.search(r"^#\s+(?:[^\n\[]*?)?`\[(WORM-[^\]]+|[^\]]+)\]`\s*(.*?)$", content, re.MULTILINE)
            m_fn = re.match(r"^BUG[_-](.+)\.md$", file_path.name, re.IGNORECASE)
            fn_id = ""
            if m_fn:
                fn_part = m_fn.group(1)
                if fn_part.isdigit():
                    fn_id = f"WORM-{int(fn_part):03d}"
                elif fn_part.startswith("WORM-"):
                    fn_id = fn_part
                else:
                    fn_id = f"WORM-{fn_part}"

            bug_id = fn_id
            bug_title = ""
            if m_header:
                bug_id = m_header.group(1).strip()
                bug_title = m_header.group(2).strip()

            if fn_id and (not bug_id or not bug_id.startswith("WORM-")):
                bug_id = fn_id

            m_id = re.search(r"-\s+\*\*ID\*\*:\s*`?([^\n`*]+)`?", content)
            if m_id and not bug_id:
                bug_id = m_id.group(1).strip()

            if not bug_id:
                bug_id = "WORM-000"

            m_uuid = re.search(r"-\s+\*\*UUID\*\*:\s*`?([0-9a-fA-F-]+)`?", content)
            uuid_val = m_uuid.group(1).strip() if m_uuid else str(uuid_pkg.uuid4())

            status_val = WormStatus.OPEN
            m_status = re.search(r"-\s+\*\*Status\*\*:\s*`?([A-Za-z_]+)`?", content)
            if m_status:
                try:
                    status_val = WormStatus(m_status.group(1).strip().upper())
                except ValueError:
                    status_val = WormStatus.OPEN

            severity_val = WormSeverity.MEDIUM
            m_sev = re.search(r"-\s+\*\*Severity\*\*:\s*`?([A-Za-z_]+)`?", content)
            if m_sev:
                try:
                    severity_val = WormSeverity(m_sev.group(1).strip().upper())
                except ValueError:
                    severity_val = WormSeverity.MEDIUM

            category_val = WormCategory.GENERAL
            m_cat = re.search(r"-\s+\*\*Category\*\*:\s*`?([A-Za-z_]+)`?", content)
            if m_cat:
                try:
                    category_val = WormCategory(m_cat.group(1).strip().upper())
                except ValueError:
                    category_val = WormCategory.GENERAL

            component_val = ""
            m_comp = re.search(r"-\s+\*\*Component\*\*:\s*`?([^\n`*]+)`?", content)
            if m_comp:
                component_val = m_comp.group(1).strip()

            created_val = ""
            m_created = re.search(r"-\s+\*\*Created\*\*:\s*`?([^\n`*]+)`?", content)
            if m_created:
                created_val = m_created.group(1).strip()

            resolved_val = None
            m_resolved = re.search(r"-\s+\*\*Resolved\*\*:\s*`?([^\n`*]+)`?", content)
            if m_resolved:
                resolved_val = m_resolved.group(1).strip()

            subsections = re.split(r"\n(?=####\s+)", content)
            desc = ""
            steps: List[str] = []
            expected = ""
            actual = ""
            logs = ""
            res_notes = ""
            attachments: List[WormAttachmentModel] = []

            for sub in subsections:
                sub_clean = sub.strip()
                if not sub_clean.startswith("####"):
                    continue
                sub_lines = sub_clean.splitlines()
                sub_title = sub_lines[0].replace("####", "").strip().lower()
                sub_body = "\n".join(sub_lines[1:]).strip()

                match sub_title:
                    case "description":
                        desc = sub_body
                    case "reproduction steps":
                        for step_line in sub_body.splitlines():
                            step_m = re.match(r"^\d+\.\s*(.*?)$", step_line.strip())
                            if step_m:
                                steps.append(step_m.group(1).strip())
                            elif step_line.strip().startswith("-"):
                                steps.append(step_line.strip().lstrip("-").strip())
                    case "behavior comparison":
                        exp_m = re.search(r"-\s+\*\*Expected\*\*:\s*(.*?)$", sub_body, re.MULTILINE)
                        act_m = re.search(r"-\s+\*\*Actual\*\*:\s*(.*?)$", sub_body, re.MULTILINE)
                        if exp_m:
                            expected = exp_m.group(1).strip()
                        if act_m:
                            actual = act_m.group(1).strip()
                    case "expected behavior":
                        expected = sub_body
                    case "actual behavior":
                        actual = sub_body
                    case "execution / console logs":
                        cleaned = sub_body
                        if cleaned.startswith("```"):
                            fnl = cleaned.find("\n")
                            if fnl != -1:
                                cleaned = cleaned[fnl + 1 :]
                        if cleaned.endswith("```"):
                            cleaned = cleaned[:-3]
                        logs = cleaned.strip()
                    case "resolution notes" | "resolution":
                        res_notes = sub_body
                    case "attachments & references":
                        for att_line in sub_body.splitlines():
                            att_m = re.match(
                                r"^\|\s*`?(.*?)`?\s*\|\s*\[(.*?)\]\((.*?)\)\s*\|\s*(.*?)\s*\|$", att_line.strip()
                            )
                            if att_m:
                                attachments.append(
                                    WormAttachmentModel(
                                        id=f"att-{len(attachments) + 1}",
                                        filename=att_m.group(2).strip(),
                                        file_type=att_m.group(1).strip(),
                                        file_path=att_m.group(3).strip(),
                                        description=""
                                        if att_m.group(4).strip() in ("_None_", "None")
                                        else att_m.group(4).strip(),
                                    )
                                )

            return WormReportModel(
                id=bug_id,
                uuid=uuid_val,
                title=bug_title or f"Bug {bug_id}",
                status=status_val,
                severity=severity_val,
                category=category_val,
                component=component_val,
                created_at=created_val or datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC"),
                resolved_at=resolved_val,
                description=desc,
                reproduction_steps=steps,
                expected_behavior=expected,
                actual_behavior=actual,
                logs=logs,
                attachments=attachments,
                resolution_notes=res_notes,
            )
        except Exception:
            return None

    def scan_and_sync_feedback_dir(
        self,
        feedback_dir: Path,
        database: WormDatabaseModel,
        store: Optional[Any] = None,
    ) -> Dict[str, Any]:
        """Scan feedback/ directory for individual WORM_*.md files, detect renames, and merge into database/SQLite.

        Args:
            feedback_dir: Path to feedback directory.
            database: Active worm database model.
            store: Optional SQLiteWormStore for atomic ID updates.

        Returns:
            Dictionary with sync summary stats.
        """
        if not feedback_dir.exists() or not feedback_dir.is_dir():
            return {"merged": 0, "renamed": 0, "scanned": 0}

        scanned = 0
        renamed = 0
        merged = 0

        for bug_file in sorted(list(feedback_dir.glob("WORM_*.md")) + list(feedback_dir.glob("BUG_*.md"))):
            scanned += 1
            md_bug = self.parse_individual_worm_file(bug_file)
            if not md_bug:
                continue

            # Determine ID implied by filename
            m_fn = re.match(r"^(?:WORM|BUG)[_-](.+)\.md$", bug_file.name, re.IGNORECASE)
            expected_id_from_fn = ""
            if m_fn:
                raw_part = m_fn.group(1)
                if raw_part.isdigit():
                    expected_id_from_fn = f"WORM-{int(raw_part):03d}"
                elif raw_part.startswith("WORM-"):
                    expected_id_from_fn = raw_part
                else:
                    expected_id_from_fn = f"WORM-{raw_part}"

            # Check if this bug already exists by UUID
            existing_by_uuid = database.get_worm_by_uuid(md_bug.uuid)
            if existing_by_uuid:
                # File name rename detection!
                if expected_id_from_fn and existing_by_uuid.id != expected_id_from_fn:
                    existing_by_uuid.id = expected_id_from_fn
                    md_bug.id = expected_id_from_fn
                    if store and hasattr(store, "update_worm_id"):
                        store.update_worm_id(existing_by_uuid.uuid, expected_id_from_fn)
                    renamed += 1
                if (
                    existing_by_uuid.status in (WormStatus.RESOLVED, WormStatus.CLOSED)
                    and md_bug.status == WormStatus.OPEN
                    and not md_bug.resolution_notes.strip()
                ):
                    pass
                else:
                    database.add_or_update(md_bug)
                merged += 1

            else:
                if expected_id_from_fn:
                    md_bug.id = expected_id_from_fn
                database.add_or_update(md_bug)
                merged += 1

        if store and hasattr(store, "save_database") and (merged > 0 or renamed > 0):
            store.save_database(database)

        return {"merged": merged, "renamed": renamed, "scanned": scanned}
