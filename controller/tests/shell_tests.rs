use controller::shell_controller::DefaultShellCli as CliCommand;
use embedded_cli::cli::CliBuilder;
use embedded_cli::command::RawCommand;
use embedded_cli::service::{CommandProcessor, FromRaw, ProcessError};

struct DummyWriter;
impl embedded_io::ErrorType for DummyWriter {
    type Error = core::convert::Infallible;
}
impl embedded_io::Write for DummyWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct TestProcessor<'a> {
    cmd: Option<CliCommand<'a>>,
}

impl<'a, W: embedded_io::Write<Error = E>, E: embedded_io::Error> CommandProcessor<W, E>
    for TestProcessor<'a>
{
    fn process<'b>(
        &mut self,
        _cli: &mut embedded_cli::cli::CliHandle<'_, W, E>,
        raw: RawCommand<'b>,
    ) -> Result<(), ProcessError<'b, E>> {
        // Since FromRaw parses with the raw lifetime, transmute or cast to match 'a is safe
        // because we only assert on parsed values within the lifetime of the raw command.
        // We can parse directly into the owned option.
        let parsed = CliCommand::parse(raw).ok();
        // Safe transmutation of lifetime because 'b is valid for the duration of the test run-loop step.
        self.cmd = unsafe {
            core::mem::transmute::<Option<CliCommand<'b>>, Option<CliCommand<'a>>>(parsed)
        };
        Ok(())
    }
}

#[test]
fn test_crash_command_parsing() {
    let mut cli = CliBuilder::default().writer(DummyWriter).build().unwrap();

    let mut processor = TestProcessor { cmd: None };
    for byte in b"system crash\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor);
    }

    assert!(matches!(
        processor.cmd,
        Some(CliCommand::System {
            subcommand: Some("crash")
        })
    ));
}

#[test]
fn test_stop_command_parsing() {
    let mut cli = CliBuilder::default().writer(DummyWriter).build().unwrap();

    let mut processor = TestProcessor { cmd: None };
    for byte in b"motor stop\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor);
    }

    assert!(matches!(
        processor.cmd,
        Some(CliCommand::Motor {
            subcommand: Some("stop"),
            ..
        })
    ));
}

#[test]
fn test_cal_near_command_parsing() {
    let mut cli = CliBuilder::default().writer(DummyWriter).build().unwrap();

    let mut processor = TestProcessor { cmd: None };
    for byte in b"sensor cal_near north\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor);
    }

    assert!(matches!(
        processor.cmd,
        Some(CliCommand::Sensor {
            subcommand: Some("cal_near"),
            arg1: Some("north")
        })
    ));
}

#[test]
fn test_cal_far_command_parsing() {
    let mut cli = CliBuilder::default().writer(DummyWriter).build().unwrap();

    let mut processor = TestProcessor { cmd: None };
    for byte in b"sensor cal_far east\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor);
    }

    assert!(matches!(
        processor.cmd,
        Some(CliCommand::Sensor {
            subcommand: Some("cal_far"),
            arg1: Some("east")
        })
    ));
}

#[test]
fn test_cal_motor_command_parsing() {
    let mut cli = CliBuilder::default().writer(DummyWriter).build().unwrap();

    let mut processor = TestProcessor { cmd: None };
    for byte in b"motor calibrate empty\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor);
    }

    assert!(matches!(
        processor.cmd,
        Some(CliCommand::Motor {
            subcommand: Some("calibrate"),
            arg1: Some("empty"),
            arg2: None,
            arg3: None
        })
    ));

    // Parsing with physical max RPM
    let mut processor2 = TestProcessor { cmd: None };
    for byte in b"motor calibrate empty 3000\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor2);
    }

    assert!(matches!(
        processor2.cmd,
        Some(CliCommand::Motor {
            subcommand: Some("calibrate"),
            arg1: Some("empty"),
            arg2: Some("3000"),
            arg3: None
        })
    ));

    // Parsing with both physical max RPM and safety RPM limit
    let mut processor3 = TestProcessor { cmd: None };
    for byte in b"motor calibrate empty 3000 2500\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor3);
    }

    assert!(matches!(
        processor3.cmd,
        Some(CliCommand::Motor {
            subcommand: Some("calibrate"),
            arg1: Some("empty"),
            arg2: Some("3000"),
            arg3: Some("2500")
        })
    ));

    // Parsing with overload state
    let mut processor4 = TestProcessor { cmd: None };
    for byte in b"motor calibrate overload\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor4);
    }

    assert!(matches!(
        processor4.cmd,
        Some(CliCommand::Motor {
            subcommand: Some("calibrate"),
            arg1: Some("overload"),
            arg2: None,
            arg3: None
        })
    ));
}
