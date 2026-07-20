#!/usr/bin/env python
import os
import re
import sys
import tree_sitter_rust as tsrust
from tree_sitter import Language, Parser
from colorama import init, Fore, Style
from halo import Halo

init(autoreset=True)

# Load the Rust grammar using tree-sitter
RUST_LANGUAGE = Language(tsrust.language())

# Files excluded from core consistency validation (e.g. macro templates, mock test files)
EXCLUDED_CORE_FILES = [
    "controller/src/lib.rs",
    "peripherals/src/mock.rs",
]

# Files excluded from call-site scans (e.g. macro templates to avoid false positives)
EXCLUDED_CALLSITE_FILES = [
    "controller/src/lib.rs",
]

# Rust attribute strings searched during discovery
INSTRUMENT_ATTRIBUTES = ["tracing::instrument", "instrument"]
CORE1_GATING_FEATURES = ["motor-core", "sensors-core"]
EMBASSY_TASK_ATTRIBUTES = ["embassy_executor::task", "task"]


def find_descendants(node, target_type, result):
    """Recursively find all descendant nodes of a specific type, excluding child functions/closures."""
    if node.type == target_type:
        result.append(node)
        return
    # Skip traversing nested functions or closures to avoid matching their returns/calls
    if node.type in ["function_item", "closure_expression"]:
        return
    for child in node.children:
        find_descendants(child, target_type, result)


def parse_rs_file(filepath):
    """Parse a Rust source file to locate all function definitions.

    Find function boundaries, and whether they are instrumented or embassy tasks.
    """
    try:
        with open(filepath, "rb") as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {filepath}: {e}", file=sys.stderr)
        return []

    parser = Parser(RUST_LANGUAGE)
    tree = parser.parse(content)

    functions = []

    def traverse_ast(node):
        if node.type == "function_item":
            # Extract function name
            fn_name = None
            for child in node.children:
                if child.type in ["name", "identifier"]:
                    fn_name = child.text.decode("utf-8")
                    break

            if fn_name:
                # Find attributes (siblings immediately preceding this function node)
                parent = node.parent
                is_instrumented = False
                is_embassy_task = False
                instrument_name = None
                is_core1_gated = False
                has_core1_instrument = False

                if parent:
                    idx = -1
                    for i, child in enumerate(parent.children):
                        if child.id == node.id:
                            idx = i
                            break

                    # Scan backward for attributes preceding the function definition
                    k = idx - 1
                    while k >= 0:
                        sibling = parent.children[k]
                        if sibling.type == "attribute_item":
                            attr_text = sibling.text.decode("utf-8")
                            if any(x in attr_text for x in INSTRUMENT_ATTRIBUTES):
                                is_instrumented = True
                                # Extract instrument name parameter if present
                                match = re.search(r'\bname\s*=\s*"([^"]+)"', attr_text)
                                if match:
                                    instrument_name = match.group(1)
                                match_core = re.search(r"\bcore\d+\b", attr_text)
                                if match_core and match_core.group(0) == "core1":
                                    has_core1_instrument = True
                            if any(x in attr_text for x in CORE1_GATING_FEATURES):
                                if "link_section" in attr_text and ".data.ram_func" in attr_text:
                                    is_core1_gated = True
                            if any(x in attr_text for x in EMBASSY_TASK_ATTRIBUTES):
                                is_embassy_task = True
                            k -= 1
                        elif sibling.type in ["line_comment", "block_comment", "\n"]:
                            k -= 1
                        else:
                            # Stop when hitting another declaration or node type
                            break

                # Check for return statements in function body
                has_return = False
                block_node = None
                for child in node.children:
                    if child.type == "block":
                        block_node = child
                        break

                if block_node:
                    return_nodes = []
                    find_descendants(block_node, "return_expression", return_nodes)
                    if return_nodes:
                        has_return = True

                functions.append(
                    {
                        "name": fn_name,
                        "start_line": node.start_point[0] + 1,
                        "end_line": node.end_point[0] + 1,
                        "is_instrumented": is_instrumented,
                        "is_embassy_task": is_embassy_task,
                        "has_return": has_return,
                        "file": filepath,
                        "instrument_name": instrument_name,
                        "is_core1_gated": is_core1_gated,
                        "has_core1_instrument": has_core1_instrument,
                    }
                )

        for child in node.children:
            traverse_ast(child)

    traverse_ast(tree.root_node)
    return functions


