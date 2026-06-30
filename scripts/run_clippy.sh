#!/usr/bin/env bash
set -euo pipefail

# Find all packages in the workspace that are under the "tools" directory to exclude them
EXCLUDE_ARGS=()
while IFS= read -r pkg; do
    if [ -n "$pkg" ]; then
        EXCLUDE_ARGS+=("--exclude" "$pkg")
    fi
done < <(cargo metadata --format-version 1 | jq -r '.packages[] | select(.manifest_path | contains("/tools/")) | .name')

echo "Checking formatting..."
cargo fmt --all --check

echo "Running clippy on Host Targets..."
cargo clippy --all-targets -- -D warnings

echo "Running clippy on MCU Target (thumbv6m-none-eabi)..."
cargo clippy --workspace "${EXCLUDE_ARGS[@]}" --lib --bins --target thumbv6m-none-eabi -- -D warnings
