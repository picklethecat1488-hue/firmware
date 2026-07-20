#!/usr/bin/env bash
set -euo pipefail

START_TIME=$(date +%s)
echo "=== Starting Quick Verification ==="

# 1. Run formatting and Python static validators in parallel
echo "1. Running static analysis & formatting checks in parallel..."
cargo fmt --all --check &
FMT_PID=$!

python scripts/validate_tracing.py &
TRACING_PID=$!

python scripts/validate_multicore_support.py &
MULTICORE_PID=$!

# Wait and report failures immediately
wait $FMT_PID || { echo "❌ Formatting check failed (run 'cargo fmt --all' to fix)"; exit 1; }
wait $TRACING_PID || { echo "❌ Tracing hierarchy check failed"; exit 1; }
wait $MULTICORE_PID || { echo "❌ Multicore placement check failed"; exit 1; }
echo "   Static checks passed!"

# 2. Fast Cargo Check for both Host and MCU Targets
echo "2. Running fast checks..."
cargo check --workspace --all-targets --color never

# Find all packages in the workspace that are under the "tools" directory to exclude them
EXCLUDE_ARGS=()
while IFS= read -r pkg; do
    if [ -n "$pkg" ]; then
        EXCLUDE_ARGS+=("--exclude" "$pkg")
    fi
done < <(cargo metadata --format-version 1 | jq -r '.packages[] | select(.manifest_path | contains("/tools/")) | .name')

cargo check --workspace "${EXCLUDE_ARGS[@]}" --bins --lib --target thumbv6m-none-eabi --color never
echo "   Compilation checks passed!"

# 3. Run tests using nextest (falls back to cargo test)
echo "3. Running test suite..."
if command -v cargo-nextest >/dev/null 2>&1; then
    cargo nextest run --color never
else
    cargo test --color never
fi

# 4. Build debug firmware target and debug host tools
echo "4. Building target binaries and host tools (Debug)..."
./scripts/build_firmware.sh --debug-only
cargo build -p host_cli -p host_fs

END_TIME=$(date +%s)
ELAPSED=$((END_TIME - START_TIME))
echo "=== 🚀 Verification PASSED in ${ELAPSED}s ==="
