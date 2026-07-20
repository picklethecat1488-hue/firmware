#!/usr/bin/env bash
set -euo pipefail

# Output file path (optional first argument, defaults to /dev/stdout)
OUTPUT_FILE="${1:-/dev/stdout}"

# Clear or touch output file if it's a regular file
if [ "$OUTPUT_FILE" != "/dev/stdout" ]; then
    mkdir -p "$(dirname "$OUTPUT_FILE")"
    true > "$OUTPUT_FILE"
fi

# Run tests
if cargo nextest --version >/dev/null 2>&1; then
    echo "Running tests in Debug configuration..."
    if [ "$OUTPUT_FILE" = "/dev/stdout" ]; then
        cargo nextest run
        echo "Running tests in Release configuration..."
        cargo nextest run --release
    else
        # Using tee to capture output to file while streaming to stdout
        cargo nextest run --color never 2>&1 | tee "$OUTPUT_FILE"
        echo "Running tests in Release configuration..." | tee -a "$OUTPUT_FILE"
        cargo nextest run --release --color never 2>&1 | tee -a "$OUTPUT_FILE"
    fi
else
    echo "cargo-nextest not found. Falling back to standard cargo test..."
    if [ "$OUTPUT_FILE" = "/dev/stdout" ]; then
        cargo test
        echo "Running tests in Release configuration..."
        cargo test --release
    else
        cargo test -- --color never 2>&1 | tee "$OUTPUT_FILE"
        echo "Running tests in Release configuration..." | tee -a "$OUTPUT_FILE"
        cargo test --release -- --color never 2>&1 | tee -a "$OUTPUT_FILE"
    fi
fi
