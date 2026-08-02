# Code Generator (`code_gen`)

`code_gen` is a host-side utility tool to inspect and generate the Rust Embassy task runner macros, channel definitions, spawning logic for the system controllers, and CLI shell resolvers. It reads and deserializes [controller/controllers.toml](../../controller/controllers.toml) and [controller/shell.toml](../../controller/shell.toml) and renders code blocks using the unified Rinja templates.

## Usage

You can run the tool directly using `cargo run`:

```bash
# Show help
cargo run -p code_gen -- --help

# List all defined controllers
cargo run -p code_gen -- list-controllers

# List all defined CLI commands/groups
cargo run -p code_gen -- list-clis

# Generate and print the Rust macros/channels for a specific controller (e.g., Led) to stdout
cargo run -p code_gen -- Led

# Generate and write compiling sample CLI wrapper code to target/out/sample_cli.rs
cargo run -p code_gen -- cli-sample

# Generate specific CLI subcommand handler skeletons (e.g., Motor, Battery)
cargo run -p code_gen -- cli-sample Motor Battery

# Generate all boilerplate runloops under target/out/sample_runloops.rs
cargo run -p code_gen -- runloop-sample

# Generate a specific controller's runloop boilerplate (e.g., Motor) under target/out/motor_runloop.rs
cargo run -p code_gen -- runloop-sample Motor

# Specify a custom output directory using --out-dir
cargo run -p code_gen -- cli-sample Motor --out-dir target/out/my_custom_dir
```

## How It Works

1. It searches upward from the current working directory to locate the `controller/controllers.toml` and `controller/shell.toml` metadata registries.
2. It parses the metadata configuration and infers the necessary receiver, telemetry, and system-specific channel types.
3. For print/inspect actions (e.g. `cargo run -p code_gen -- Led`), it renders to `stdout`.
4. For file-generation actions (e.g. `cli-sample`, `runloop-sample`), it renders the corresponding Rinja templates and writes the files under the designated `--out-dir` (defaulting to `target/out`), displaying generation progress with an `indicatif` spinner.

