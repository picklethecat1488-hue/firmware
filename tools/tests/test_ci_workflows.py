"""Regression tests for CI gate workflow configuration."""

from pathlib import Path
import yaml

WORKSPACE_ROOT = Path(__file__).resolve().parent.parent.parent


def test_ci_gate_workflow_concurrency_and_triggers() -> None:
    """Verify ci-gate.yml has concurrency cancellation and ready_for_review trigger."""
    ci_gate_path = WORKSPACE_ROOT / ".github" / "workflows" / "ci-gate.yml"
    assert ci_gate_path.exists(), "ci-gate.yml must exist"

    raw_text = ci_gate_path.read_text(encoding="utf-8")
    data = yaml.safe_load(raw_text)

    # 1. Concurrency configuration
    assert "concurrency" in data, "ci-gate.yml must define concurrency"
    concurrency = data["concurrency"]
    assert concurrency.get("cancel-in-progress") is True, "concurrency must enable cancel-in-progress: true"
    assert "github.workflow" in str(concurrency.get("group", "")), "concurrency group must include workflow name"

    # 2. Event triggers must include ready_for_review
    on_dict = data.get("on") if "on" in data else data.get(True, {})
    on_pr = (on_dict or {}).get("pull_request", {})
    types = on_pr.get("types", [])
    assert "ready_for_review" in types, f"pull_request types must include 'ready_for_review', got: {types}"
    assert "opened" in types
    assert "synchronize" in types
    assert "reopened" in types


def test_environment_yml_and_workflows_support_async_device_harness() -> None:
    """Verify environment.yml includes pytest-asyncio and CI workflows lint/test tools/device_tests."""
    env_path = WORKSPACE_ROOT / "environment.yml"
    env_data = yaml.safe_load(env_path.read_text(encoding="utf-8"))
    deps = env_data.get("dependencies", [])
    pip_deps = []
    for entry in deps:
        if isinstance(entry, dict) and "pip" in entry:
            pip_deps.extend(str(x) for x in entry["pip"])

    assert any(p.startswith("pytest-asyncio") for p in pip_deps), (
        f"environment.yml must include pytest-asyncio in pip dependencies, got: {pip_deps}"
    )
    assert any(p.startswith("pluggy<1.7.0") for p in pip_deps)

    for wf_name in ("ci-gate.yml", "nightly.yml"):
        wf_text = (WORKSPACE_ROOT / ".github" / "workflows" / wf_name).read_text(encoding="utf-8")
        assert "tools/device_tests/" in wf_text, f"{wf_name} must lint tools/device_tests/"
        assert "test_device_harness.py" in wf_text, f"{wf_name} must run test_device_harness.py"