def is_boot_context(func):
    """Check if a function represents an uninstrumented boot/initialization context.

    Determine if calling instrumented functions from this context is prohibited.
    """
    name = func["name"]
    return (
        name == "main"
        or name == "new"
        or name.startswith("new_")
        or (name == "init" and not func["is_instrumented"])
        or (name.endswith("_init") and not func["is_instrumented"])
    )


def get_called_function_name(call_node):
    """Extract the function identifier being called from a call_expression node."""
    func_node = call_node.children[0]
    match func_node.type:
        case "identifier":
            return func_node.text.decode("utf-8")
        case "field_expression":
            # obj.foo() -> last child is field_identifier
            for child in reversed(func_node.children):
                if child.type == "field_identifier":
                    return child.text.decode("utf-8")
        case "scoped_identifier":
            # Foo::foo() -> last child is identifier
            for child in reversed(func_node.children):
                if child.type == "identifier":
                    return child.text.decode("utf-8")
        case "generic_function":
            # foo::<T>() -> first child is identifier
            first_child = func_node.children[0]
            match first_child.type:
                case "identifier":
                    return first_child.text.decode("utf-8")
                case "scoped_identifier":
                    for child in reversed(first_child.children):
                        if child.type == "identifier":
                            return child.text.decode("utf-8")
    return None


