# Code Generator (`code_gen`)

`code_gen` is a host-side utility tool to inspect and generate the Rust Embassy task runner macros, channel definitions, spawning logic for the system controllers, and CLI shell resolvers. It reads and deserializes [controller/controllers.toml](../../controller/controllers.toml) and [controller/shell.toml](../../controller/shell.toml) and renders code blocks using the unified Rinja templates.

## Usage

You can run the tool directly using `cargo run`:

```bash
# Show help
cargo run -p code_gen -- --help

# List all defined controllers
cargo run -p code_gen -- list

# Generate and print the Rust code block for a specific controller (e.g., Led)
cargo run -p code_gen -- Led

# Generate and print the code blocks for all controllers
cargo run -p code_gen

# Generate and print a compiling sample CLI implementation
cargo run -p code_gen -- cli-sample
```

## How It Works

1. It searches upward from the current working directory to locate the `controller/controllers.toml` and `controller/shell.toml` metadata registries.
2. It parses the metadata configuration and infers the necessary receiver, telemetry, and system-specific channel types.
3. It filters the controllers list based on the optional command line argument.
4. It renders the matching templates to `stdout`.
