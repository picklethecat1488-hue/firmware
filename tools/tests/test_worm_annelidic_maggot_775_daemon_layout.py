"""Regression test for WORM-ANNELIDIC-MAGGOT-775: RTT and system log view don't appear when opening host daemon.

Asserts DOM hierarchy and unnested layout for workspace-body and target bug modal.
"""

from pathlib import Path
from bs4 import BeautifulSoup
import jinja2
import pytest


TEMPLATE_PATH = Path(__file__).resolve().parent.parent / "host_daemon" / "templates" / "host_daemon.html.j2"


def test_reproduction_worm_annelidic_maggot_775_workspace_body_not_nested_in_modal() -> None:
    """Verify WORM-ANNELIDIC-MAGGOT-775: workspace-body is a direct child of body, not trapped in targetBugModal."""
    assert TEMPLATE_PATH.exists(), f"Template not found at {TEMPLATE_PATH}"
    content = TEMPLATE_PATH.read_text(encoding="utf-8")

    soup = BeautifulSoup(content, "html.parser")
    workspace_body = soup.find("main", class_="workspace-body")
    assert workspace_body is not None, "Could not find <main class='workspace-body'> in template"

    # Invariant 1: workspace-body parent MUST be <body>, NOT #targetBugModal or #syncModal
    parent = workspace_body.parent
    assert parent.name == "body", (
        f"<main class='workspace-body'> parent is <{parent.name} id='{parent.get('id')}' class='{parent.get('class')}'>, "
        "expected parent to be <body>. It must not be nested inside any modal!"
    )

    # Invariant 2: targetBugModal must be closed before workspace-body starts
    target_bug_modal = soup.find("div", id="targetBugModal")
    assert target_bug_modal is not None, "Could not find <div id='targetBugModal'>"
    assert workspace_body not in target_bug_modal.descendants, (
        "<main class='workspace-body'> is inside targetBugModal descendants! "
        "When targetBugModal is display:none, workspace-body is hidden."
    )

    # Invariant 3: Rendered HTML with mock runner also keeps workspace-body directly under body
    env = jinja2.Environment(autoescape=False)
    template = env.from_string(content)
    rendered = template.render(
        runner={
            "package": "cat_detector",
            "bin_name": "cat_detector_shell",
            "target": "thumbv6m-none-eabi",
            "release": True,
            "speed": 1000,
            "active_mode": "CLI",
        }
    )
    rendered_soup = BeautifulSoup(rendered, "html.parser")
    rendered_wb = rendered_soup.find("main", class_="workspace-body")
    assert rendered_wb is not None
    assert rendered_wb.parent.name == "body"
    assert rendered_wb.find_parent(id="targetBugModal") is None
    assert rendered_wb.find_parent(id="syncModal") is None
