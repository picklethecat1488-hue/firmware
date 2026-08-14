#!/usr/bin/env python
import sys
import re
import os
import tree_sitter_rust as tsrust
from tree_sitter import Language, Parser
from colorama import init, Fore, Style
from halo import Halo
import glob

init(autoreset=True)

RUST_LANGUAGE = Language(tsrust.language())

# Whitelist of features to check for RAM linker attributes
SUPPORTED_FEATURES = ["motor-core", "sensors-core", "core1"]

# Boot entry points whitelisted to bypass cross-core caller checks
CROSS_CORE_FUNCS = {"new", "init", "bootstrap_core1_task"}


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


def get_struct_name(struct_node):
    """Extract the struct identifier name from a struct_item node."""
    for child in struct_node.children:
        if child.type == "type_identifier":
            return child.text.decode("utf-8")
    return None


def parse_impl_info(impl_text):
    """Extract trait name and struct name from impl header text."""
    impl_text = re.sub(r"//.*", "", impl_text)
    impl_text = re.sub(r"\s+", " ", impl_text)

    match_for = re.search(
        r"impl\s*(?:<[^>]+>)?\s*([a-zA-Z0-9_:]+)(?:<[^>]+>)?\s*for\s*([a-zA-Z0-9_:]+)",
        impl_text,
    )
    if match_for:
        return match_for.group(1), match_for.group(2)

    match_concrete = re.search(r"impl\s*(?:<[^>]+>)?\s*([a-zA-Z0-9_:]+)", impl_text)
    if match_concrete:
        return None, match_concrete.group(1)

    return None, None


def parse_code(content, filepath="<string>"):
    """Parse Rust code using tree-sitter to find functions, attributes, and calls."""
    parser = Parser(RUST_LANGUAGE)
    tree = parser.parse(content)

    functions = {}
    controller_structs = []

    def traverse(node):
        if node.type == "struct_item":
            is_controller_context = False
            parent = node.parent
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
                        if "controller_context" in attr_text:
                            is_controller_context = True
                        k -= 1
                    elif sibling.type in ["line_comment", "block_comment", "\n"]:
                        k -= 1
                    else:
                        break
            if is_controller_context:
                s_name = get_struct_name(node)
                if s_name:
                    core1_feature = None
                    core1_roots = None
                    # Search back for the controller_context attribute to parse its arguments
                    k = idx - 1
                    while k >= 0:
                        sibling = parent.children[k]
                        if sibling.type == "attribute_item":
                            attr_text = sibling.text.decode("utf-8")
                            if "controller_context" in attr_text:
                                match = re.search(r'core1_feature\s*=\s*"([^"]+)"', attr_text)
                                if match:
                                    core1_feature = match.group(1)
                                roots_match = re.search(r"core1_roots\s*=\s*\[([^\]]+)\]", attr_text)
                                if roots_match:
                                    core1_roots = [
                                        r.strip().strip('"').strip("'") for r in roots_match.group(1).split(",")
                                    ]
                                break
                        k -= 1
                    controller_structs.append(
                        {
                            "name": s_name,
                            "filepath": filepath,
                            "core1_feature": core1_feature,
                            "core1_roots": core1_roots,
                        }
                    )

        elif node.type == "function_item":
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

                parent_impl = None
                curr = node.parent
                while curr:
                    if curr.type == "impl_item":
                        parent_impl = curr
                        break
                    curr = curr.parent

                trait_name = None
                struct_name = None
                if parent_impl:
                    impl_text = parent_impl.text.decode("utf-8")
                    trait_name, struct_name = parse_impl_info(impl_text)

                functions[f"{filepath}:{fn_name}"] = {
                    "name": fn_name,
                    "filepath": filepath,
                    "line": node.start_point[0] + 1,
                    "ram_features": ram_features,
                    "calls": list(set(calls)),
                    "forbidden_calls": forbidden_calls,
                    "text": node.text.decode("utf-8"),
                    "trait_name": trait_name,
                    "struct_name": struct_name,
                }

        for child in node.children:
            traverse(child)

    traverse(tree.root_node)
    return functions, controller_structs


