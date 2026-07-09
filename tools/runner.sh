#!/bin/bash
set -e
ELF_PATH="$1"
probe-rs download --chip RP2040 "$ELF_PATH"
# Unset cargo env vars so cargo run for the host doesn't inherit ARM configurations
exec env -u CARGO_ENCODED_RUSTFLAGS -u CARGO_BUILD_TARGET cargo run --manifest-path /Users/daparker/gh/firmware/Cargo.toml -p host_cli --release -- --elf "$ELF_PATH" 
