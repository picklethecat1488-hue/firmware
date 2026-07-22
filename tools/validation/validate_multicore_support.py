#!/usr/bin/env python
import sys
import re
import os
import tree_sitter_rust as tsrust
from tree_sitter import Language, Parser
from colorama import init, Fore, Style
from halo import Halo

init(autoreset=True)

RUST_LANGUAGE = Language(tsrust.language())

# Whitelist of features to check for RAM linker attributes
SUPPORTED_FEATURES = ["motor-core", "sensors-core", "core1"]


def get_called_function_name(call_node):
    """Extract the function identifier being called from a call_expression node."""
    func_node = call_node.children[0]
    if func_node.type == "identifier":
        return func_node.text.decode("utf-8")
    elif func_node.type == "field_expression":
        for child in reversed(func_node.children):
            if child.type == "field_identifier":
                return child.text.decode("utf-8")
    elif func_node.type == "scoped_identifier":
        for child in reversed(func_node.children):
            if child.type == "identifier":
                return child.text.decode("utf-8")
    elif func_node.type == "generic_function":
        first_child = func_node.children[0]
        if first_child.type == "identifier":
            return first_child.text.decode("utf-8")
        elif first_child.type == "scoped_identifier":
            for child in reversed(first_child.children):
                if child.type == "identifier":
                    return child.text.decode("utf-8")
    return None


def parse_code(content, filepath="<string>"):
    """Parse Rust code using tree-sitter to find functions, attributes, and calls."""
    parser = Parser(RUST_LANGUAGE)
    tree = parser.parse(content)

    functions = {}

    def traverse(node):
        if node.type == "function_item":
            fn_name = None
            for child in node.children:
                if child.type in ["name", "identifier"]:
                    fn_name = child.text.decode("utf-8")
                    break

            if fn_name:
                parent = node.parent
                ram_features = set()
                if parent:
                    idx = -1
                    for i, child in enumerate(parent.children):
                        if child.id == node.id:
                            idx = i
                            break
                    k = idx - 1
                    while k >= 0:
                        sibling = parent.children[k]
                        if sibling.type == "attribute_item":
                            attr_text = sibling.text.decode("utf-8")
                            if "link_section" in attr_text and ".data.core1_func" in attr_text:
                                has_feature = False
                                for f in SUPPORTED_FEATURES:
                                    if f'feature = "{f}"' in attr_text or f"feature = '{f}'" in attr_text:
                                        ram_features.add(f)
                                        has_feature = True
                                if not has_feature:
                                    for f in SUPPORTED_FEATURES:
                                        ram_features.add(f)
                            k -= 1
                        elif sibling.type in ["line_comment", "block_comment", "\n"]:
                            k -= 1
                        else:
                            break

                calls = []
                forbidden_calls = []

                def find_calls_in_node(n):
                    if n.type == "call_expression":
                        if len(n.children) > 0:
                            func_node = n.children[0]
                            func_text = func_node.text.decode("utf-8")
                            if func_text in [
                                "cortex_m::interrupt::free",
                                "interrupt::free",
                                "free",
                                "cortex_m::register::primask::write",
                                "primask::write",
                                "primask::disable",
                                "cortex_m::interrupt::disable",
                                "interrupt::disable",
                            ]:
                                forbidden_calls.append((func_text, n.start_point[0] + 1))

                            # Check for forbidden flash or filesystem access
                            if any(
                                pattern in func_text
                                for pattern in [
                                    "sequential_storage",
                                    "FilesystemClient",
                                    "Flash",
                                ]
                            ):
                                forbidden_calls.append((f"flash/fs access ({func_text})", n.start_point[0] + 1))

                        name = get_called_function_name(n)
                        if name:
                            calls.append(name)
                    elif n.type == "method_call_expression":
                        for child in n.children:
                            if child.type == "field_identifier":
                                method_name = child.text.decode("utf-8")
                                calls.append(method_name)
                                if method_name == "erase":
                                    forbidden_calls.append(("flash erase method call", n.start_point[0] + 1))
                                break

                    if n != node and n.type == "function_item":
                        return

                    for child in n.children:
                        find_calls_in_node(child)

                for child in node.children:
                    if child.type == "block":
                        find_calls_in_node(child)

                functions[f"{filepath}:{fn_name}"] = {
                    "name": fn_name,
                    "filepath": filepath,
                    "line": node.start_point[0] + 1,
                    "ram_features": ram_features,
                    "calls": list(set(calls)),
                    "forbidden_calls": forbidden_calls,
                }

        for child in node.children:
            traverse(child)

    traverse(tree.root_node)
    return functions