def validate_call_graph(funcs_list, roots, feature, root_files=None):
    """Trace call graph from roots and check that reached functions have RAM placement attribute."""
    # Build maps of definitions
    defs_by_name = {}
    defs_by_key = {}
    core1_structs = set()
    for f in funcs_list:
        defs_by_name.setdefault(f["name"], []).append(f)
        defs_by_key[f"{f['filepath']}:{f['name']}"] = f
        if f.get("struct_name") and (feature in f["ram_features"] or "core1" in f["ram_features"]):
            core1_structs.add(f["struct_name"])

    visited = set()
    queue = []

    # Initialize queue with root function keys matched by name and root_files
    for r in roots:
        if r in defs_by_name:
            for f in defs_by_name[r]:
                if root_files is None or any(os.path.basename(f["filepath"]) == rf for rf in root_files):
                    key = f"{f['filepath']}:{f['name']}"
                    queue.append(key)

    warnings = 0
    errors = 0
    parent_map = {}

    while queue:
        curr_key = queue.pop(0)
        if curr_key in visited:
            continue
        visited.add(curr_key)

        if curr_key in defs_by_key:
            d = defs_by_key[curr_key]
            if (
                d["name"] in ["new", "init", "bootstrap_core1_task"]
                or d["name"].startswith("new_")
                or d["name"].endswith("_init")
            ):
                continue
            if feature not in d["ram_features"] and "core1" not in d["ram_features"]:
                # Print the call path
                path = []
                step = curr_key
                while step in parent_map:
                    path.append(step.split(":")[-1])
                    step = parent_map[step]
                path.append(step.split(":")[-1])
                path.reverse()
                path_str = " -> ".join(path)

                print(
                    f"{Fore.RED}ERROR:{Style.RESET_ALL} Driver function '{d['name']}' in {d['filepath']}:{d['line']} is reached in RAM call chain but missing RAM attribute for '{feature}'!"
                )
                print(f"  Path: {path_str}")
                print(f'  Expected: #[cfg_attr(target_arch = "arm", link_section = ".data.core1_func")]')
                print()
                errors += 1

            if "forbidden_calls" in d:
                for forbidden_name, line_num in d["forbidden_calls"]:
                    print(
                        f"{Fore.RED}ERROR:{Style.RESET_ALL} Driver function '{d['name']}' in {d['filepath']}:{line_num} "
                        f"executes on Core 1 call path but calls single-core blocking/interrupt control '{forbidden_name}'!"
                    )
                    print("  Expected: Use critical_section::with() for multicore-safe synchronization.")
                    print()
                    errors += 1

            for child in d["calls"]:
                if child in defs_by_name:
                    # Prioritize definitions within the same source file to avoid cross-controller name collisions
                    local_defs = [child_f for child_f in defs_by_name[child] if child_f["filepath"] == d["filepath"]]
                    targets = local_defs if local_defs else defs_by_name[child]
                    for child_f in targets:
                        # Skip trait implementations for structs that are not Core 1 structs
                        if child_f.get("trait_name") and child_f.get("struct_name") not in core1_structs:
                            continue
                        child_key = f"{child_f['filepath']}:{child_f['name']}"
                        if child_key not in visited and child_key not in parent_map:
                            parent_map[child_key] = curr_key
                            queue.append(child_key)

    return warnings, errors


def validate_multicore_support():
    scan_dirs = ["controller/src", "peripherals/src", "app/src", "board/src"]

    all_functions = []
    all_controller_structs = []

    with Halo(text="Scanning and parsing AST for multicore support...", spinner="dots") as spinner:
        for s_dir in scan_dirs:
            if not os.path.exists(s_dir):
                continue
            for root, _, files in os.walk(s_dir):
                for file in files:
                    if file.endswith(".rs") and file != "mock.rs":
                        filepath = os.path.join(root, file)
                        try:
                            with open(filepath, "rb") as f:
                                content = f.read()
                            funcs, structs = parse_code(content, filepath)
                            all_functions.extend(funcs.values())
                            all_controller_structs.extend(structs)
                        except Exception as e:
                            print(f"Error reading/parsing {filepath}: {e}", file=sys.stderr)

    # Validate call graphs for all controller contexts running on Core 1
    total_warnings = 0
    total_errors = 0
    for ctrl in all_controller_structs:
        feature = ctrl.get("core1_feature")
        if not feature:
            continue

        # Resolve roots from core1_roots attribute, defaulting to ["run"]
        roots = ctrl.get("core1_roots")
        if not roots:
            roots = ["run"]

        warnings, errors = validate_call_graph(
            funcs_list=all_functions,
            roots=roots,
            feature=feature,
            root_files=[os.path.basename(ctrl["filepath"])],
        )
        total_warnings += warnings
        total_errors += errors

    # Validate that controllers don't instantiate other controllers directly
    instantiation_errors = 0
    for func in all_functions:
        filepath = func["filepath"]
        filename = os.path.basename(filepath)
        if filepath.startswith("controller/src/") and filename not in ["shell_controller.rs"]:
            for ctrl in all_controller_structs:
                if filepath == ctrl["filepath"]:
                    continue

                pattern = f"{ctrl['name']}::"
                if pattern in func["text"]:
                    print(
                        f"{Fore.RED}ERROR:{Style.RESET_ALL} Controller file '{filepath}' directly references '{ctrl['name']}' via '{pattern}' in function '{func['name']}' at line {func['line']}!"
                    )
                    print(
                        f"  Expected: Controllers must communicate via client/channel interfaces, not direct instantiation or static references."
                    )
                    print()
                    instantiation_errors += 1

    total_errors += instantiation_errors

    # Validate that Core 0 context/CLI functions do not call Core 1 functions directly
    cross_core_errors = 0
    core1_func_names = {f["name"] for f in all_functions if len(f["ram_features"]) > 0}

    for func in all_functions:
        filepath = func["filepath"]
        func_name = func["name"]
        # Check if this function runs on Core 0
        is_core0 = (filepath.startswith("app/src/") and ("_app.rs" in filepath or "_shell.rs" in filepath)) or (
            filepath.startswith("controller/src/") and func_name.startswith("handle_") and func_name.endswith("_cli")
        )
        if is_core0:
            for called in func["calls"]:
                if called in core1_func_names and called not in CROSS_CORE_FUNCS:
                    print(
                        f"{Fore.RED}ERROR:{Style.RESET_ALL} Core 0 function '{func_name}' in {filepath}:{func['line']} "
                        f"calls Core 1 function/method '{called}' (decorated with core1_func)!"
                    )
                    print(
                        f"  Expected: Core 0 code must not directly invoke Core 1 functions. Use message channels or cached state."
                    )
                    print()
                    cross_core_errors += 1

    total_errors += cross_core_errors

    if total_errors > 0:
        print(f"{Fore.RED}Validation FAILED: Found {total_errors} errors and {total_warnings} warnings.")
        return 1
    elif total_warnings > 0:
        print(f"{Fore.YELLOW}Validation completed with {total_warnings} warnings.")
        return 0
    else:
        print(f"{Fore.GREEN}Validation passed: All checks successful.")
        return 0


if __name__ == "__main__":
    sys.exit(validate_multicore_support())
