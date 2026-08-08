use embedded_cli::arguments::FromArgument;
use platform::subcommand_enum;

subcommand_enum! {
    /// Test subcommand enum
    pub enum TestSubcommand {
        /// Default variant
        First,
        /// Custom string override
        Second = "custom_second",
        /// Another default
        Third,
    }
    "Invalid test subcommand. Expected: first, custom_second, third"
}

#[test]
fn test_subcommand_enum_parsing() {
    // 1. Test exact matches
    assert_eq!(
        TestSubcommand::from_arg("First").ok(),
        Some(TestSubcommand::First)
    );
    assert_eq!(
        TestSubcommand::from_arg("first").ok(),
        Some(TestSubcommand::First)
    );
    assert_eq!(
        TestSubcommand::from_arg("FIRST").ok(),
        Some(TestSubcommand::First)
    );

    // 2. Test custom override matches (case-insensitively)
    assert_eq!(
        TestSubcommand::from_arg("custom_second").ok(),
        Some(TestSubcommand::Second)
    );
    assert_eq!(
        TestSubcommand::from_arg("CUSTOM_SECOND").ok(),
        Some(TestSubcommand::Second)
    );

    // 3. Test that stringified variant name is NOT matched for custom override
    assert!(TestSubcommand::from_arg("Second").is_err());

    // 4. Test another default match
    assert_eq!(
        TestSubcommand::from_arg("third").ok(),
        Some(TestSubcommand::Third)
    );

    // 5. Test invalid command error message
    let err = TestSubcommand::from_arg("invalid").unwrap_err();
    assert_eq!(err.value, "invalid");
    assert_eq!(
        err.expected,
        "Invalid test subcommand. Expected: first, custom_second, third"
    );
}

struct MockI2c {
    active_address: u8,
}

impl embedded_hal::i2c::ErrorType for MockI2c {
    type Error = embedded_hal::i2c::ErrorKind;
}

impl embedded_hal::i2c::I2c for MockI2c {
    fn read(&mut self, _address: u8, _read: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn write(&mut self, address: u8, _write: &[u8]) -> Result<(), Self::Error> {
        if address == self.active_address {
            Ok(())
        } else {
            Err(embedded_hal::i2c::ErrorKind::NoAcknowledge(
                embedded_hal::i2c::NoAcknowledgeSource::Address,
            ))
        }
    }

    fn write_read(
        &mut self,
        _address: u8,
        _write: &[u8],
        _read: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct MockI2cResolver {
    i2c: core::cell::RefCell<MockI2c>,
}

impl platform::i2c::I2cResolver for MockI2cResolver {
    type I2c = MockI2c;
    fn resolve_i2c(&self, _name: Option<&str>) -> Result<&mut Self::I2c, &'static str> {
        Ok(unsafe { &mut *self.i2c.as_ptr() })
    }
}

struct MockWriter {
    buf: heapless::Vec<u8, 2048>,
}

impl embedded_io::ErrorType for MockWriter {
    type Error = core::convert::Infallible;
}

impl embedded_io::Write for MockWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let _ = self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn test_i2c_scan() {
    let resolver = MockI2cResolver {
        i2c: core::cell::RefCell::new(MockI2c {
            active_address: 0x3c,
        }),
    };

    let mut mock_write = MockWriter {
        buf: heapless::Vec::new(),
    };

    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = platform::i2c::handle_i2c_cli(
            &resolver,
            Some(platform::i2c::I2cSubcommand::Scan),
            None,
            &mut writer,
        );
        assert!(res.is_ok());
    }

    let output_str = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(output_str.contains("Scanning I2C bus..."));
    assert!(output_str.contains("3c"));
    assert!(!output_str.contains("78"));
}

#[test]
fn test_gpio_cli() {
    let mock_resolver = ();
    let mut mock_write = MockWriter {
        buf: heapless::Vec::new(),
    };

    // Test Gpio status
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = platform::gpio::handle_gpio_cli(
            &mock_resolver,
            Some(platform::gpio::GpioSubcommand::Status),
            None,
            &mut writer,
        );
        assert!(res.is_ok());
    }
    let output_str = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(output_str.contains("GPIO Pin Status") || output_str.contains("Mock GPIO"));

    // Test Gpio read
    mock_write.buf.clear();
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = platform::gpio::handle_gpio_cli(
            &mock_resolver,
            Some(platform::gpio::GpioSubcommand::Read),
            Some(5),
            &mut writer,
        );
        assert!(res.is_ok());
    }
    let output_str = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(output_str.contains("GP5"));

    // Test Gpio read whole bank (no pin)
    mock_write.buf.clear();
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = platform::gpio::handle_gpio_cli(
            &mock_resolver,
            Some(platform::gpio::GpioSubcommand::Read),
            None,
            &mut writer,
        );
        assert!(res.is_ok());
    }
    let output_str = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(output_str.contains("GP0") && output_str.contains("GP29"));
}
