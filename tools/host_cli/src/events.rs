//! Structured NDJSON event protocol and stateful RTT CLI chunk parser for `host_cli --json-events`.

use serde_json::json;

/// ANSI erase-line control sequence emitted by `embedded-cli` before redrawing prompts.
pub const ANSI_CLEAR_LINE: &str = "\x1b[2K";

/// Canonical embedded shell prompt token.
pub const SHELL_PROMPT_TOKEN: &str = "shell>";

/// Structured single-line JSON event emitted by `host_cli --json-events` on stdout.
#[derive(Clone, Debug, PartialEq)]
pub enum HostCliEvent {
    /// Emitted once RTT channels have been resolved and bound on the target.
    SessionStarted {
        /// Target chip identifier (e.g. `"rp2040"` or `"MCXN947"`).
        chip: String,
        /// Whether Up Channel 0 (`defmt`) is active.
        defmt: bool,
        /// Whether Up Channel 1 (`cli`) is active.
        cli_up: bool,
        /// Whether Down Channel 0 (`cli`) is active.
        cli_down: bool,
    },
    /// Decoded target `defmt` log frame.
    Defmt {
        /// Log severity level (`"TRACE"`, `"DEBUG"`, `"INFO"`, `"WARN"`, `"ERROR"`).
        level: String,
        /// Optional formatted Rust module namespace (e.g. `"controller::shell_controller"`).
        module: Option<String>,
        /// Optional target Embassy timestamp in seconds.
        device_ts: Option<f64>,
        /// Plain decoded message text without ANSI escape sequences.
        message: String,
        /// Colorized / namespace-annotated display string for UI log streams.
        formatted: String,
    },
    /// Complete target CLI output line (excluding shell prompts and empty clears).
    CliOutput {
        /// Cleaned single line of CLI output text.
        text: String,
    },
    /// Emitted immediately whenever the target shell restores its `shell> ` prompt.
    CliPrompt,
    /// Emitted when `controller::shell_controller` logs synchronous or async command completion.
    CommandCompleted {
        /// True when the command succeeded, false when `command execution failed` was logged.
        success: bool,
        /// Optional failure reason parsed from `command execution failed: <reason>`.
        error: Option<String>,
        /// Optional target Embassy timestamp in seconds.
        device_ts: Option<f64>,
    },
    /// Emitted when the active RTT session disconnects or terminates.
    SessionDisconnected {
        /// Optional error string if the session terminated due to a probe/read error.
        error: Option<String>,
    },
}

impl HostCliEvent {
    /// Serialize this event into a single-line JSON string (without trailing newline).
    pub fn to_json_line(&self) -> String {
        let value = match self {
            Self::SessionStarted {
                chip,
                defmt,
                cli_up,
                cli_down,
            } => json!({
                "event": "session_started",
                "chip": chip,
                "defmt": defmt,
                "cli_up": cli_up,
                "cli_down": cli_down,
            }),
            Self::Defmt {
                level,
                module,
                device_ts,
                message,
                formatted,
            } => json!({
                "event": "defmt",
                "level": level,
                "module": module,
                "device_ts": device_ts,
                "message": message,
                "formatted": formatted,
            }),
            Self::CliOutput { text } => json!({
                "event": "cli_output",
                "text": text,
            }),
            Self::CliPrompt => json!({
                "event": "cli_prompt",
            }),
            Self::CommandCompleted {
                success,
                error,
                device_ts,
            } => json!({
                "event": "command_completed",
                "success": success,
                "error": error,
                "device_ts": device_ts,
            }),
            Self::SessionDisconnected { error } => json!({
                "event": "session_disconnected",
                "error": error,
            }),
        };
        value.to_string()
    }
}

/// Extract the severity token (`TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`) from a `defmt` plain line.
pub fn extract_defmt_level(plain_line: &str) -> &'static str {
    for lvl in ["ERROR", "WARN", "INFO", "DEBUG", "TRACE"] {
        if plain_line.split_whitespace().take(3).any(|tok| tok == lvl) {
            return lvl;
        }
    }
    "INFO"
}

/// Inspect a decoded `defmt` frame for shell command completion markers.
pub fn detect_shell_completion_event(
    plain_line: &str,
    device_ts_sec: Option<f64>,
) -> Option<HostCliEvent> {
    if plain_line.contains("command execution succeeded") {
        return Some(HostCliEvent::CommandCompleted {
            success: true,
            error: None,
            device_ts: device_ts_sec,
        });
    }
    if let Some(idx) = plain_line.find("command execution failed") {
        let suffix = plain_line[idx + "command execution failed".len()..]
            .trim_start_matches(':')
            .trim();
        return Some(HostCliEvent::CommandCompleted {
            success: false,
            error: if suffix.is_empty() {
                None
            } else {
                Some(suffix.to_string())
            },
            device_ts: device_ts_sec,
        });
    }
    None
}

/// Stateful line & prompt parser assembling arbitrary RTT CLI byte chunks into [`HostCliEvent`]s.
#[derive(Default, Debug)]
pub struct CliChunkParser {
    pending: String,
}

impl CliChunkParser {
    /// Construct a new empty [`CliChunkParser`].
    pub fn new() -> Self {
        Self {
            pending: String::new(),
        }
    }

    /// Push echo-cancelled RTT CLI bytes and return any completed output line and prompt events.
    pub fn push_bytes(&mut self, bytes: &[u8]) -> Vec<HostCliEvent> {
        if bytes.is_empty() {
            return Vec::new();
        }
        self.pending.push_str(&String::from_utf8_lossy(bytes));
        let mut events = Vec::new();

        while let Some(nl_pos) = self.pending.find('\n') {
            let raw_line: String = self.pending.drain(..=nl_pos).collect();
            let cleaned = raw_line
                .trim_end_matches(['\r', '\n'])
                .replace(ANSI_CLEAR_LINE, "");
            let trimmed = cleaned.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed == SHELL_PROMPT_TOKEN {
                events.push(HostCliEvent::CliPrompt);
            } else if let Some(after_prompt) = cleaned
                .strip_prefix("shell> ")
                .or_else(|| cleaned.strip_prefix(SHELL_PROMPT_TOKEN))
            {
                events.push(HostCliEvent::CliPrompt);
                if !after_prompt.trim().is_empty() {
                    events.push(HostCliEvent::CliOutput {
                        text: after_prompt.to_string(),
                    });
                }
            } else {
                events.push(HostCliEvent::CliOutput { text: cleaned });
            }
        }

        let unescaped_tail = self.pending.replace(ANSI_CLEAR_LINE, "").replace('\r', "");
        if unescaped_tail.is_empty() {
            // Empty `\x1b[2K\r` (`cli.set_prompt("")` when an async command begins): drop without prompt event!
            self.pending.clear();
        } else if unescaped_tail == "shell> " || unescaped_tail == SHELL_PROMPT_TOKEN {
            self.pending.clear();
            events.push(HostCliEvent::CliPrompt);
        }

        events
    }

    /// Flush any trailing non-empty partial line on session teardown.
    pub fn flush_remaining(&mut self) -> Vec<HostCliEvent> {
        if self.pending.is_empty() {
            return Vec::new();
        }
        let cleaned = std::mem::take(&mut self.pending)
            .replace(ANSI_CLEAR_LINE, "")
            .trim_matches(['\r', '\n'])
            .to_string();
        if cleaned.trim().is_empty() {
            Vec::new()
        } else if cleaned.trim() == SHELL_PROMPT_TOKEN {
            vec![HostCliEvent::CliPrompt]
        } else {
            vec![HostCliEvent::CliOutput { text: cleaned }]
        }
    }
}
