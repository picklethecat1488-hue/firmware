#!/usr/bin/env bash
set -euo pipefail

START_TIME=$(date +%s)

# Run parallel codebase validations (cargo format, clippy, python AST validators)
echo "Running parallel codebase validations..."
python tools/validation/validate_all.py

# Run tests using nextest (falls back to cargo test)
echo "Running test suite..."
if command -v cargo-nextest >/dev/null 2>&1; then
    cargo nextest run
else
    cargo test
fi

echo "   Running Python tests..."
pytest

# Build debug firmware target and debug host tools
echo "Building target binaries and host tools (Debug)..."
./tools/build/build_firmware.sh --debug-only
./tools/build/build_host_tools.sh --debug

END_TIME=$(date +%s)
ELAPSED=$((END_TIME - START_TIME))
echo "Verification PASSED in ${ELAPSED}s"
