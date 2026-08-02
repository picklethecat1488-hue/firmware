import os
import sys
import tempfile

# Ensure scripts directory is in path
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import validate_constant_addresses


def test_compliant_variable_cast():
    code = """
    fn test(sp: u32) {
        let stack_ptr = sp as *const u32;
    }
    """
    with tempfile.NamedTemporaryFile(suffix=".rs", delete=False) as tmp:
        tmp.write(code.encode("utf-8"))
        tmp_path = tmp.name
    try:
        errors = validate_constant_addresses.validate_file(tmp_path)
        assert errors == 0
    finally:
        os.remove(tmp_path)


def test_direct_hex_literal_cast():
    code = """
    fn reboot() {
        unsafe {
            core::ptr::write_volatile(0x40058000 as *mut u32, 1 << 31);
        }
    }
    """
    with tempfile.NamedTemporaryFile(suffix=".rs", delete=False) as tmp:
        tmp.write(code.encode("utf-8"))
        tmp_path = tmp.name
    try:
        errors = validate_constant_addresses.validate_file(tmp_path)
        assert errors == 1
    finally:
        os.remove(tmp_path)


def test_indirect_constant_cast():
    code = """
    const BASE: usize = 0x40000000;
    fn read() {
        let ptr = BASE as *const u16;
    }
    """
    with tempfile.NamedTemporaryFile(suffix=".rs", delete=False) as tmp:
        tmp.write(code.encode("utf-8"))
        tmp_path = tmp.name
    try:
        errors = validate_constant_addresses.validate_file(tmp_path)
        assert errors == 1
    finally:
        os.remove(tmp_path)


def test_whitelisted_path():
    code = """
    fn reboot() {
        unsafe {
            core::ptr::write_volatile(0x40058000 as *mut u32, 1 << 31);
        }
    }
    """
    with tempfile.NamedTemporaryFile(suffix=".rs", delete=False) as tmp:
        tmp.write(code.encode("utf-8"))
        tmp_path = tmp.name
    try:
        validate_constant_addresses.WHITELISTED_PATHS.append(tmp_path)
        errors = validate_constant_addresses.validate_file(tmp_path)
        assert errors == 0
    finally:
        validate_constant_addresses.WHITELISTED_PATHS.remove(tmp_path)
        os.remove(tmp_path)
