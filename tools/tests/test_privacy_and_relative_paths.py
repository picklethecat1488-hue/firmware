"""Regression unit test for WORM-016: Privacy, relative paths, and Windows support verification.

Guards against:
1. Leaking personal information (usernames, local user home directories, personal emails) in git-tracked files.
2. Embedding absolute file URLs or hardcoded local paths in documentation, configs, and skills.
3. Leaking personal information in bug/worm and code report feedback and tracebacks across Unix and Windows.
4. Hardcoding personal identifiers in test suites or production code.
"""

from datetime import datetime, timezone
import os
from pathlib import Path
import subprocess

from model.code_review import CommentModel, ReviewSessionModel, ReviewSeverity, ReviewStatus
from model.worm_report import WormCategory, WormDatabaseModel, WormReportModel, WormSeverity, WormStatus
from provider.code_review.markdown_exporter import MarkdownReviewExporter
from provider.sanitizer import (
    elide_commit_author,
    elide_commit_email,
    elide_personal_info,
    get_author_blacklist,
    get_email_blacklist,
    get_personal_identifiers,
    get_usernames_to_redact,
)
from provider.vcs.git_engine import GitEngine
from provider.worm_report.markdown_exporter import MarkdownWormExporter


WORKSPACE_ROOT = Path(__file__).resolve().parent.parent.parent


def test_dynamic_personal_identifiers_discovered() -> None:
    """Verify get_personal_identifiers dynamically derives identifiers without hardcoded data."""
    names, emails, handles = get_personal_identifiers()
    assert isinstance(names, set)
    assert isinstance(emails, set)
    assert isinstance(handles, set)
    # The active user's environment must yield discovered usernames/handles
    usernames = get_usernames_to_redact()
    assert len(usernames) > 0, "Expected at least one discovered username candidate"
    blacklist = get_author_blacklist()
    assert len(blacklist) > 0, "Expected non-empty author blacklist"


def test_no_personal_username_or_email_in_tracked_files() -> None:
    """Verify git tracked files do not contain personal usernames, real names, or personal emails."""
    terms: set[str] = set(get_usernames_to_redact())
    _, emails, _ = get_personal_identifiers()
    for e in emails:
        terms.add(e)

    for term in sorted(terms):
        if len(term) < 4:
            continue
        cmd = ["git", "grep", "-i", "-I", term]
        proc = subprocess.run(cmd, cwd=WORKSPACE_ROOT, capture_output=True, text=True)
        if proc.returncode == 0:
            violations = [
                line
                for line in proc.stdout.splitlines()
                if not any(
                    part in line
                    for part in (
                        "feedback/CR_",
                        "tools/dashboard/provider/sanitizer.py",
                        "tools/tests/test_privacy_and_relative_paths.py",
                    )
                )
            ]
            assert not violations, f"Found unredacted personal term '{term}' in tracked files:\n" + "\n".join(
                violations
            )


def test_elide_personal_info_tracebacks_and_paths() -> None:
    """Verify elide_personal_info redacts usernames and local home directories on POSIX systems."""
    usernames = sorted([u for u in get_usernames_to_redact() if " " not in u])
    test_user = usernames[0] if usernames else "testuser"
    traceback_sample = f"""
Traceback (most recent call last):
  File "/Users/{test_user}/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    return subprocess.run(cmd, check=True)
  File "/Users/{test_user}/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    stdout, stderr = process.communicate(input, timeout=timeout)
FileNotFoundError: [Errno 2] No such file or directory: 'pytest-of-{test_user}/pytest-123'
"""
    cleaned = elide_personal_info(traceback_sample)
    assert test_user not in cleaned
    assert "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py" in cleaned
    assert "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py" in cleaned
    assert "pytest-of-<username>/pytest-123" in cleaned


