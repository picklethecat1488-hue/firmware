#!/usr/bin/env bash
set -euo pipefail

# Output file path (optional first argument, defaults to /dev/stdout)
OUTPUT_FILE="${1:-/dev/stdout}"

# Clear or touch output file if it's a regular file
if [ "$OUTPUT_FILE" != "/dev/stdout" ]; then
    mkdir -p "$(dirname "$OUTPUT_FILE")"
    true > "$OUTPUT_FILE"
fi

# Find all packages in the workspace that are under the "tools" directory to exclude them
EXCLUDE_ARGS=()
while IFS= read -r pkg; do
    if [ -n "$pkg" ]; then
        EXCLUDE_ARGS+=("--exclude" "$pkg")
    fi
done < <(cargo metadata --format-version 1 | jq -r '.packages[] | select(.manifest_path | contains("/tools/") or contains("/host/")) | .name')

# Helper function to run commands and redirect both stdout and stderr
run_and_report() {
    local cmd="$1"
    if [ "$OUTPUT_FILE" = "/dev/stdout" ]; then
        eval "$cmd"
    else
        echo "=== Running: $cmd ===" >> "$OUTPUT_FILE"
        eval "$cmd" 2>&1 | tee -a "$OUTPUT_FILE"
    fi
}

echo "Checking formatting..."
run_and_report "cargo fmt --all --check"

echo "Running clippy on Host Targets..."
run_and_report "cargo clippy --all-targets --color never -- -D warnings"

echo "Running clippy on MCU Target (thumbv6m-none-eabi)..."
run_and_report "cargo clippy --workspace ${EXCLUDE_ARGS[*]} --lib --bins --target thumbv6m-none-eabi --color never -- -D warnings"

echo "Validating tracing hierarchy and early returns..."
run_and_report "python tools/validation/validate_tracing.py"

echo "Validating RAM placement for multicore execution..."
run_and_report "python tools/validation/validate_multicore_support.py"
