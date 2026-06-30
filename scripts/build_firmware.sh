#!/usr/bin/env bash
set -euo pipefail

# Find all tool packages to exclude from target build, and include in host build
TOOL_PACKAGES=()
EXCLUDE_ARGS=()
while IFS= read -r pkg; do
    if [ -n "$pkg" ]; then
        TOOL_PACKAGES+=("$pkg")
        EXCLUDE_ARGS+=("--exclude" "$pkg")
    fi
done < <(cargo metadata --format-version 1 | jq -r '.packages[] | select(.manifest_path | contains("/tools/")) | .name')

# Check if we should only build debug, release, or both
BUILD_MODE="both"
ORGANIZE_DIR=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --debug-only)
            BUILD_MODE="debug"
            shift
            ;;
        --release-only)
            BUILD_MODE="release"
            shift
            ;;
        --out-dir)
            ORGANIZE_DIR="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1" >&2
            exit 1
            ;;
    esac
done

build_mcu_debug() {
    echo "Building Target MCU Binaries (Debug)..."
    cargo build --workspace "${EXCLUDE_ARGS[@]}" --bins --target thumbv6m-none-eabi
}

build_mcu_release() {
    echo "Building Target MCU Binaries (Release)..."
    cargo build --release --workspace "${EXCLUDE_ARGS[@]}" --bins --target thumbv6m-none-eabi
}

build_host_tools() {
    echo "Building Host Tools (Release)..."
    for tool in "${TOOL_PACKAGES[@]}"; do
        cargo build --release -p "$tool"
    done
}

# Execute builds
if [ "$BUILD_MODE" = "debug" ]; then
    build_mcu_debug
elif [ "$BUILD_MODE" = "release" ]; then
    build_mcu_release
    build_host_tools
else
    build_mcu_debug
    build_mcu_release
    build_host_tools
fi

# Organize outputs if output directory is specified
if [ -n "$ORGANIZE_DIR" ]; then
    echo "Organizing build outputs into $ORGANIZE_DIR..."
    
    # Clean/create target folders
    rm -rf "$ORGANIZE_DIR"
    
    if [ "$BUILD_MODE" = "debug" ] || [ "$BUILD_MODE" = "both" ]; then
        mkdir -p "$ORGANIZE_DIR/debug"
        # Copy target MCU debug binaries (excluding hidden files/dependency files)
        find target/thumbv6m-none-eabi/debug -maxdepth 1 -type f ! -name "*.*" ! -name ".*" -exec cp {} "$ORGANIZE_DIR/debug/" \;
    fi

    if [ "$BUILD_MODE" = "release" ] || [ "$BUILD_MODE" = "both" ]; then
        mkdir -p "$ORGANIZE_DIR/release"
        # Copy target MCU release binaries (excluding hidden files/dependency files)
        find target/thumbv6m-none-eabi/release -maxdepth 1 -type f ! -name "*.*" ! -name ".*" -exec cp {} "$ORGANIZE_DIR/release/" \;
        
        # Copy host tools
        for tool in "${TOOL_PACKAGES[@]}"; do
            if [ -f "target/release/$tool" ]; then
                cp "target/release/$tool" "$ORGANIZE_DIR/release/$tool"
            fi
        done
    fi
fi
