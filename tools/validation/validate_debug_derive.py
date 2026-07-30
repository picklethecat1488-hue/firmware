#!/usr/bin/env python
"""
Debug Derive Validator (Tree-Sitter based).

Validates that Rust enums and structs targeting embedded MCU devices do not unconditionally
derive 'Debug' to conserve code size. Enums and structs must conditionally derive 'Debug'
via:
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
"""

import os
import sys
import tree_sitter_rust as tsrust
from tree_sitter import Language, Parser
from colorama import init, Fore, Style

init(autoreset=True)

RUST_LANGUAGE = Language(tsrust.language())

TARGET_DIRS = [
    "controller/src",
    "peripherals/src",
    "platform/src",
    "model/src",
    "projects/cat_detector/src",
]


def get_item_name(item_node):
    """Extract the identifier name from an enum_item or struct_item node."""
    for child in item_node.children:
        if child.type in ["name", "type_identifier", "identifier"]:
            return child.text.decode("utf-8")
    return None


def is_unconditional_derive_debug(attr_node):
    """Check if an attribute_item node is a direct #[derive(...)] containing Debug."""
    attribute_node = None
    for child in attr_node.children:
        if child.type == "attribute":
            attribute_node = child
            break

    if not attribute_node:
        return False

    is_derive = False
    token_tree_node = None
    for child in attribute_node.children:
        if child.type == "identifier" and child.text.decode("utf-8") == "derive":
            is_derive = True
        elif child.type == "token_tree":
            token_tree_node = child

    if not is_derive or not token_tree_node:
        return False

    identifiers = []

    def find_identifiers(n):
        if n.type == "identifier":
            identifiers.append(n.text.decode("utf-8"))
        for c in n.children:
            find_identifiers(c)

    find_identifiers(token_tree_node)

    return "Debug" in identifiers


def validate_file(filepath):
    with open(filepath, "rb") as f:
        content = f.read()

    parser = Parser(RUST_LANGUAGE)
    tree = parser.parse(content)

    errors = 0

    def traverse(node):
        nonlocal errors
        if node.type in ["enum_item", "struct_item"]:
            item_name = get_item_name(node) or "<unknown>"
            item_type = "Enum" if node.type == "enum_item" else "Struct"

            # Find preceding attribute items
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
                        if is_unconditional_derive_debug(sibling):
                            line_no = content[: node.start_byte].count(b"\n") + 1
                            print(
                                f"{Fore.RED}ERROR:{Style.RESET_ALL} {item_type} '{item_name}' in '{filepath}' at line {line_no} "
                                f"unconditionally derives 'Debug'! {item_type}s targeting embedded devices must conditionally "
                                f'derive \'Debug\' via #[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))].'
                            )
                            errors += 1
                        k -= 1
                    elif sibling.type in ["line_comment", "block_comment", "\n"]:
                        k -= 1
                    else:
                        break

        for child in node.children:
            traverse(child)

    traverse(tree.root_node)
    return errors


def validate_debug_derive():
    total_errors = 0
    workspace_root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))

    for target_dir in TARGET_DIRS:
        full_dir = os.path.join(workspace_root, target_dir)
        if not os.path.exists(full_dir):
            continue

        for root, _, files in os.walk(full_dir):
            for file in files:
                if file.endswith(".rs"):
                    filepath = os.path.join(root, file)
                    total_errors += validate_file(filepath)

    if total_errors > 0:
        print(
            f"\n{Fore.RED}Validation FAILED: Found {total_errors} enum(s) or struct(s) deriving Debug unconditionally.{Style.RESET_ALL}"
        )
        return 1
    else:
        print(f"{Fore.GREEN}Validation PASSED: All enums and structs conditionally derive Debug.{Style.RESET_ALL}")
        return 0


if __name__ == "__main__":
    sys.exit(validate_debug_derive())