def test_elide_personal_info_windows_paths() -> None:
    """Verify elide_personal_info redacts Windows paths, backslashes, drive letters, and URLs."""
    usernames = sorted([u for u in get_usernames_to_redact() if " " not in u])
    test_user = usernames[0] if usernames else "testuser"
    win_sample = f"""
Traceback (most recent call last):
  File "C:\\Users\\{test_user}\\gh\\firmware\\tools\\runner.py", line 12, in <module>
    main()
  File "C:/Users/{test_user}/gh/firmware/tools/cli.py", line 45, in main
    test()
FileNotFoundError: [Errno 2] No such file or directory: 'C:\\Users\\{test_user}\\AppData\\Local\\Temp\\pytest-of-{test_user}\\test1'
Link: file:///C:/Users/{test_user}/gh/firmware/app/src/main.rs
Legacy: D:\\Documents and Settings\\{test_user}\\Desktop\\notes.txt
"""
    cleaned = elide_personal_info(win_sample)
    assert test_user not in cleaned
    assert r"C:\Users\<username>\gh\firmware\tools\runner.py" in cleaned
    assert "C:/Users/<username>/gh/firmware/tools/cli.py" in cleaned
    assert r"C:\Users\<username>\AppData\Local\Temp\pytest-of-<username>\test1" in cleaned
    assert "file:///C:/Users/<username>/gh/firmware/app/src/main.rs" in cleaned
    assert r"D:\Documents and Settings\<username>\Desktop\notes.txt" in cleaned


def test_skills_and_configs_use_relative_paths() -> None:
    """Verify skill files, cargo config, and documentation do not contain absolute file:///Users/ links."""
    usernames = [u for u in get_usernames_to_redact() if len(u) >= 4]

    # 1. Skill file
    skill_file = WORKSPACE_ROOT / ".agents" / "skills" / "cat-fountain-development" / "SKILL.md"
    assert skill_file.exists(), "SKILL.md must exist"
    skill_content = skill_file.read_text(encoding="utf-8")
    assert "file:///Users/" not in skill_content
    for u in usernames:
        assert u.lower() not in skill_content.lower()
    assert "[controller/controllers.toml](controller/controllers.toml)" in skill_content
    assert "[shell.toml](shell.toml)" in skill_content
    assert "Relative Paths & Privacy Hygiene Mandate" in skill_content

    # 2. Cargo config
    cargo_config = WORKSPACE_ROOT / ".cargo" / "config.toml"
    assert cargo_config.exists(), ".cargo/config.toml must exist"
    cargo_content = cargo_config.read_text(encoding="utf-8")
    assert "file:///Users/" not in cargo_content
    for u in usernames:
        assert u.lower() not in cargo_content.lower()
    assert 'runner = "tools/runner.sh"' in cargo_content

    # 3. Contributing & Gemini docs
    for doc_name in [
        "CONTRIBUTING.md",
        "GEMINI.md",
        "app/carrier_board.md",
        "app/cat_detector.md",
        "docs/hardware_firmware_codesign.md",
    ]:
        doc_path = WORKSPACE_ROOT / doc_name
        assert doc_path.exists(), f"{doc_name} must exist"
        doc_text = doc_path.read_text(encoding="utf-8")
        assert "file:///Users/" not in doc_text, f"{doc_name} must not contain file:///Users/ links"
        for u in usernames:
            assert u.lower() not in doc_text.lower(), f"{doc_name} must not contain personal username '{u}'"


