import os
import sys
import tempfile

# Ensure scripts directory is in path
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import validate_debug_derive


def test_compliant_enum_with_cfg_attr():
    code = """
    #[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
    #[derive(Clone, Copy)]
    pub enum Compliant {
        A,
        B,
    }
    """
    with tempfile.NamedTemporaryFile(suffix=".rs", delete=False) as tmp:
        tmp.write(code.encode("utf-8"))
        tmp_path = tmp.name
    try:
        errors = validate_debug_derive.validate_file(tmp_path)
        assert errors == 0
    finally:
        os.remove(tmp_path)


def test_non_compliant_enum_with_derive_debug():
    code = """
    #[derive(Debug, Clone, Copy)]
    pub enum NonCompliant {
        A,
        B,
    }
    """
    with tempfile.NamedTemporaryFile(suffix=".rs", delete=False) as tmp:
        tmp.write(code.encode("utf-8"))
        tmp_path = tmp.name
    try:
        errors = validate_debug_derive.validate_file(tmp_path)
        assert errors == 1
    finally:
        os.remove(tmp_path)


def test_enum_without_debug():
    code = """
    #[derive(Clone, Copy)]
    pub enum NoDebug {
        A,
        B,
    }
    """
    with tempfile.NamedTemporaryFile(suffix=".rs", delete=False) as tmp:
        tmp.write(code.encode("utf-8"))
        tmp_path = tmp.name
    try:
        errors = validate_debug_derive.validate_file(tmp_path)
        assert errors == 0
    finally:
        os.remove(tmp_path)


def test_struct_with_derive_debug_non_compliant():
    code = """
    #[derive(Debug)]
    pub struct NonCompliantStruct {
        a: u32,
    }
    """
    with tempfile.NamedTemporaryFile(suffix=".rs", delete=False) as tmp:
        tmp.write(code.encode("utf-8"))
        tmp_path = tmp.name
    try:
        errors = validate_debug_derive.validate_file(tmp_path)
        assert errors == 1
    finally:
        os.remove(tmp_path)


def test_compliant_struct_with_cfg_attr():
    code = """
    #[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
    pub struct CompliantStruct {
        a: u32,
    }
    """
    with tempfile.NamedTemporaryFile(suffix=".rs", delete=False) as tmp:
        tmp.write(code.encode("utf-8"))
        tmp_path = tmp.name
    try:
        errors = validate_debug_derive.validate_file(tmp_path)
        assert errors == 0
    finally:
        os.remove(tmp_path)


def test_multiline_derive_debug_non_compliant():
    code = """
    #[derive(
        Clone,
        Debug,
        Copy
    )]
    enum MultiLine {
        A,
    }
    """
    with tempfile.NamedTemporaryFile(suffix=".rs", delete=False) as tmp:
        tmp.write(code.encode("utf-8"))
        tmp_path = tmp.name
    try:
        errors = validate_debug_derive.validate_file(tmp_path)
        assert errors == 1
    finally:
        os.remove(tmp_path)
