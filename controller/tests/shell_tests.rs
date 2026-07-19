controller::declare_shell_commands! {
    CliCommand (CliCommandProcessor) {
        Battery,
        Thermal,
        Motor,
        Sensor,
        Fs,
        System,
        Core1,
    }
}
use controller::motor_controller::MotorSubcommand;
use controller::sensor_controller::SensorSubcommand;
use controller::shell_controller::Core1Subcommand;
use controller::system_controller::SystemSubcommand;
use embedded_cli::cli::CliBuilder;
use embedded_cli::command::RawCommand;
use embedded_cli::service::{CommandProcessor, FromRaw, ProcessError};

fn handle_core1_cli<
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
    C: controller::ShellConfig,
>(
    _ctrl: &mut controller::shell_controller::ShellController<'_, C>,
    _subcommand: Option<Core1Subcommand>,
    _writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    Ok(())
}

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
            subcommand: Some(SystemSubcommand::Crash)
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
            subcommand: Some(MotorSubcommand::Stop),
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
            subcommand: Some(SensorSubcommand::CalNear),
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
            subcommand: Some(SensorSubcommand::CalFar),
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
            subcommand: Some(MotorSubcommand::Calibrate),
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
            subcommand: Some(MotorSubcommand::Calibrate),
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
            subcommand: Some(MotorSubcommand::Calibrate),
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
            subcommand: Some(MotorSubcommand::Calibrate),
            arg1: Some("overload"),
            arg2: None,
            arg3: None
        })
    ));
}

#[test]
fn test_core1_panic_command_parsing() {
    let mut cli = CliBuilder::default().writer(DummyWriter).build().unwrap();

    let mut processor = TestProcessor { cmd: None };
    for byte in b"core1 panic\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor);
    }

    assert!(matches!(
        processor.cmd,
        Some(CliCommand::Core1 {
            subcommand: Some(Core1Subcommand::Panic)
        })
    ));
}
