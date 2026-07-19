import os
import sys
import pytest

# Ensure scripts directory is in path
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import validate_ram_functions


def test_compliant_call_chain():
    code = """
    #[cfg_attr(all(target_arch = "arm", feature = "motor-core"), link_section = ".data.ram_func")]
    pub fn start() {
        process();
    }
    
    #[cfg_attr(all(target_arch = "arm", feature = "motor-core"), link_section = ".data.ram_func")]
    fn process() {
        let x = 1;
    }
    """
    funcs = validate_ram_functions.parse_code(code.encode("utf-8"), "test_file.rs")
    assert "test_file.rs:start" in funcs
    assert "test_file.rs:process" in funcs
    assert "motor-core" in funcs["test_file.rs:start"]["ram_features"]
    assert "motor-core" in funcs["test_file.rs:process"]["ram_features"]
    assert "process" in funcs["test_file.rs:start"]["calls"]

    warnings = validate_ram_functions.validate_call_graph(
        funcs_list=list(funcs.values()), roots=["start"], feature="motor-core"
    )
    assert warnings == 0


def test_missing_attribute_in_call_chain():
    code_missing = """
    #[cfg_attr(all(target_arch = "arm", feature = "motor-core"), link_section = ".data.ram_func")]
    pub fn start() {
        process();
    }
    
    fn process() {
        let x = 1;
    }
    """
    funcs_missing = validate_ram_functions.parse_code(code_missing.encode("utf-8"), "test_file_missing.rs")
    warnings_missing = validate_ram_functions.validate_call_graph(
        funcs_list=list(funcs_missing.values()), roots=["start"], feature="motor-core"
    )
    assert warnings_missing == 1
