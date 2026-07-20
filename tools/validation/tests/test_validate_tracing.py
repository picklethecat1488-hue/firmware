import os
import sys
import pytest

# Ensure scripts directory is in path
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import validate_tracing


def test_is_boot_context():
    assert validate_tracing.is_boot_context({"name": "main", "is_instrumented": False}) is True
    assert validate_tracing.is_boot_context({"name": "new", "is_instrumented": False}) is True
    assert validate_tracing.is_boot_context({"name": "new_controller", "is_instrumented": False}) is True
    assert validate_tracing.is_boot_context({"name": "init", "is_instrumented": False}) is True
    assert validate_tracing.is_boot_context({"name": "init", "is_instrumented": True}) is False
    assert validate_tracing.is_boot_context({"name": "update", "is_instrumented": False}) is False


def test_parse_rs_file_valid_function(tmp_path):
    rs_content = """
    #[tracing::instrument(name = "test_fn")]
    pub async fn test_fn(
        a: u32,
    ) -> Result<(), ()> {
        let x = 12;
        Ok(())
    }
    """
    f_path = tmp_path / "valid.rs"
    f_path.write_text(rs_content)

    funcs = validate_tracing.parse_rs_file(str(f_path))
    assert len(funcs) == 1
    assert funcs[0]["name"] == "test_fn"
    assert funcs[0]["is_instrumented"] is True
    assert funcs[0]["is_embassy_task"] is False


def test_parse_rs_file_const_generics(tmp_path):
    rs_content = """
    #[tracing::instrument(name = "test_const")]
    pub fn test_const(
        x: Option<{ CHANNEL_CAPACITY }>,
    ) -> Result<(), ()> {
        Ok(())
    }
    """
    f_path = tmp_path / "const_gen.rs"
    f_path.write_text(rs_content)

    funcs = validate_tracing.parse_rs_file(str(f_path))
    assert len(funcs) == 1
    assert funcs[0]["name"] == "test_const"
    assert funcs[0]["is_instrumented"] is True


def test_parse_rs_file_uninstrumented(tmp_path):
    rs_content = """
    pub fn plain_fn() {
        // no-op
    }
    """
    f_path = tmp_path / "plain.rs"
    f_path.write_text(rs_content)

    funcs = validate_tracing.parse_rs_file(str(f_path))
    assert len(funcs) == 1
    assert funcs[0]["name"] == "plain_fn"
    assert funcs[0]["is_instrumented"] is False
