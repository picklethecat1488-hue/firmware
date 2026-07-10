use cat_detector::shell_controller::{CliCommand, MotorCalState, SensorDirection};
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

struct TestProcessor {
    cmd: Option<CliCommand>,
}

impl<W: embedded_io::Write<Error = E>, E: embedded_io::Error> CommandProcessor<W, E>
    for TestProcessor
{
    fn process<'a>(
        &mut self,
        _cli: &mut embedded_cli::cli::CliHandle<'_, W, E>,
        raw: RawCommand<'a>,
    ) -> Result<(), ProcessError<'a, E>> {
        self.cmd = CliCommand::parse(raw).ok();
        Ok(())
    }
}

#[test]
fn test_crash_command_parsing() {
    let mut cli = CliBuilder::default().writer(DummyWriter).build().unwrap();

    let mut processor = TestProcessor { cmd: None };
    for byte in b"crash\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor);
    }

    assert!(matches!(processor.cmd, Some(CliCommand::Crash)));
}

#[test]
fn test_stop_command_parsing() {
    let mut cli = CliBuilder::default().writer(DummyWriter).build().unwrap();

    let mut processor = TestProcessor { cmd: None };
    for byte in b"stop\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor);
    }

    assert!(matches!(processor.cmd, Some(CliCommand::Stop)));
}

#[test]
fn test_cal_near_command_parsing() {
    let mut cli = CliBuilder::default().writer(DummyWriter).build().unwrap();

    let mut processor = TestProcessor { cmd: None };
    for byte in b"cal_near north\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor);
    }

    assert!(matches!(
        processor.cmd,
        Some(CliCommand::CalNear {
            direction: SensorDirection::North
        })
    ));
}

#[test]
fn test_cal_far_command_parsing() {
    let mut cli = CliBuilder::default().writer(DummyWriter).build().unwrap();

    let mut processor = TestProcessor { cmd: None };
    for byte in b"cal_far east\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor);
    }

    assert!(matches!(
        processor.cmd,
        Some(CliCommand::CalFar {
            direction: SensorDirection::East
        })
    ));
}

#[test]
fn test_cal_motor_command_parsing() {
    let mut cli = CliBuilder::default().writer(DummyWriter).build().unwrap();

    let mut processor = TestProcessor { cmd: None };
    for byte in b"cal_motor empty\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor);
    }

    assert!(matches!(
        processor.cmd,
        Some(CliCommand::CalMotor {
            state: MotorCalState::Empty,
            max_rpm: None,
            rpm_limit: None
        })
    ));

    // Parsing with physical max RPM
    let mut processor2 = TestProcessor { cmd: None };
    for byte in b"cal_motor empty 3000\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor2);
    }

    assert!(matches!(
        processor2.cmd,
        Some(CliCommand::CalMotor {
            state: MotorCalState::Empty,
            max_rpm: Some(3000),
            rpm_limit: None
        })
    ));

    // Parsing with both physical max RPM and safety RPM limit
    let mut processor3 = TestProcessor { cmd: None };
    for byte in b"cal_motor empty 3000 2500\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor3);
    }

    assert!(matches!(
        processor3.cmd,
        Some(CliCommand::CalMotor {
            state: MotorCalState::Empty,
            max_rpm: Some(3000),
            rpm_limit: Some(2500)
        })
    ));

    // Parsing with overload state
    let mut processor4 = TestProcessor { cmd: None };
    for byte in b"cal_motor overload\n" {
        let _ = cli.process_byte::<CliCommand, _>(*byte, &mut processor4);
    }

    assert!(matches!(
        processor4.cmd,
        Some(CliCommand::CalMotor {
            state: MotorCalState::Overload,
            max_rpm: None,
            rpm_limit: None
        })
    ));
}
