#!/bin/bash
set -e
ELF_PATH="$1"
# Build host_cli first to ensure quick execution of the chip query
cargo build --manifest-path /Users/daparker/gh/firmware/Cargo.toml -p host_cli --release

# Query chip name from ELF metadata
CHIP=$(cargo run --manifest-path /Users/daparker/gh/firmware/Cargo.toml -p host_cli --quiet --release -- --elf "$ELF_PATH" --print-chip)

probe-rs download --chip "$CHIP" "$ELF_PATH"
probe-rs reset --chip "$CHIP"
# Unset cargo env vars so cargo run for the host doesn't inherit ARM configurations
exec env -u CARGO_ENCODED_RUSTFLAGS -u CARGO_BUILD_TARGET cargo run --manifest-path /Users/daparker/gh/firmware/Cargo.toml -p host_cli --release -- --elf "$ELF_PATH" "${@:2}"
