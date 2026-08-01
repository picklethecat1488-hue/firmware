# Controller Code Generator (`controller_gen`)

`controller_gen` is a host-side utility tool to inspect and generate the Rust Embassy task runner macros, channel definitions, and spawning logic for the system controllers. It reads and deserializes [controller/controllers.toml](../../controller/controllers.toml) and renders code blocks using the unified [generated_controllers.rs.jinja](../../controller/templates/generated_controllers.rs.jinja) Rinja template.

## Usage

You can run the tool directly using `cargo run`:

```bash
# Show help
cargo run -p controller_gen -- --help

# List all defined controllers
cargo run -p controller_gen -- list

# Generate and print the Rust code block for a specific controller (e.g., Led)
cargo run -p controller_gen -- Led

# Generate and print the code blocks for all controllers
cargo run -p controller_gen
```

## How It Works

1. It searches upward from the current working directory to locate the `controller/controllers.toml` metadata registry.
2. It parses the metadata configuration and infers the necessary receiver, telemetry, and system-specific channel types.
3. It filters the controllers list based on the optional command line argument.
4. It renders the matching portion of the template to `stdout`.
