#!/usr/bin/env bash
set -euo pipefail

START_TIME=$(date +%s)

# Raise open file descriptor limit if possible
ulimit -n 65536 2>/dev/null || true

if [ -d "/workspace/miniforge3/envs/firmware-env/bin" ]; then
    export PATH="/workspace/miniforge3/envs/firmware-env/bin:${PATH}"
elif [ -d "/root/miniforge3/envs/firmware-env/bin" ]; then
    export PATH="/root/miniforge3/envs/firmware-env/bin:${PATH}"
elif [ -d "${HOME}/miniforge3/envs/firmware-env/bin" ]; then
    export PATH="${HOME}/miniforge3/envs/firmware-env/bin:${PATH}"
fi

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
NUM_CPUS=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)
if [ "${NUM_CPUS}" -gt 16 ]; then
    PYTEST_WORKERS=16
else
    PYTEST_WORKERS="${NUM_CPUS}"
fi
OPENBLAS_NUM_THREADS=1 MKL_NUM_THREADS=1 OMP_NUM_THREADS=1 pytest -n "${PYTEST_WORKERS}"

# Build debug firmware target and debug host tools
echo "Building target binaries and host tools (Debug)..."
./tools/build/build_firmware.sh --debug-only
./tools/build/build_host_tools.sh --debug

END_TIME=$(date +%s)
ELAPSED=$((END_TIME - START_TIME))
echo "Verification PASSED in ${ELAPSED}s"
