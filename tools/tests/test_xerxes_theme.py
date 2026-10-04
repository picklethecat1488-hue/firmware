"""Unit and integration tests for System Shock 2 - Xerxes theme.

Validates CSS palette variables, CRT scanlines, cybernetic typography,
UNN Von Braun / AI primary core branding, Jinja2 template rendering,
and CLI terminal output formatting.
"""

from pathlib import Path
import jinja2
import pytest

from model.code_review import (
    CommentModel,
    FileReviewStatus,
    FileStateModel,
    ReviewSessionModel,
    ReviewSeverity,
    ReviewStatus,
)
from model.vcs import (
    BranchInfoModel,
    CommitNodeModel,
    DiffHunk,
    DiffLine,
    DiffLineType,
    DiffMode,
    DiffSideBySideRow,
    DiffViewSessionModel,
    FileDiffModel,
    WorkingTreeFileModel,
)
from model.worm_report import (
    WormCategory,
    WormDatabaseModel,
    WormReportModel,
    WormSeverity,
    WormStatus,
)
from dashboard.cli import print_cli_reviews, print_cli_smartlog, print_cli_worms


TEMPLATES_DIR = Path(__file__).resolve().parent.parent / "dashboard" / "templates"


def test_xerxes_css_variables_and_palette() -> None:
    """Verify all templates contain the System Shock 2 - Xerxes color palette tokens."""
    main_templates = [
        "diff_view.html.j2",
        "code_review.html.j2",
        "worm_report.html.j2",
    ]
    required_root_tokens = [
        "--xerxes-bg: #04090c",
        "--xerxes-panel: #08161d",
        "--xerxes-cyan: #00f0ff",
        "--xerxes-teal: #14d9c4",
        "--xerxes-amber: #ffb000",
        "--xerxes-red: #ff2a4b",
    ]

    for tpl_name in main_templates:
        tpl_path = TEMPLATES_DIR / tpl_name
        assert tpl_path.exists(), f"Template {tpl_name} must exist"
        content = tpl_path.read_text(encoding="utf-8")
        for token in required_root_tokens:
            assert token in content, f"Template {tpl_name} missing required Xerxes token: {token}"

    # Verify component template uses Xerxes variables
    diff_comp = (TEMPLATES_DIR / "diff_component.html.j2").read_text(encoding="utf-8")
    assert "--xerxes-cyan" in diff_comp
    assert "--xerxes-panel" in diff_comp
    assert "--xerxes-border-mid" in diff_comp


def test_xerxes_crt_scanlines_and_cybernetic_hud() -> None:
    """Verify scanline overlays, cybernetic fonts, and neon cyan glow shadows in templates."""
    diff_view = (TEMPLATES_DIR / "diff_view.html.j2").read_text(encoding="utf-8")
    cr_view = (TEMPLATES_DIR / "code_review.html.j2").read_text(encoding="utf-8")
    worm_view = (TEMPLATES_DIR / "worm_report.html.j2").read_text(encoding="utf-8")

    for content, name in [(diff_view, "diff_view"), (cr_view, "code_review"), (worm_view, "worm_report")]:
        # CRT scanline pattern on background
        assert "repeating-linear-gradient" in content, f"{name} must use repeating-linear-gradient for CRT scanlines"
        assert "radial-gradient" in content, f"{name} must use radial-gradient for cybernetic terminal backdrop"

        # Cybernetic Monospace typography
        assert "Share Tech Mono" in content, f"{name} must include Share Tech Mono cybernetic font"

        # AI terminal branding
        assert "XERXES" in content, f"{name} must include XERXES branding"
        assert "SYSTEM SHOCK 2" in content or "VON BRAUN" in content, (
            f"{name} must include System Shock 2 / Von Braun AI branding"
        )


def test_render_diff_view_template() -> None:
    """Verify diff_view.html.j2 renders cleanly with full mock DiffViewSessionModel."""
    env = jinja2.Environment(loader=jinja2.FileSystemLoader(str(TEMPLATES_DIR)))
    tpl = env.get_template("diff_view.html.j2")

    session = DiffViewSessionModel(
        repo_name="firmware-test",
        current_branch="main",
        head_commit_hash="a1b2c3d4e5f6",
        smartlog_nodes=[
            CommitNodeModel(
                commit_hash="a1b2c3d4e5f6",
                short_hash="a1b2c3d",
                date="2026-10-02T12:00:00Z",
                subject="Fix DMA overrun in SPI buffer",
                author="Xerxes AI",
                relative_date="10 minutes ago",
                branches=["main"],
                is_head=True,
            )
        ],
        working_files=[
            WorkingTreeFileModel(
                path="app/src/main.rs",
                status="M",
                is_staged=True,
                additions=12,
                deletions=3,
            )
        ],
        branches=[
            BranchInfoModel(
                name="main",
                commit_hash="a1b2c3d4e5f6",
                is_current=True,
            )
        ],
    )

    rendered = tpl.render(session=session)
    assert "<!DOCTYPE html>" in rendered
    assert "SYSTEM SHOCK 2" in rendered
    assert "firmware-test" in rendered
    assert "Fix DMA overrun in SPI buffer" in rendered
    assert "XERXES 2.0" in rendered


