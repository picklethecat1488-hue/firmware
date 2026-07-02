use cat_detector::CliCommand;
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
            direction: cat_detector::SensorDirection::North
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
            direction: cat_detector::SensorDirection::East
        })
    ));
}
