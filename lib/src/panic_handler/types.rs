use core::fmt::Write;

/// Global circular log buffer for crash logging.
pub struct LogBuffer {
    /// Internal byte buffer.
    pub buffer: [u8; 1024],
    /// Current write head.
    pub head: usize,
    /// Whether the buffer has wrapped around.
    pub wrapped: bool,
}

impl LogBuffer {
    /// Creates a new empty LogBuffer.
    pub const fn new() -> Self {
        Self {
            buffer: [0u8; 1024],
            head: 0,
            wrapped: false,
        }
    }
}

impl Default for LogBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Write for LogBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        for &b in bytes {
            self.buffer[self.head] = b;
            self.head += 1;
            if self.head >= 1024 {
                self.head = 0;
                self.wrapped = true;
            }
        }
        Ok(())
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Configuration parameters for the panic handler
pub struct PanicConfig {
    /// Flash peripheral instance
    pub flash: embassy_rp::peripherals::FLASH,
    /// Offset range in flash partition used for filesystem
    pub range: core::ops::Range<u32>,
}