def test_worm_exporter_automatically_elides_personal_info(tmp_path: Path) -> None:
    """Verify MarkdownWormExporter automatically sanitizes personal info when rendering and exporting."""
    usernames = sorted([u for u in get_usernames_to_redact() if " " not in u])
    test_user = usernames[0] if usernames else "testuser"
    exporter = MarkdownWormExporter(repo_root=tmp_path)
    worm = WormReportModel(
        id="WORM-999",
        uuid="99999999-9999-9999-9999-999999999999",
        title="Test Bug with Personal Info",
        status=WormStatus.OPEN,
        severity=WormSeverity.MEDIUM,
        category=WormCategory.FIRMWARE,
        description=f"Encountered failure under /Users/{test_user}/gh/firmware/tools/runner.sh",
        reproduction_steps=[f"Run pytest under /private/var/folders/pytest-of-{test_user}/test_1"],
        expected_behavior=f"Expected clean execution without leaking /Users/{test_user} paths",
        actual_behavior=f"Leaked /Users/{test_user}/miniforge3/env",
        logs=f'File "/Users/{test_user}/gh/firmware/tools/cli.py", line 10',
        resolution_notes=f"Fixed by {test_user}",
    )
    database = WormDatabaseModel(
        title="Privacy Test Database",
        summary=f"Summary mentioning /Users/{test_user}",
        worms=[worm],
    )

    rendered_worm = exporter.render_worm_markdown(worm)
    assert test_user not in rendered_worm
    assert "<username>" in rendered_worm

    rendered_db = exporter.render_markdown(database)
    assert test_user not in rendered_db
    assert "<username>" in rendered_db

    exported_file = tmp_path / "WORMS.md"
    exporter.export_markdown(database, exported_file, feedback_dir=tmp_path / "feedback")
    content = exported_file.read_text(encoding="utf-8")
    assert test_user not in content
    assert "<username>" in content

    exported_worm_file = tmp_path / "feedback" / "WORM_999.md"
    assert exported_worm_file.exists()
    worm_content = exported_worm_file.read_text(encoding="utf-8")
    assert test_user not in worm_content
    assert "<username>" in worm_content


def test_code_review_exporter_automatically_elides_personal_info(tmp_path: Path) -> None:
    """Verify MarkdownReviewExporter automatically sanitizes personal info when exporting."""
    usernames = sorted([u for u in get_usernames_to_redact() if " " not in u])
    test_user = usernames[0] if usernames else "testuser"
    exporter = MarkdownReviewExporter(repo_root=tmp_path)
    session = ReviewSessionModel(
        title="Privacy Review",
        verdict=ReviewStatus.CHANGES_REQUESTED,
        summary=f"Review by {test_user} for /Users/{test_user}/gh/firmware",
        comments=[
            CommentModel(
                id="c99",
                uuid="aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
                file_path="app/src/main.rs",
                start_line=1,
                end_line=5,
                severity=ReviewSeverity.MUST_FIX,
                body=f"Please check /Users/{test_user}/gh/firmware/app/src/main.rs",
                code_snippet=f"// Author: {test_user}\nlet path = '/Users/{test_user}';",
                created_at=datetime.now(timezone.utc).isoformat(),
                commit="12345678",
            )
        ],
    )

    rendered_cr = exporter.render_markdown(session)
    assert test_user not in rendered_cr
    assert "<username>" in rendered_cr

    out_file = tmp_path / "CR.md"
    exporter.export_markdown(session, out_file)
    content = out_file.read_text(encoding="utf-8")
    assert test_user not in content
    assert "<username>" in content

    commit_out = exporter.export_commit_markdown(session, "12345678", tmp_path / "feedback")
    assert commit_out is not None and commit_out.exists()
    commit_content = commit_out.read_text(encoding="utf-8")
    assert test_user not in commit_content
    assert "<username>" in commit_content


def test_elide_personal_info_redacts_discovered_names() -> None:
    """Verify elide_personal_info redacts discovered user names and handles."""
    names, _, _ = get_personal_identifiers()
    for name in names:
        if len(name) >= 3:
            sample = f"Customer Name: {name}\nReviewed by {name}."
            cleaned = elide_personal_info(sample)
            assert name not in cleaned
            assert "<username>" in cleaned


def test_elide_commit_author() -> None:
    """Verify elide_commit_author elides personal names and aliases to 'User'."""
    for author in get_author_blacklist():
        assert elide_commit_author(author) == "User"
    assert elide_commit_author("FLINT HEARTH-BINDER") == "FLINT HEARTH-BINDER"
    assert elide_commit_author("Local Working Tree") == "Local Working Tree"
    assert elide_commit_author("") == ""
    assert elide_commit_author(None) == ""


def test_elide_commit_email() -> None:
    """Verify elide_commit_email elides personal email addresses."""
    for email in get_email_blacklist():
        assert elide_commit_email(email) == "user@workspace"
    assert elide_commit_email("randomdeveloper@gmail.com") == "user@workspace"
    assert elide_commit_email("alternate@me.com") == "user@workspace"
    assert elide_commit_email("user@icloud.com") == "user@workspace"
    assert elide_commit_email("local@workspace") == "local@workspace"
    assert elide_commit_email("") == ""
    assert elide_commit_email(None) == ""


