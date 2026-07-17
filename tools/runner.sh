#!/bin/bash
set -e
ELF_PATH="$1"

# Find the repository root dynamically
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
REPO_ROOT=$(dirname "$SCRIPT_DIR")

# Build host_cli first to ensure quick execution of the chip query
cargo build --manifest-path "$REPO_ROOT/Cargo.toml" -p host_cli --release

# Query chip name from ELF metadata
CHIP=$("$REPO_ROOT/target/release/host_cli" --elf "$ELF_PATH" --print-chip)

probe-rs download --chip "$CHIP" "$ELF_PATH"
probe-rs reset --chip "$CHIP"
# Unset cargo env vars so cargo run for the host doesn't inherit ARM configurations
exec env -u CARGO_ENCODED_RUSTFLAGS -u CARGO_BUILD_TARGET "$REPO_ROOT/target/release/host_cli" --elf "$ELF_PATH" "${@:2}"
