#!/usr/bin/env python3
import sys
import re
import os
import tree_sitter_rust as tsrust
from tree_sitter import Language, Parser

RUST_LANGUAGE = Language(tsrust.language())

# Whitelist of features to check for RAM linker attributes
SUPPORTED_FEATURES = ["motor-core", "sensors-core"]


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
                            if "link_section" in attr_text and ".data.ram_func" in attr_text:
                                for f in SUPPORTED_FEATURES:
                                    if f in attr_text:
                                        ram_features.add(f)
                            k -= 1
                        elif sibling.type in ["line_comment", "block_comment", "\n"]:
                            k -= 1
                        else:
                            break

                calls = []

                def find_calls_in_node(n):
                    if n.type == "call_expression":
                        name = get_called_function_name(n)
                        if name:
                            calls.append(name)
                    elif n.type == "method_call_expression":
                        for child in n.children:
                            if child.type == "field_identifier":
                                calls.append(child.text.decode("utf-8"))
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
                if feature not in d["ram_features"]:
                    print(
                        f"WARNING: Driver function '{curr_name}' in {d['filepath']}:{d['line']} is reached in RAM call chain but missing RAM attribute for '{feature}'!"
                    )
                    print(
                        f'  Expected: #[cfg_attr(all(target_arch = "arm", feature = "{feature}"), link_section = ".data.ram_func")]'
                    )
                    print()
                    warnings += 1

                for child in d["calls"]:
                    if child not in visited:
                        queue.append(child)

    return warnings


def main():
    scan_dirs = ["controller/src", "peripherals/src"]
    all_functions = []

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
    motor_warnings = validate_call_graph(
        funcs_list=all_functions,
        roots=motor_roots,
        feature="motor-core",
        allowed_files=["motor_controller.rs", "l9110s.rs", "ina219.rs"],
    )

    # Validate sensors-core call graph
    # Start from run, and update in sensor_controller.rs
    sensor_roots = ["run", "update"]
    sensor_warnings = validate_call_graph(
        funcs_list=all_functions,
        roots=sensor_roots,
        feature="sensors-core",
        allowed_files=["sensor_controller.rs", "vl53l0x.rs"],
    )

    total_warnings = motor_warnings + sensor_warnings
    if total_warnings > 0:
        print(f"RAM placement check completed with {total_warnings} warnings.")
    else:
        print("RAM placement check passed: All critical execution path functions have RAM routing attributes.")


if __name__ == "__main__":
    main()