def main():
    # Directories to scan
    scan_dirs = ["controller/src", "projects/cat_detector/src", "peripherals/src"]

    # 1. Discover all functions
    all_functions = []
    with Halo(text="Scanning and parsing AST for tracing hierarchy...", spinner="dots") as spinner:
        for s_dir in scan_dirs:
            if not os.path.exists(s_dir):
                continue
            for root, _, files in os.walk(s_dir):
                for file in files:
                    if file.endswith(".rs"):
                        filepath = os.path.join(root, file)
                        all_functions.extend(parse_rs_file(filepath))

    # Index functions by file for fast call-site context lookups
    funcs_by_file = {}
    for f in all_functions:
        funcs_by_file.setdefault(f["file"], []).append(f)

    # 2. Build set of instrumented function names
    instrumented_funcs = [f for f in all_functions if f["is_instrumented"]]
    instrumented_names = {f["name"] for f in instrumented_funcs}
    embassy_tasks = {f["name"] for f in all_functions if f["is_embassy_task"]}

    # Discard common generic names to avoid collision with standard libraries/BSP configs
    # where the local instrumented version is private or already verified (e.g. TelemetryController::init)
    instrumented_names.discard("init")

    print(f"Found {len(all_functions)} total function definitions.")
    print(f"Found {len(instrumented_funcs)} instrumented functions.")
    print(f"Found {len(embassy_tasks)} Embassy tasks.")
    print("-" * 60)

    errors = 0

    # 2.5 Check core instrumentation consistency
    for f in all_functions:
        if any(f["file"].endswith(ex) for ex in EXCLUDED_CORE_FILES):
            continue

        match (f["is_instrumented"], f["is_core1_gated"], f["has_core1_instrument"]):
            case (True, True, False):
                print(
                    f"{Fore.RED}ERROR:{Style.RESET_ALL} Core 1 gated function '{f['name']}' must be instrumented with #[instrument(core1)]!"
                )
                print(f"  File: {f['file']}:{f['start_line']}")
                print()
                errors += 1
            case (True, False, True):
                print(
                    f"{Fore.RED}ERROR:{Style.RESET_ALL} Core 0 function '{f['name']}' must NOT be instrumented with core1!"
                )
                print(f"  File: {f['file']}:{f['start_line']}")
                print()
                errors += 1
            case _:
                pass

    # 3. Check for early returns (i.e. 'return' statements) inside instrumented functions
    for f in instrumented_funcs:
        if f["has_return"]:
            try:
                with open(f["file"], "r", encoding="utf-8") as file_obj:
                    lines = file_obj.readlines()
            except Exception:
                continue

            # Find the actual line with the return statement
            # Look inside the function body
            for idx in range(f["start_line"] - 1, f["end_line"]):
                line_content = lines[idx]
                if "//" in line_content:
                    line_content = line_content.split("//")[0]
                if "return " in line_content or "return;" in line_content:
                    print(
                        f"{Fore.RED}ERROR:{Style.RESET_ALL} Instrumented function '{f['name']}' contains a 'return' statement (early returns bypass async span exits!)"
                    )
                    print(f"  File: {f['file']}:{idx + 1}")
                    print(f"  Line: {lines[idx].strip()}")
                    print()
                    errors += 1
                    break

    # 4. Scan all files for call sites of non-task instrumented functions
    for filepath, file_funcs in funcs_by_file.items():
        # Skip macro definition files (templates in lib.rs) to avoid template false positives
        if any(filepath.endswith(ex) for ex in EXCLUDED_CALLSITE_FILES):
            continue

        try:
            with open(filepath, "rb") as f:
                content = f.read()
        except Exception:
            continue

        parser = Parser(RUST_LANGUAGE)
        tree = parser.parse(content)

        def find_calls(node):
            nonlocal errors
            if node.type == "call_expression":
                target_name = get_called_function_name(node)
                if target_name and target_name in instrumented_names:
                    # Filter out third-party library calls (like embassy_rp::init)
                    func_text = node.children[0].text
                    if b"embassy_rp::" in func_text or b"Default::" in func_text:
                        pass
                    elif target_name not in embassy_tasks:
                        # Find containing function
                        containing_func = None
                        curr = node.parent
                        while curr:
                            if curr.type == "function_item":
                                # Match with parsed function range
                                start_line = curr.start_point[0] + 1
                                for f in file_funcs:
                                    if f["start_line"] == start_line:
                                        containing_func = f
                                        break
                                break
                            curr = curr.parent

                        line_num = node.start_point[0] + 1
                        line_text = content.splitlines()[line_num - 1].decode("utf-8", errors="ignore")

                        if containing_func:
                            # Exclude self-recursive calls or name matches in the same function block
                            if containing_func["name"] != target_name:
                                # Check if the containing function is an uninstrumented boot/initialization context
                                if is_boot_context(containing_func):
                                    print(
                                        f"{Fore.RED}ERROR:{Style.RESET_ALL} Instrumented function '{target_name}' called from uninstrumented boot/init context '{containing_func['name']}'!"
                                    )
                                    print(f"  File: {filepath}:{line_num}")
                                    print(f"  Line: {line_text.strip()}")
                                    print()
                                    errors += 1
                        else:
                            # Call is outside any function body
                            print(
                                f"{Fore.RED}ERROR:{Style.RESET_ALL} Instrumented function '{target_name}' called outside of any function context!"
                            )
                            print(f"  File: {filepath}:{line_num}")
                            print(f"  Line: {line_text.strip()}")
                            print()
                            errors += 1

            for child in node.children:
                find_calls(child)

        find_calls(tree.root_node)

    if errors > 0:
        print(f"{Fore.RED}Validation FAILED: {errors} boot-time tracing hierarchy or early return violations found.")
        sys.exit(1)
    else:
        print(f"{Fore.GREEN}Validation PASSED: All instrumented functions are correctly nested and exit cleanly.")
        sys.exit(0)


if __name__ == "__main__":
    main()
