#!/usr/bin/env python
"""
Constant Address Validator (Tree-Sitter based).

Validates that Rust source code in the 'platform' and 'controller' crates does not
directly read or write to constant addresses in memory using raw pointer casts.
"""

import os
import sys
import tree_sitter_rust as tsrust
from tree_sitter import Language, Parser
from colorama import init, Fore, Style

init(autoreset=True)

RUST_LANGUAGE = Language(tsrust.language())

TARGET_DIRS = [
    "platform/src",
    "controller/src",
]

# Paths relative to workspace root that are allowed to perform raw pointer casts
WHITELISTED_PATHS = []


def validate_file(filepath):
    """Scan a Rust file using Tree-Sitter for disallowed constant address memory operations."""
    # Check whitelist
    for white_path in WHITELISTED_PATHS:
        if filepath.startswith(white_path):
            return 0

    with open(filepath, "rb") as f:
        content = f.read()

    lines = content.split(b"\n")
    parser = Parser(RUST_LANGUAGE)
    tree = parser.parse(content)

    errors = 0
    const_integers = set()

    def find_constants(node):
        """Identify all constants/statics defined with integer literal values."""
        if node.type in ["const_item", "static_item"]:
            name = None
            value_node = None
            for child in node.children:
                if child.type == "identifier":
                    name = child.text.decode("utf-8")
                elif child.type in [
                    "integer_literal",
                    "unary_expression",
                    "type_cast_expression",
                    "parenthesized_expression",
                ]:
                    value_node = child

            if name and value_node:
                has_int = False

                def check_int(n):
                    nonlocal has_int
                    if n.type == "integer_literal":
                        has_int = True
                    for c in n.children:
                        check_int(c)

                check_int(value_node)
                if has_int:
                    const_integers.add(name)

        for c in node.children:
            find_constants(c)

    def validate_casts(node):
        """Detect pointer casts of integer literals or resolved integer constants."""
        nonlocal errors
        if node.type == "type_cast_expression":
            as_idx = -1
            for idx, child in enumerate(node.children):
                if child.type == "as" or child.text == b"as":
                    as_idx = idx
                    break

            if as_idx != -1:
                value_node = node.children[as_idx - 1] if as_idx > 0 else None
                type_node = node.children[as_idx + 1] if as_idx + 1 < len(node.children) else None

                if value_node and type_node:
                    # Check if target type is a raw pointer type (*const T or *mut T)
                    is_ptr = False

                    def check_ptr_type(n):
                        nonlocal is_ptr
                        if n.type == "pointer_type":
                            is_ptr = True
                        for c in n.children:
                            check_ptr_type(c)

                    check_ptr_type(type_node)

                    if is_ptr:
                        # Check if the value cast is a literal or constant integer
                        is_const_val = False
                        if value_node.type == "integer_literal":
                            is_const_val = True
                        elif value_node.type == "identifier":
                            ident_name = value_node.text.decode("utf-8")
                            if ident_name in const_integers:
                                is_const_val = True
                        elif value_node.type == "unary_expression":
                            for child in value_node.children:
                                if child.type == "integer_literal":
                                    is_const_val = True

                        if is_const_val:
                            line_num = content[: node.start_byte].count(b"\n") + 1
                            line_content = (
                                lines[line_num - 1].decode("utf-8", errors="replace")
                                if line_num - 1 < len(lines)
                                else ""
                            )
                            print(
                                f"{Fore.RED}ERROR:{Style.RESET_ALL} {filepath}:{line_num} - "
                                f"Pointer cast of constant address is forbidden."
                            )
                            print(f"  Line: {line_content.strip()}")
                            errors += 1

        for c in node.children:
            validate_casts(c)

    find_constants(tree.root_node)
    validate_casts(tree.root_node)

    return errors


def validate_constant_addresses():
    """Scan all target files in platform and controller crates."""
    workspace_root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))

    total_errors = 0
    scanned_files = 0

    for target_dir in TARGET_DIRS:
        full_path = os.path.join(workspace_root, target_dir)
        if not os.path.exists(full_path):
            continue

        for root, _, files in os.walk(full_path):
            for file in files:
                if file.endswith(".rs"):
                    filepath = os.path.join(root, file)
                    rel_path = os.path.relpath(filepath, workspace_root)
                    total_errors += validate_file(rel_path)
                    scanned_files += 1

    print(f"[Constant Address Validator] Scanned {scanned_files} files. Errors found: {total_errors}")
    return 1 if total_errors > 0 else 0


if __name__ == "__main__":
    sys.exit(validate_constant_addresses())
