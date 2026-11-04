import os
import sys
import tempfile
import json
from unittest.mock import patch, MagicMock

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import license_compliance


def test_check_licenses():
    """Verify that license allowlist checking identifies violations correctly."""
    allowlist = {"mit", "apache-2.0"}

    # 1. Compliant licenses
    licenses = {
        "pkg_a": ["mit"],
        "pkg_b": ["Apache-2.0"],
        "pkg_c": ["mit", "apache-2.0"],
    }
    violations = license_compliance.check_licenses(licenses, allowlist)
    assert not violations

    # 2. Non-compliant licenses
    licenses = {
        "pkg_a": ["gpl-3.0-only"],
        "pkg_b": ["mit", "unknown"],
        "pkg_c": ["unknown"],
        "pkg_d": [None],
    }
    violations = license_compliance.check_licenses(licenses, allowlist)
    assert "pkg_a" in violations
    assert violations["pkg_a"] == ["gpl-3.0-only"]
    assert "pkg_b" in violations
    assert violations["pkg_b"] == ["unknown"]
    assert "pkg_c" in violations
    assert violations["pkg_c"] == ["unknown"]
    assert "pkg_d" in violations
    assert violations["pkg_d"] == ["unknown"]


def test_parse_report_package_level():
    """Verify parsing scancode report package-level metadata."""
    report_data = {
        "packages": [
            {"name": "pkg_mit", "licenses": [{"license_key": "mit"}]},
            {"package_name": "pkg_apache", "licenses": [{"short_name": "Apache-2.0"}]},
            {"path": "pkg_other", "other_license": "gpl-3.0-only"},
            {"package_url": "pkg_url", "licenses": []},
        ]
    }

    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as f:
        json.dump(report_data, f)
        temp_name = f.name

    try:
        res = license_compliance.parse_report(temp_name)
        assert res["pkg_mit"] == ["mit"]
        assert res["pkg_apache"] == ["Apache-2.0"]
        assert res["pkg_other"] == ["gpl-3.0-only"]
        assert res["pkg_url"] == ["unknown"]
    finally:
        os.remove(temp_name)


def test_parse_report_file_fallback():
    """Verify parsing scancode report file-level fallback licenses."""
    report_data = {
        "packages": [],
        "files": [
            {"path": "src/lib.rs", "licenses": [{"key": "mit"}]},
            {"path": "src/main.rs", "licenses": [{"license_key": "apache-2.0"}]},
        ],
    }

    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as f:
        json.dump(report_data, f)
        temp_name = f.name

    try:
        res = license_compliance.parse_report(temp_name)
        assert res["src/lib.rs"] == ["mit"]
        assert res["src/main.rs"] == ["apache-2.0"]
    finally:
        os.remove(temp_name)


@patch("shutil.which")
@patch("subprocess.run")
def test_run_scancode_success(mock_run, mock_which):
    """Verify run_scancode triggers the system scancode command successfully."""
    mock_which.return_value = "/usr/local/bin/scancode"

    license_compliance.run_scancode(".", "report.json")

    mock_which.assert_called_once_with("scancode")
    mock_run.assert_called_once()

    # Assert env is passed with PYTHONWARNINGS=ignore
    called_args, called_kwargs = mock_run.call_args
    assert "PYTHONWARNINGS" in called_kwargs["env"]
    assert called_kwargs["env"]["PYTHONWARNINGS"] == "ignore"


@patch("shutil.which")
def test_run_scancode_not_found(mock_which):
    """Verify run_scancode fails with exit code 1 if scancode is not installed."""
    mock_which.return_value = None

    try:
        license_compliance.run_scancode(".", "report.json")
        assert False, "Should have exited"
    except SystemExit as e:
        assert e.code == 1


@patch("shutil.which")
@patch("subprocess.run")
def test_run_scancode_failure_surfaces_stderr(mock_run, mock_which, capsys):
    """Verify run_scancode prints stderr and exits with code 1 on subprocess failure."""
    import subprocess

    mock_which.return_value = "/usr/local/bin/scancode"
    mock_run.side_effect = subprocess.CalledProcessError(
        returncode=1,
        cmd=["scancode"],
        output="stdout details",
        stderr="ValueError: did not find any pre_scan hooks in module",
    )

    try:
        license_compliance.run_scancode(".", "report.json")
        assert False, "Should have exited with code 1"
    except SystemExit as e:
        assert e.code == 1

    captured = capsys.readouterr()
    assert "ValueError: did not find any pre_scan hooks" in captured.err


def test_code_scan_pins_pluggy_compatible_version():
    """Verify workflow and environment pin pluggy < 1.7.0 to prevent scancode plugin loader failures."""
    from pathlib import Path

    repo_root = Path(__file__).resolve().parent.parent.parent.parent
    workflow_path = repo_root / ".github" / "workflows" / "license-compliance.yml"
    env_path = repo_root / "environment.yml"

    assert workflow_path.exists(), "license-compliance.yml workflow missing"
    assert env_path.exists(), "environment.yml missing"

    workflow_text = workflow_path.read_text(encoding="utf-8")
    env_text = env_path.read_text(encoding="utf-8")

    assert "pluggy<1.7.0" in workflow_text, "Workflow must pin pluggy<1.7.0 for scancode-toolkit compatibility"
    assert "pluggy<1.7.0" in env_text, "environment.yml must pin pluggy<1.7.0 for scancode-toolkit compatibility"