def test_render_code_review_template() -> None:
    """Verify code_review.html.j2 renders cleanly with mock ReviewSessionModel."""
    env = jinja2.Environment(loader=jinja2.FileSystemLoader(str(TEMPLATES_DIR)))
    tpl = env.get_template("code_review.html.j2")

    comment = CommentModel(
        id="c-999",
        file_path="controller/src/motor.rs",
        start_line=24,
        end_line=28,
        severity=ReviewSeverity.MUST_FIX,
        body="Verify interrupt priority configuration.",
        author="TriOptimum AI",
        created_at="2026-10-02T12:00:00Z",
    )

    session = ReviewSessionModel(
        title="Motor Controller DMA Verification",
        repo_name="firmware",
        commit_hash="f0e1d2c3b4a5",
        verdict=ReviewStatus.IN_REVIEW,
        comments=[comment],
        files={
            "controller/src/motor.rs": FileStateModel(
                path="controller/src/motor.rs",
                status=FileReviewStatus.PENDING,
            )
        },
    )

    rendered = tpl.render(session=session)
    assert "<!DOCTYPE html>" in rendered
    assert "Xerxes Code Review: firmware" in rendered
    assert "SYSTEM SHOCK 2 // XERXES CYBERNETIC CODE REVIEW" in rendered
    assert "IN_REVIEW" in rendered


def test_render_worm_report_template() -> None:
    """Verify worm_report.html.j2 renders cleanly with mock WormDatabaseModel."""
    env = jinja2.Environment(loader=jinja2.FileSystemLoader(str(TEMPLATES_DIR)))
    tpl = env.get_template("worm_report.html.j2")

    worm = WormReportModel(
        id="WORM-042",
        title="Fuel Gauge I2C Bus Hang Under Low Voltage",
        status=WormStatus.OPEN,
        severity=WormSeverity.HIGH,
        category=WormCategory.DRIVER,
        component="platform/max17048",
        description="I2C lines remain pulled low when cell drops below 3.0V.",
    )

    db = WormDatabaseModel(
        title="Firmware Telemetry & Anomaly Tracker",
        summary="UNN Von Braun Core Worm Management",
        worms=[worm],
    )

    rendered = tpl.render(database=db, database_json=db.model_dump_json())
    assert "<!DOCTYPE html>" in rendered
    assert "SYSTEM SHOCK 2 // XERXES WORM MATRIX" in rendered
    assert "WORM-042" in rendered
    assert "🪱" in rendered


def test_render_diff_component_template() -> None:
    """Verify diff_component.html.j2 renders side-by-side and unified diff hunks."""
    env = jinja2.Environment(loader=jinja2.FileSystemLoader(str(TEMPLATES_DIR)))
    tpl = env.get_template("diff_component.html.j2")

    file_diff = FileDiffModel(
        file_path="platform/src/lib.rs",
        mode=DiffMode.SIDE_BY_SIDE,
        side_by_side=[
            DiffSideBySideRow(
                old_no=1,
                new_no=1,
                old_text="pub fn init() {}",
                new_text="pub fn init_xerxes() {}",
                row_type="replace",
            )
        ],
        hunks=[
            DiffHunk(
                header="@@ -1,1 +1,1 @@",
                old_start=1,
                old_count=1,
                new_start=1,
                new_count=1,
                lines=[
                    DiffLine(type=DiffLineType.DELETE, content="pub fn init() {}"),
                    DiffLine(type=DiffLineType.ADD, content="pub fn init_xerxes() {}"),
                ],
            )
        ],
    )

    rendered = tpl.render(diff=file_diff)
    assert "diff-workstation-panel" in rendered
    assert "diff-toolbar" in rendered
    assert "diff-file-title" in rendered


def test_cli_xerxes_terminal_branding(capsys: pytest.CaptureFixture[str], tmp_path: Path) -> None:
    """Verify CLI smartlog, bugs, and reviews functions display authentic Xerxes terminal headers."""

    # 1. Test smartlog banner
    class DummyEngine:
        repo_root = tmp_path

        def get_current_branch(self) -> str:
            return "feature/xerxes-ai"

        def get_head_commit(self) -> str:
            return "1234567"

        def get_smartlog_dag(self, limit: int = 30) -> list:
            return []

        def get_working_tree_files(self) -> list:
            return []

        def get_merge_conflicts(self) -> list:
            return []

    print_cli_smartlog(DummyEngine())  # type: ignore[arg-type]
    out = capsys.readouterr().out
    assert "SYSTEM SHOCK 2 // XERXES SMARTLOG DAG TREE" in out
    assert "SECURITY PROTOCOLS ACTIVE // AI CONSOLE MONITORING COMMITS" in out

    # 2. Test reviews banner
    class DummyReviewServer:
        class DummySession:
            title = "Von Braun Sensor Scan"
            verdict = ReviewStatus.IN_REVIEW
            comments: list = []

            def count_by_severity(self) -> dict:
                return {}

        session = DummySession()

    class DummyServer:
        review_server = DummyReviewServer()

        class DummyWormServer:
            class DummyDatabase:
                title = "UNN Primary Core"
                worms: list = []

                def count_by_status(self) -> dict:
                    return {}

                def count_by_severity(self) -> dict:
                    return {}

            database = DummyDatabase()

        worm_server = DummyWormServer()

    print_cli_reviews(DummyServer())  # type: ignore[arg-type]
    out_rev = capsys.readouterr().out
    assert "SYSTEM SHOCK 2 // XERXES CODE REVIEW AUDIT" in out_rev

    print_cli_worms(DummyServer())  # type: ignore[arg-type]
    out_worm = capsys.readouterr().out
    assert "SYSTEM SHOCK 2 // XERXES WORM MATRIX" in out_worm
    assert "🪱" in out_worm
