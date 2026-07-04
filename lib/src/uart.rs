//! Reusable hardware bringup shell console infrastructure.

#![deny(missing_docs)]

use embassy_rp::uart::{Instance, Mode, UartRx, UartTx};
use embedded_io::Write as IoWrite;

/// Helper adapter to wrap embassy-rp UartTx and expose embedded-io::Write.
pub struct UartWriter<'d, T: Instance, M: Mode> {
    uart: UartTx<'d, T, M>,
}

impl<'d, T: Instance, M: Mode> UartWriter<'d, T, M> {
    /// Wrap a raw UartTx device.
    pub const fn new(uart: UartTx<'d, T, M>) -> Self {
        Self { uart }
    }
}

impl<'d, T: Instance, M: Mode> embedded_io::ErrorType for UartWriter<'d, T, M> {
    type Error = core::convert::Infallible;
}

impl<'d, T: Instance, M: Mode> IoWrite for UartWriter<'d, T, M> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let _ = self.uart.blocking_write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Helper function to execute the blocking UART character receive loop,
/// routing bytes into the embedded-cli processor.
pub fn run_uart_shell_loop<
    'a,
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
    C: embedded_cli::service::Autocomplete + embedded_cli::service::Help,
    P: embedded_cli::service::CommandProcessor<W, E>,
    T: Instance,
    M: Mode,
    B1: embedded_cli::buffer::Buffer,
    B2: embedded_cli::buffer::Buffer,
>(
    cli: &mut embedded_cli::cli::Cli<W, E, B1, B2>,
    rx: &mut UartRx<'a, T, M>,
    processor: &mut P,
) -> ! {
    loop {
        let mut rx_byte = [0u8; 1];
        if rx.blocking_read(&mut rx_byte).is_ok() {
            let _ = cli.process_byte::<C, _>(rx_byte[0], processor);
        }
    }
}
