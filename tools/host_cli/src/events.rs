//! Structured NDJSON event protocol, RTT shell-idle flag poller, and CLI chunk parser for `host_cli --json-events`.

use serde_json::json;
use tool_common::decode_rtt_shell_flags;

/// ANSI erase-line control sequence emitted by `embedded-cli` when redrawing its input/prompt line.
pub const ANSI_CLEAR_LINE: &str = "\x1b[2K";

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
    /// Complete target CLI output line (excluding line-editor redraws).
    CliOutput {
        /// Cleaned single line of CLI output text.
        text: String,
    },
    /// Emitted when the target's RTT control flag (`RTT_SHELL_IDLE_FLAG`) indicates the shell is idle.
    ShellIdle {
        /// Wrapping 16-bit idle-transition epoch read from `_SEGGER_RTT.down_channels[0].flags`.
        epoch: u16,
    },
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
            Self::ShellIdle { epoch } => json!({
                "event": "shell_idle",
                "epoch": epoch,
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

/// Stateful poller tracking `_SEGGER_RTT.down_channels[0].flags` idle transitions and epochs.
#[derive(Default, Debug, Clone)]
pub struct ShellIdlePoller {
    last_emitted_epoch: Option<u16>,
}

impl ShellIdlePoller {
    /// Construct a new [`ShellIdlePoller`].
    pub fn new() -> Self {
        Self {
            last_emitted_epoch: None,
        }
    }

    /// Evaluate the raw 32-bit `down_channels[0].flags` word alongside `cli_up_empty` and `cli_down_empty`.
    ///
    /// Emits `Some(epoch)` strictly once per idle epoch after both up and down CLI ring buffers are drained.
    pub fn poll(
        &mut self,
        down_flags: u32,
        cli_up_empty: bool,
        cli_down_empty: bool,
    ) -> Option<u16> {
        if !cli_up_empty || !cli_down_empty {
            return None;
        }
        let (is_idle, epoch) = decode_rtt_shell_flags(down_flags);
        if !is_idle {
            return None;
        }
        if self.last_emitted_epoch == Some(epoch) {
            return None;
        }
        self.last_emitted_epoch = Some(epoch);
        Some(epoch)
    }
}

/// Stateful line parser assembling arbitrary RTT CLI byte chunks into [`HostCliEvent::CliOutput`] lines.
#[derive(Default, Debug)]
pub struct CliChunkParser {
    pending: String,
    in_editor_redraw: bool,
}

impl CliChunkParser {
    /// Construct a new empty [`CliChunkParser`].
    pub fn new() -> Self {
        Self {
            pending: String::new(),
            in_editor_redraw: false,
        }
    }

    /// Push echo-cancelled RTT CLI bytes and return completed [`HostCliEvent::CliOutput`] events.
    pub fn push_bytes(&mut self, bytes: &[u8]) -> Vec<HostCliEvent> {
        if bytes.is_empty() {
            return Vec::new();
        }
        self.pending.push_str(&String::from_utf8_lossy(bytes));
        let mut events = Vec::new();

        while let Some(nl_pos) = self.pending.find('\n') {
            let raw_line: String = self.pending.drain(..=nl_pos).collect();
            let had_clear = raw_line.contains(ANSI_CLEAR_LINE);
            let cleaned = raw_line
                .trim_end_matches(['\r', '\n'])
                .replace(ANSI_CLEAR_LINE, "");
            if self.in_editor_redraw {
                self.in_editor_redraw = false;
                continue;
            }
            if had_clear && cleaned.trim().is_empty() {
                // `embedded-cli` emits `\x1b[2K\r\n<prompt>` when drawing a multi-char prompt:
                // mark the unterminated tail immediately following `\x1b[2K\r\n` as editor redraw.
                self.in_editor_redraw = true;
                continue;
            }
            if cleaned.trim().is_empty() {
                continue;
            }
            events.push(HostCliEvent::CliOutput { text: cleaned });
        }

        if self.pending.contains(ANSI_CLEAR_LINE) {
            let stripped = self.pending.replace(ANSI_CLEAR_LINE, "").replace('\r', "");
            if stripped.is_empty() {
                self.pending.clear();
            } else {
                self.in_editor_redraw = true;
            }
        }

        events
    }

    /// Discard any unterminated prompt redraw sitting in `pending` when `ShellIdlePoller` observes idle,
    /// and return the structured [`HostCliEvent::ShellIdle`] event.
    pub fn on_shell_idle(&mut self, epoch: u16) -> HostCliEvent {
        self.pending.clear();
        self.in_editor_redraw = false;
        HostCliEvent::ShellIdle { epoch }
    }

    /// Flush any trailing non-empty output line on session teardown.
    pub fn flush_remaining(&mut self) -> Vec<HostCliEvent> {
        if self.pending.is_empty() || self.in_editor_redraw {
            self.pending.clear();
            self.in_editor_redraw = false;
            return Vec::new();
        }
        let cleaned = std::mem::take(&mut self.pending)
            .replace(ANSI_CLEAR_LINE, "")
            .trim_matches(['\r', '\n'])
            .to_string();
        if cleaned.trim().is_empty() {
            Vec::new()
        } else {
            vec![HostCliEvent::CliOutput { text: cleaned }]
        }
    }
}
