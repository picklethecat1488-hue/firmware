use code_gen::ShellConfigToml;

/// Handles listing all defined CLI commands/groups.
pub fn handle(shell_config: &ShellConfigToml) {
    for cmd in &shell_config.cli_commands {
        println!("{}", cmd.group);
    }
}
