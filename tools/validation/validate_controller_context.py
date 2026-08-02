#!/usr/bin/env python
import os
import re
import sys
import tree_sitter_rust as tsrust
from tree_sitter import Language, Parser
from colorama import init, Fore, Style

init(autoreset=True)

RUST_LANGUAGE = Language(tsrust.language())


def get_struct_name(struct_node):
    """Extract the struct identifier name from a struct_item node."""
    for child in struct_node.children:
        if child.type == "type_identifier":
            return child.text.decode("utf-8")
    return None


def to_snake_case(name):
    """Convert PascalCase name to snake_case."""
    s1 = re.sub("(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub("([a-z0-9])([A-Z])", r"\1_\2", s1).lower()


def validate_controller_context():
    # Find controllers.toml
    workspace_root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    toml_path = os.path.join(workspace_root, "controller", "controllers.toml")
    if not os.path.exists(toml_path):
        print(f"{Fore.RED}ERROR:{Style.RESET_ALL} controllers.toml not found!")
        return 1

    with open(toml_path, "r") as f:
        toml_content = f.read()

    # Get all controller names
    controller_names = re.findall(r'name\s*=\s*"([^"]+)"', toml_content)
    if not controller_names:
        print(f"{Fore.RED}ERROR:{Style.RESET_ALL} No controllers found in controllers.toml!")
        return 1

    errors = 0
    parser = Parser(RUST_LANGUAGE)

    for name in controller_names:
        snake_name = to_snake_case(name)
        filepath = os.path.join(workspace_root, "controller", "src", f"{snake_name}_controller.rs")

        if not os.path.exists(filepath):
            print(f"{Fore.RED}ERROR:{Style.RESET_ALL} Controller source file {filepath} not found!")
            errors += 1
            continue

        with open(filepath, "rb") as f:
            content = f.read()

        tree = parser.parse(content)
        expected_struct_name = f"{name}Controller"
        found_struct = False
        is_decorated = False
        struct_line = 0

        def traverse(node):
            nonlocal found_struct, is_decorated, struct_line
            if node.type == "struct_item":
                s_name = get_struct_name(node)
                if s_name == expected_struct_name:
                    found_struct = True
                    struct_line = node.start_point[0] + 1

                    # Check for controller_context attribute preceding the struct
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
                                    is_decorated = True
                                k -= 1
                            elif sibling.type in ["line_comment", "block_comment", "\n"]:
                                k -= 1
                            else:
                                break

            for child in node.children:
                traverse(child)

        traverse(tree.root_node)

        if not found_struct:
            print(f"{Fore.RED}ERROR:{Style.RESET_ALL} Could not find struct '{expected_struct_name}' in {filepath}!")
            errors += 1
        elif not is_decorated:
            print(
                f"{Fore.RED}ERROR:{Style.RESET_ALL} Struct '{expected_struct_name}' in {filepath}:{struct_line} is not decorated with #[controller_context]!"
            )
            errors += 1

    if errors > 0:
        print(
            f"{Fore.RED}Validation FAILED:{Style.RESET_ALL} {errors} controller context(s) are missing #[controller_context] decoration."
        )
        return 1

    print(
        f"{Fore.GREEN}Validation PASSED:{Style.RESET_ALL} All defined controller contexts are correctly decorated with #[controller_context]."
    )
    return 0


if __name__ == "__main__":
    sys.exit(validate_controller_context())