def test_git_engine_commits_and_smartlog_elide_personal_identity() -> None:
    """Verify GitEngine.get_commits and get_smartlog_dag elide personal author/email data."""
    engine = GitEngine(repo_root=WORKSPACE_ROOT)
    author_blacklist = {x.lower() for x in get_author_blacklist()}
    email_blacklist = {x.lower() for x in get_email_blacklist()}

    commits = engine.get_commits(limit=50)
    for c in commits:
        assert c.author.lower() not in author_blacklist, (
            f"Found unredacted author '{c.author}' in commit {c.short_hash}"
        )
        assert c.email.lower() not in email_blacklist, f"Found unredacted email '{c.email}' in commit {c.short_hash}"

    nodes = engine.get_smartlog_dag(limit=50)
    for n in nodes:
        assert n.author.lower() not in author_blacklist, (
            f"Found unredacted author '{n.author}' in smartlog node {n.short_hash}"
        )
        assert n.email.lower() not in email_blacklist, (
            f"Found unredacted email '{n.email}' in smartlog node {n.short_hash}"
        )


def test_regression_bug_loamy_bristle_455_elide_address_and_email() -> None:
    """Verify BUG-LOAMY-BRISTLE-455: Privacy sanitizer elides physical addresses and personal emails.

    Guards against:
    1. Leaking user email addresses and consumer domains (@gmail.com, @me.com, etc.).
    2. Leaking physical street addresses, suite/apt numbers, and P.O. Box locations.
    3. Eliding supplier and third-party delivery facility addresses without whitelist.
    4. Mapping user commit handles to 'User' and commit emails to 'user@workspace'.
    """
    # 1. Commit author & email redaction for user handle
    for h in get_author_blacklist():
        assert elide_commit_author(h) == "User"
    for e in get_email_blacklist():
        assert elide_commit_email(e) == "user@workspace"

    # 2. Email redaction in general text / feedback / logs
    sample_text = "Customer email: testuser@gmail.com\nAlternate: developer@me.com\nCompany contact: info@example.com\n"
    sanitized_text = elide_personal_info(sample_text)
    assert "testuser@gmail.com" not in sanitized_text
    assert "developer@me.com" not in sanitized_text
    assert "<email>" in sanitized_text

    # 3. Physical street address & P.O. Box redaction
    addr_samples = [
        "Please send RMA to 123 Main Street, Suite 400, San Francisco, CA 94105.",
        "Shipment returned to P.O. Box 789.",
        "Residential address: 456 Elm Ave, Apt 2B, Springfield, IL 62701.",
        "Office at 742 Evergreen Terrace, Unit 12.",
        "Address: 999 Oak Ridge Blvd, Austin, TX 78701.",
    ]
    for sample in addr_samples:
        cleaned = elide_personal_info(sample)
        assert "<address>" in cleaned, f"Expected '<address>' in: {cleaned} (from: {sample})"

    # 4. Supplier delivery facility address is also elided per review feedback
    factory_addr = (
        "Consignee: PCBWay SMT Assembly Team\n"
        "Address: Building 2, No. 1, Baolong 6th Road, Baolong Community,\n"
        "         Shenzhen, Guangdong, China\n"
        "Postal Code: 518116\n"
    )
    cleaned_factory = elide_personal_info(factory_addr)
    assert "<address>" in cleaned_factory
    assert "<zip>" in cleaned_factory

    # 5. File extensions like .zip should not be mangled as postal/ZIP codes
    file_list_sample = (
        "-gerbers.zip: contains .pcb .pcbdoc .cam .brd and gerber files\n"
        "-bom_templates.zip: contains BOM list for each PCB\n"
        "ZIP Code: 94043\n"
    )
    cleaned_file_list = elide_personal_info(file_list_sample)
    assert "-gerbers.zip: contains .pcb" in cleaned_file_list
    assert "-bom_templates.zip: contains BOM list" in cleaned_file_list
    assert "ZIP Code: <zip>" in cleaned_file_list
