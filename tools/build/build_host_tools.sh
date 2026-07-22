#!/usr/bin/env bash
set -euo pipefail

# Find all tool packages in the workspace
TOOL_PACKAGES=()
while IFS= read -r pkg; do
    if [ -n "$pkg" ]; then
        TOOL_PACKAGES+=("$pkg")
    fi
done < <(cargo metadata --format-version 1 | jq -r '.packages[] | select(.manifest_path | contains("/tools/") or contains("\\tools\\") or contains("/host/") or contains("\\host\\")) | .name' | tr -d '\r')

ORGANIZE_DIR=""
ZIP_FILE=""
TARGET=""
WORKSPACE_ROOT="$(pwd)"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --out-dir)
            ORGANIZE_DIR="$2"
            shift 2
            ;;
        --zip)
            ZIP_FILE="$2"
            shift 2
            ;;
        --target)
            TARGET="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1" >&2
            exit 1
            ;;
    esac
done

RELEASE_DIR="target/release"
if [ -n "$TARGET" ]; then
    RELEASE_DIR="target/$TARGET/release"
fi

echo "Building Host Tools (Release)..."
for tool in "${TOOL_PACKAGES[@]}"; do
    if [ -n "$TARGET" ]; then
        cargo build --release -p "$tool" --target "$TARGET"
    else
        cargo build --release -p "$tool"
    fi
done

if [ -n "$ORGANIZE_DIR" ]; then
    echo "Organizing host tools into $ORGANIZE_DIR..."
    
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

    mkdir -p "$ORGANIZE_DIR/release/$PLATFORM"
    for tool in "${TOOL_PACKAGES[@]}"; do
        if [ -f "$RELEASE_DIR/$tool" ]; then
            cp "$RELEASE_DIR/$tool" "$ORGANIZE_DIR/release/$PLATFORM/$tool"
        elif [ -f "$RELEASE_DIR/$tool.exe" ]; then
            cp "$RELEASE_DIR/$tool.exe" "$ORGANIZE_DIR/release/$PLATFORM/$tool.exe"
        fi
    done
fi

if [ -n "$ZIP_FILE" ]; then
    echo "Creating zip archive $ZIP_FILE..."
    ABS_ZIP_FILE=""
    if [[ "$ZIP_FILE" = /* ]]; then
        ABS_ZIP_FILE="$ZIP_FILE"
    else
        ABS_ZIP_FILE="$WORKSPACE_ROOT/$ZIP_FILE"
    fi

    STAGE_DIR=$(mktemp -d)
    for tool in "${TOOL_PACKAGES[@]}"; do
        if [ -f "$RELEASE_DIR/$tool" ]; then
            cp "$RELEASE_DIR/$tool" "$STAGE_DIR/"
        elif [ -f "$RELEASE_DIR/$tool.exe" ]; then
            cp "$RELEASE_DIR/$tool.exe" "$STAGE_DIR/"
        fi
    done
    PYTHON_BIN=""
    if command -v python3 >/dev/null 2>&1; then
        PYTHON_BIN="python3"
    elif command -v python >/dev/null 2>&1; then
        PYTHON_BIN="python"
    else
        echo "Error: Python was not found to execute tools/helpers/zip_folder.py!" >&2
        exit 1
    fi
    "$PYTHON_BIN" "$WORKSPACE_ROOT/tools/helpers/zip_folder.py" "$STAGE_DIR" "$ABS_ZIP_FILE"
    rm -rf "$STAGE_DIR"
fi
