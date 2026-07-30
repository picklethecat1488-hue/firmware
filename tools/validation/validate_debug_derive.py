#!/usr/bin/env python
"""
Debug Derive Validator.

Validates that Rust enums targeting embedded MCU devices do not unconditionally
derive 'Debug' to conserve code size. Enums must conditionally derive 'Debug'
via:
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
"""

import os
import re
import sys
from colorama import init, Fore, Style

init(autoreset=True)

ENUM_RE = re.compile(r"\b(?:pub\s+)?enum\s+([a-zA-Z0-9_]+)\b")
DERIVE_RE = re.compile(r"#\[\s*derive\s*\(([^)]+)\)\s*\]", re.DOTALL)

TARGET_DIRS = [
    "controller/src",
    "peripherals/src",
    "platform/src",
    "model/src",
    "projects/cat_detector/src",
]


def validate_file(filepath):
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()

    errors = 0
    lines = content.split("\n")
    for match in ENUM_RE.finditer(content):
        enum_name = match.group(1)
        char_idx = match.start()
        line_idx = content[:char_idx].count("\n")

        # Look backwards for attributes preceding the enum
        attrs = []
        i = line_idx - 1
        while i >= 0:
            line = lines[i].strip()
            if not line:
                i -= 1
                continue
            if line.startswith("///") or line.startswith("//") or line.startswith("/*") or line.endswith("*/"):
                i -= 1
                continue
            if line.startswith("#[") or (attrs and not line.startswith("#[")):
                attrs.append(line)
                i -= 1
            else:
                break

        attrs_text = "\n".join(reversed(attrs))
        for args in DERIVE_RE.findall(attrs_text):
            derived_traits = [t.strip() for t in args.split(",")]
            if "Debug" in derived_traits:
                print(
                    f"{Fore.RED}ERROR:{Style.RESET_ALL} Enum '{enum_name}' in '{filepath}' at line {line_idx + 1} "
                    f"unconditionally derives 'Debug'! Enums targeting embedded devices must conditionally "
                    f'derive \'Debug\' via #[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))].'
                )
                errors += 1
    return errors


def main():
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
            f"\n{Fore.RED}Validation FAILED: Found {total_errors} enum(s) deriving Debug unconditionally.{Style.RESET_ALL}"
        )
        sys.exit(1)
    else:
        print(f"{Fore.GREEN}Validation PASSED: All enums conditionally derive Debug.{Style.RESET_ALL}")
        sys.exit(0)


if __name__ == "__main__":
    main()
