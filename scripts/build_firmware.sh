#!/usr/bin/env bash
set -euo pipefail

# Print and unset RUSTFLAGS if set, as it overrides the linker configs in .cargo/config.toml
if [ -n "${RUSTFLAGS:-}" ]; then
    echo "Warning: RUSTFLAGS is set to '${RUSTFLAGS}'."
    echo "This overrides the workspace linker configuration in .cargo/config.toml and causes MCU binaries to be miscompiled (missing .text/.data sections)."
    echo "Unsetting RUSTFLAGS for the build..."
    unset RUSTFLAGS
fi

# Dynamically construct target-specific RUSTFLAGS based on local/CI environment paths
WORKSPACE_ROOT="$(pwd)"
CARGO_HOME_DIR="${CARGO_HOME:-$HOME/.cargo}"
RUSTC_SYSROOT="$(rustc --print sysroot)"

MCU_RUSTFLAGS=(
  "--remap-path-prefix" "${WORKSPACE_ROOT}=firmware"
  "--remap-path-prefix" "${CARGO_HOME_DIR}/registry/src/index.crates.io-1949cf8c6b5b557f=cargo"
  "--remap-path-prefix" "${CARGO_HOME_DIR}/registry/src=cargo"
  "--remap-path-prefix" "${CARGO_HOME_DIR}/git/checkouts=cargo-git"
  "--remap-path-prefix" "${CARGO_HOME_DIR}=cargo"
  "--remap-path-prefix" "${RUSTC_SYSROOT}=sysroot"
)

export CARGO_TARGET_THUMBV6M_NONE_EABI_RUSTFLAGS="${MCU_RUSTFLAGS[*]}"


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
    
    # Determine host platform name
    HOST_OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    case "$HOST_OS" in
        darwin)
            PLATFORM="macos"
            ;;
        linux)
            PLATFORM="linux"
            ;;
        mingw*|msys*|cygwin*|windows)
            PLATFORM="windows"
            ;;
        *)
            PLATFORM="linux" # fallback
            ;;
    esac

    if [ "$BUILD_MODE" = "debug" ] || [ "$BUILD_MODE" = "both" ]; then
        mkdir -p "$ORGANIZE_DIR/debug/embedded"
        # Copy target MCU debug binaries (excluding hidden files/dependency files)
        find target/thumbv6m-none-eabi/debug -maxdepth 1 -type f ! -name "*.*" ! -name ".*" -exec cp {} "$ORGANIZE_DIR/debug/embedded/" \;
    fi

    if [ "$BUILD_MODE" = "release" ] || [ "$BUILD_MODE" = "both" ]; then
        mkdir -p "$ORGANIZE_DIR/release/embedded"
        # Copy target MCU release binaries (excluding hidden files/dependency files)
        find target/thumbv6m-none-eabi/release -maxdepth 1 -type f ! -name "*.*" ! -name ".*" -exec cp {} "$ORGANIZE_DIR/release/embedded/" \;
        
        # Copy host tools
        mkdir -p "$ORGANIZE_DIR/release/$PLATFORM"
        for tool in "${TOOL_PACKAGES[@]}"; do
            if [ -f "target/release/$tool" ]; then
                cp "target/release/$tool" "$ORGANIZE_DIR/release/$PLATFORM/$tool"
            elif [ -f "target/release/$tool.exe" ]; then
                cp "target/release/$tool.exe" "$ORGANIZE_DIR/release/$PLATFORM/$tool.exe"
            fi
        done
    fi
fi
