use embedded_cli::arguments::FromArgument;
use platform::mock::{MockI2c, MockI2cResolver, MockWriter};
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

#[test]
fn test_core_monitor_cli() {
    let resolver = ();

    let mut mock_write = MockWriter {
        buf: heapless::Vec::new(),
    };

    // 1. Test status subcommand (happy path)
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = platform::core_monitor::handle_core_monitor_cli(
            &resolver,
            Some(platform::core_monitor::CoreMonitorSubcommand::Status),
            None,
            &mut writer,
        );
        assert!(res.is_ok());
    }
    let output_str = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(output_str.contains("Core Monitor Status:"));
    assert!(output_str.contains("Core0"));

    // 2. Test crash core0 subcommand (happy path)
    mock_write.buf.clear();
    platform::core_monitor::LAST_PANICKED_CORE.with(|cell| cell.set(None));
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = platform::core_monitor::handle_core_monitor_cli(
            &resolver,
            Some(platform::core_monitor::CoreMonitorSubcommand::Crash),
            Some("core0"),
            &mut writer,
        );
        assert!(res.is_ok());
        assert_eq!(
            platform::core_monitor::LAST_PANICKED_CORE.with(|cell| cell.get()),
            Some(0)
        );
    }

    // 3. Test crash core1 subcommand (happy path)
    platform::core_monitor::LAST_PANICKED_CORE.with(|cell| cell.set(None));
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = platform::core_monitor::handle_core_monitor_cli(
            &resolver,
            Some(platform::core_monitor::CoreMonitorSubcommand::Crash),
            Some("core1"),
            &mut writer,
        );
        assert!(res.is_ok());
        assert_eq!(
            platform::core_monitor::LAST_PANICKED_CORE.with(|cell| cell.get()),
            Some(1)
        );
    }

    // 4. Test sad paths
    // Sad Path A: Missing subcommand
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res =
            platform::core_monitor::handle_core_monitor_cli(&resolver, None, None, &mut writer);
        assert_eq!(
            res,
            Err("Missing core monitor subcommand (expected: status, crash)")
        );
    }

    // Sad Path B: Missing core target for crash
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = platform::core_monitor::handle_core_monitor_cli(
            &resolver,
            Some(platform::core_monitor::CoreMonitorSubcommand::Crash),
            None,
            &mut writer,
        );
        assert_eq!(
            res,
            Err("Missing target core for crash (expected: core0, core1)")
        );
    }

    // Sad Path C: Invalid core name
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = platform::core_monitor::handle_core_monitor_cli(
            &resolver,
            Some(platform::core_monitor::CoreMonitorSubcommand::Crash),
            Some("invalid_core"),
            &mut writer,
        );
        assert_eq!(res, Err("Invalid core name (must be core0 or core1)"));
    }
}
