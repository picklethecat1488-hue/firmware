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
done < <(cargo metadata --format-version 1 | jq -r '.packages[] | select(.manifest_path | contains("/tools/") or contains("/host/")) | .name')

# Check if multiple binary targets with the same name exist in the workspace
DUPLICATE_BINS=$(cargo metadata --format-version 1 | jq -r '.packages[].targets[] | select(.kind[] == "bin") | .name' | sort | uniq -d)
if [ -n "$DUPLICATE_BINS" ]; then
    echo "Error: Duplicate binary target names detected in the workspace:" >&2
    echo "$DUPLICATE_BINS" | sed 's/^/  - /' >&2
    echo "Cargo requires all binary targets to have unique names across the workspace." >&2
    exit 1
fi

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

restructure_target_dir() {
    local dir="$1"
    if [ ! -d "$dir" ]; then
        return
    fi

    # Find all files with no extension in the directory that contain an underscore
    find "$dir" -maxdepth 1 -type f ! -name "*.*" ! -name ".*" 2>/dev/null | while read -r file; do
        local filename
        filename=$(basename "$file")
        if [[ "$filename" == *_* ]]; then
            local component="${filename##*_}"
            local project="${filename%_*}"
            if [ -n "$project" ] && [ -n "$component" ]; then
                mkdir -p "$dir/$project"
                cp "$file" "$dir/$project/$component"
            fi
        fi
    done
}

build_mcu_debug() {
    echo "Building Target MCU Binaries (Debug)..."
    cargo build --workspace "${EXCLUDE_ARGS[@]}" --bins --target thumbv6m-none-eabi

    # Restructure outputs into project subdirectories
    restructure_target_dir "target/thumbv6m-none-eabi/debug"
}

build_mcu_release() {
    echo "Building Target MCU Binaries (Release)..."
    cargo build --release --workspace "${EXCLUDE_ARGS[@]}" --bins --target thumbv6m-none-eabi

    # Restructure outputs into project subdirectories
    restructure_target_dir "target/thumbv6m-none-eabi/release"
}

# Execute builds
if [ "$BUILD_MODE" = "debug" ]; then
    build_mcu_debug
elif [ "$BUILD_MODE" = "release" ]; then
    build_mcu_release
else
    build_mcu_debug
    build_mcu_release
fi

# Organize outputs if output directory is specified
if [ -n "$ORGANIZE_DIR" ]; then
    echo "Organizing build outputs into $ORGANIZE_DIR..."
    
    # Clean/create target folders
    rm -rf "$ORGANIZE_DIR"
    
    if [ "$BUILD_MODE" = "debug" ] || [ "$BUILD_MODE" = "both" ]; then
        mkdir -p "$ORGANIZE_DIR/debug/embedded"
        # Copy target MCU debug binaries
        cp -R target/thumbv6m-none-eabi/debug/cat_detector "$ORGANIZE_DIR/debug/embedded/"
    fi

    if [ "$BUILD_MODE" = "release" ] || [ "$BUILD_MODE" = "both" ]; then
        mkdir -p "$ORGANIZE_DIR/release/embedded"
        # Copy target MCU release binaries
        cp -R target/thumbv6m-none-eabi/release/cat_detector "$ORGANIZE_DIR/release/embedded/"
    fi
fi