def validate_call_graph(funcs_list, roots, feature, allowed_files=None):
    """Trace call graph from roots and check that reached functions have RAM placement attribute."""
    if allowed_files is not None:
        filtered_funcs = [
            f for f in funcs_list if any(os.path.basename(f["filepath"]) == f_name for f_name in allowed_files)
        ]
    else:
        filtered_funcs = funcs_list

    defs_by_name = {}
    for f in filtered_funcs:
        defs_by_name.setdefault(f["name"], []).append(f)

    visited = set()
    queue = list(roots)
    warnings = 0
    errors = 0

    while queue:
        curr_name = queue.pop(0)
        if curr_name in visited:
            continue
        visited.add(curr_name)

        if curr_name in defs_by_name:
            for d in defs_by_name[curr_name]:
                if (
                    d["name"] in ["new", "init", "bootstrap_core1_task"]
                    or d["name"].startswith("new_")
                    or d["name"].endswith("_init")
                ):
                    continue
                if feature not in d["ram_features"] and "core1" not in d["ram_features"]:
                    print(
                        f"{Fore.YELLOW}WARNING:{Style.RESET_ALL} Driver function '{curr_name}' in {d['filepath']}:{d['line']} is reached in RAM call chain but missing RAM attribute for '{feature}'!"
                    )
                    print(f'  Expected: #[cfg_attr(target_arch = "arm", link_section = ".data.core1_func")]')
                    print()
                    warnings += 1

                if "forbidden_calls" in d:
                    for forbidden_name, line_num in d["forbidden_calls"]:
                        print(
                            f"{Fore.RED}ERROR:{Style.RESET_ALL} Driver function '{curr_name}' in {d['filepath']}:{line_num} "
                            f"executes on Core 1 call path but calls single-core blocking/interrupt control '{forbidden_name}'!"
                        )
                        print("  Expected: Use critical_section::with() for multicore-safe synchronization.")
                        print()
                        errors += 1

                for child in d["calls"]:
                    if child not in visited:
                        queue.append(child)

    return warnings, errors


def main():
    scan_dirs = ["controller/src", "peripherals/src"]
    all_functions = []

    with Halo(text="Scanning and parsing AST for multicore support...", spinner="dots") as spinner:
        for s_dir in scan_dirs:
            if not os.path.exists(s_dir):
                continue
            for root, _, files in os.walk(s_dir):
                for file in files:
                    if file.endswith(".rs"):
                        filepath = os.path.join(root, file)
                        try:
                            with open(filepath, "rb") as f:
                                content = f.read()
                            funcs = parse_code(content, filepath)
                            all_functions.extend(funcs.values())
                        except Exception as e:
                            print(f"Error reading/parsing {filepath}: {e}", file=sys.stderr)

    # Validate motor-core call graph
    # Start from run, tick_motor, and update in motor_controller.rs
    motor_roots = ["run", "tick_motor", "update"]
    motor_warnings, motor_errors = validate_call_graph(
        funcs_list=all_functions,
        roots=motor_roots,
        feature="motor-core",
        allowed_files=["motor_controller.rs", "l9110s.rs", "ina219.rs"],
    )

    # Validate sensors-core call graph
    # Start from run, and update in sensor_controller.rs
    sensor_roots = ["run", "update"]
    sensor_warnings, sensor_errors = validate_call_graph(
        funcs_list=all_functions,
        roots=sensor_roots,
        feature="sensors-core",
        allowed_files=["sensor_controller.rs", "vl53l0x.rs"],
    )

    total_warnings = motor_warnings + sensor_warnings
    total_errors = motor_errors + sensor_errors

    if total_errors > 0:
        print(f"{Fore.RED}Validation FAILED: Found {total_errors} errors and {total_warnings} warnings.")
        sys.exit(1)
    elif total_warnings > 0:
        print(f"{Fore.YELLOW}Validation completed with {total_warnings} warnings.")
    else:
        print(f"{Fore.GREEN}Validation passed: All checks successful.")


if __name__ == "__main__":
    main()
