#!/usr/bin/env bash
set -euo pipefail

# Find llvm-size in the rustup sysroot
SYSROOT=$(rustc --print sysroot 2>/dev/null || echo "")
if [ -n "$SYSROOT" ]; then
    HOST_TRIPLE=$(rustc -vV | grep host | cut -d' ' -f2)
    LLVM_SIZE="$SYSROOT/lib/rustlib/$HOST_TRIPLE/bin/llvm-size"
else
    LLVM_SIZE=""
fi

# Find size command or error out if neither llvm-size nor arm-none-eabi-size is available
if [ -n "$LLVM_SIZE" ] && [ -f "$LLVM_SIZE" ]; then
    SIZE_CMD="$LLVM_SIZE"
elif command -v arm-none-eabi-size >/dev/null 2>&1; then
    SIZE_CMD="arm-none-eabi-size"
else
    echo "Error: Neither llvm-size nor arm-none-eabi-size was found. Please install the llvm-tools component or the GNU ARM toolchain." >&2
    exit 1
fi

report_size() {
    local label="$1"
    local dir="$2"
    local output_file="$3"

    log_echo() {
        echo "$@"
        if [ "$output_file" != "/dev/stdout" ]; then
            echo "$@" >> "$output_file"
        fi
    }

    log_echo "=== $label ==="
    
    # Find ELF files (files with no extension in the directory)
    local files
    files=$(find "$dir" -maxdepth 1 -type f ! -name "*.*" ! -name ".*" 2>/dev/null || true)
    
    if [ -n "$files" ]; then
        # Run size command on all found files
        # Convert newline-separated list to arguments safely
        # shellcheck disable=SC2086
        local size_out
        size_out=$("$SIZE_CMD" $files)
        log_echo "$size_out"
    else
        log_echo "No binaries found in $dir"
    fi
}

# Output file path (optional first argument, defaults to /dev/stdout)
OUTPUT_FILE="${1:-/dev/stdout}"

# Clear or touch output file if it's a regular file
if [ "$OUTPUT_FILE" != "/dev/stdout" ]; then
    mkdir -p "$(dirname "$OUTPUT_FILE")"
    true > "$OUTPUT_FILE"
fi

report_size "Debug Binaries Size" "target/thumbv6m-none-eabi/debug" "$OUTPUT_FILE"
report_size "Release Binaries Size" "target/thumbv6m-none-eabi/release" "$OUTPUT_FILE"
