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
        i2c: core::cell::RefCell::new(MockI2c::new(0x3c)),
    };

    let mut mock_write = MockWriter {
        buf: heapless::Vec::new(),
    };

    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = futures::executor::block_on(platform::i2c::handle_i2c_cli(
            &resolver,
            Some(platform::i2c::I2cSubcommand::Scan),
            false,
            None,
            None,
            None,
            None,
            &mut writer,
        ));
        assert!(res.is_ok());
    }

    let output_str = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(output_str.contains("Scanning I2C bus..."));
    assert!(output_str.contains(" 3c"));
    assert!(!output_str.contains(" 36"));
    assert!(!output_str.contains("78"));
}

#[test]
fn test_i2c_peek_and_poke_8_and_16_cli() {
    let resolver = MockI2cResolver {
        i2c: core::cell::RefCell::new(MockI2c::new(0x36)),
    };
    let mut mock_write = MockWriter::default();

    // 1. Happy path: 8-bit register offset poke8 + peek8 (hex and decimal tokens, plus --bus i2c0)
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        assert!(futures::executor::block_on(platform::i2c::handle_i2c_cli(
            &resolver,
            Some(platform::i2c::I2cSubcommand::Poke8),
            false,
            Some("i2c0"),
            Some("0x36"),
            Some("0x0c"),
            Some("0x97"),
            &mut writer,
        ))
        .is_ok());
    }
    assert_eq!(
        resolver.i2c.borrow().last_tx_bytes.as_slice(),
        &[0x0c, 0x97]
    );
    let out = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(out.contains("Wrote 0x97 (151) to I2C[0x36] reg 0x0c"));

    mock_write.buf.clear();
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        assert!(futures::executor::block_on(platform::i2c::handle_i2c_cli(
            &resolver,
            Some(platform::i2c::I2cSubcommand::Peek8),
            false,
            None,
            Some("54"),
            Some("12"),
            None,
            &mut writer,
        ))
        .is_ok());
    }
    assert_eq!(resolver.i2c.borrow().last_tx_bytes.as_slice(), &[0x0c]);
    let out = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(out.contains("I2C[0x36] reg 0x0c = 0x97 (151)"));

    // 2. Happy path: 8-bit register offset poke16 + peek16 (big-endian register word)
    mock_write.buf.clear();
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        assert!(futures::executor::block_on(platform::i2c::handle_i2c_cli(
            &resolver,
            Some(platform::i2c::I2cSubcommand::Poke16),
            false,
            None,
            Some("0x36"),
            Some("0x0c"),
            Some("0x971c"),
            &mut writer,
        ))
        .is_ok());
    }
    assert_eq!(
        resolver.i2c.borrow().last_tx_bytes.as_slice(),
        &[0x0c, 0x97, 0x1c]
    );
    let out = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(out.contains("Wrote 0x971c (38684) to I2C[0x36] reg 0x0c"));

    mock_write.buf.clear();
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        assert!(futures::executor::block_on(platform::i2c::handle_i2c_cli(
            &resolver,
            Some(platform::i2c::I2cSubcommand::Peek16),
            false,
            None,
            Some("0x36"),
            Some("0x0c"),
            None,
            &mut writer,
        ))
        .is_ok());
    }
    let out = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(out.contains("I2C[0x36] reg 0x0c = 0x971c (38684, bytes: [0x97, 0x1c])"));

    // 3. Happy path: 16-bit register addressing via `--reg16` (for <= 0xFF like 0x000c) and auto-promotion (> 0xFF like 0x010f)
    resolver.i2c.borrow_mut().use_16bit_reg_addr = true;
    mock_write.buf.clear();
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        assert!(futures::executor::block_on(platform::i2c::handle_i2c_cli(
            &resolver,
            Some(platform::i2c::I2cSubcommand::Poke8),
            true,
            None,
            Some("0x36"),
            Some("0x0c"),
            Some("0x42"),
            &mut writer,
        ))
        .is_ok());
    }
    assert_eq!(
        resolver.i2c.borrow().last_tx_bytes.as_slice(),
        &[0x00, 0x0c, 0x42]
    );
    assert!(core::str::from_utf8(&mock_write.buf)
        .unwrap()
        .contains("Wrote 0x42 (66) to I2C[0x36] reg 0x000c"));

    mock_write.buf.clear();
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        assert!(futures::executor::block_on(platform::i2c::handle_i2c_cli(
            &resolver,
            Some(platform::i2c::I2cSubcommand::Peek8),
            true,
            None,
            Some("0x36"),
            Some("0x0c"),
            None,
            &mut writer,
        ))
        .is_ok());
    }
    assert_eq!(
        resolver.i2c.borrow().last_tx_bytes.as_slice(),
        &[0x00, 0x0c]
    );
    assert!(core::str::from_utf8(&mock_write.buf)
        .unwrap()
        .contains("I2C[0x36] reg 0x000c = 0x42 (66)"));

    mock_write.buf.clear();
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        assert!(futures::executor::block_on(platform::i2c::handle_i2c_cli(
            &resolver,
            Some(platform::i2c::I2cSubcommand::Poke16),
            false,
            None,
            Some("0x36"),
            Some("0x010f"),
            Some("0xeacc"),
            &mut writer,
        ))
        .is_ok());
    }
    assert_eq!(
        resolver.i2c.borrow().last_tx_bytes.as_slice(),
        &[0x01, 0x0f, 0xea, 0xcc]
    );

    mock_write.buf.clear();
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        assert!(futures::executor::block_on(platform::i2c::handle_i2c_cli(
            &resolver,
            Some(platform::i2c::I2cSubcommand::Peek16),
            false,
            None,
            Some("0x36"),
            Some("0x010f"),
            None,
            &mut writer,
        ))
        .is_ok());
    }
    assert_eq!(
        resolver.i2c.borrow().last_tx_bytes.as_slice(),
        &[0x01, 0x0f]
    );
    assert!(core::str::from_utf8(&mock_write.buf)
        .unwrap()
        .contains("I2C[0x36] reg 0x010f = 0xeacc (60108, bytes: [0xea, 0xcc])"));

    // 4. Sad paths: missing args, reserved 7-bit address, invalid hex, unknown --bus, and NACKed address
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        assert_eq!(
            futures::executor::block_on(platform::i2c::handle_i2c_cli(
                &resolver,
                Some(platform::i2c::I2cSubcommand::Peek16),
                false,
                None,
                Some("0x36"),
                None,
                None,
                &mut writer,
            )),
            Err("Usage: i2c peek16 <addr> <reg> [--reg16] [-b|--bus <name>]")
        );
        assert_eq!(
            futures::executor::block_on(platform::i2c::handle_i2c_cli(
                &resolver,
                Some(platform::i2c::I2cSubcommand::Poke8),
                false,
                None,
                Some("0x03"),
                Some("0x00"),
                Some("0x01"),
                &mut writer,
            )),
            Err("Invalid 7-bit I2C address (must be 0x08..=0x77)")
        );
        assert_eq!(
            futures::executor::block_on(platform::i2c::handle_i2c_cli(
                &resolver,
                Some(platform::i2c::I2cSubcommand::Poke16),
                false,
                None,
                Some("0x36"),
                Some("0x0c"),
                Some("0xZZZZ"),
                &mut writer,
            )),
            Err("Invalid 16-bit hex value (expected 0x0000..=0xFFFF)")
        );
        assert_eq!(
            futures::executor::block_on(platform::i2c::handle_i2c_cli(
                &resolver,
                Some(platform::i2c::I2cSubcommand::Peek8),
                false,
                Some("i2c99"),
                Some("0x36"),
                Some("0x00"),
                None,
                &mut writer,
            )),
            Err("Unknown I2C bus")
        );
        assert_eq!(
            futures::executor::block_on(platform::i2c::handle_i2c_cli(
                &resolver,
                Some(platform::i2c::I2cSubcommand::Peek8),
                false,
                None,
                Some("0x40"),
                Some("0x00"),
                None,
                &mut writer,
            )),
            Err("I2C peek8 transaction failed (NACK or bus timeout)")
        );
    }
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
        let res = futures::executor::block_on(platform::gpio::handle_gpio_cli(
            &mock_resolver,
            Some(platform::gpio::GpioSubcommand::Status),
            None,
            None,
            &mut writer,
        ));
        assert!(res.is_ok());
    }
    let output_str = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(output_str.contains("GPIO Pin Status") || output_str.contains("Mock GPIO"));

    // Test Gpio read
    mock_write.buf.clear();
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = futures::executor::block_on(platform::gpio::handle_gpio_cli(
            &mock_resolver,
            Some(platform::gpio::GpioSubcommand::Read),
            Some(5),
            None,
            &mut writer,
        ));
        assert!(res.is_ok());
    }
    let output_str = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(output_str.contains("GP5"));

    // Test Gpio read whole bank (no pin)
    mock_write.buf.clear();
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = futures::executor::block_on(platform::gpio::handle_gpio_cli(
            &mock_resolver,
            Some(platform::gpio::GpioSubcommand::Read),
            None,
            None,
            &mut writer,
        ));
        assert!(res.is_ok());
    }
    let output_str = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(output_str.contains("GP0") && output_str.contains("GP29"));

    // Test Gpio set high
    mock_write.buf.clear();
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = futures::executor::block_on(platform::gpio::handle_gpio_cli(
            &mock_resolver,
            Some(platform::gpio::GpioSubcommand::Set),
            Some(5),
            Some(1),
            &mut writer,
        ));
        assert!(res.is_ok());
    }
    let output_str = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(output_str.contains("Mock GP5 set to High (1)"));

    // Test Gpio set low
    mock_write.buf.clear();
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = futures::executor::block_on(platform::gpio::handle_gpio_cli(
            &mock_resolver,
            Some(platform::gpio::GpioSubcommand::Set),
            Some(5),
            Some(0),
            &mut writer,
        ));
        assert!(res.is_ok());
    }
    let output_str = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(output_str.contains("Mock GP5 set to Low (0)"));

    // Test Gpio set missing pin
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = futures::executor::block_on(platform::gpio::handle_gpio_cli(
            &mock_resolver,
            Some(platform::gpio::GpioSubcommand::Set),
            None,
            Some(1),
            &mut writer,
        ));
        assert_eq!(
            res,
            Err("Missing pin number (expected: gpio set <pin> <0|1>)")
        );
    }

    // Test Gpio set missing value
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = futures::executor::block_on(platform::gpio::handle_gpio_cli(
            &mock_resolver,
            Some(platform::gpio::GpioSubcommand::Set),
            Some(5),
            None,
            &mut writer,
        ));
        assert_eq!(
            res,
            Err("Missing pin value (expected: gpio set <pin> <0|1>)")
        );
    }

    // Test Gpio set invalid pin (out of range)
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = futures::executor::block_on(platform::gpio::handle_gpio_cli(
            &mock_resolver,
            Some(platform::gpio::GpioSubcommand::Set),
            Some(30),
            Some(1),
            &mut writer,
        ));
        assert_eq!(res, Err("Invalid pin number (must be 0..29)"));
    }

    // Test Gpio set on alternate function pin (GP12 - I2C0 SDA)
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = futures::executor::block_on(platform::gpio::handle_gpio_cli(
            &mock_resolver,
            Some(platform::gpio::GpioSubcommand::Set),
            Some(12),
            Some(1),
            &mut writer,
        ));
        assert_eq!(
            res,
            Err("Cannot set pin configured for alternate function (bus assigned)")
        );
    }
}

#[test]
fn test_is_pin_high() {
    assert!(!platform::gpio::is_pin_high(0, 0));
    assert!(platform::gpio::is_pin_high(1 << 5, 5));
    assert!(!platform::gpio::is_pin_high(1 << 5, 6));
    assert!(platform::gpio::is_pin_high(0xFFFF_FFFF, 29));
    assert!(!platform::gpio::is_pin_high(0, 29));
}

#[test]
fn test_is_bus_assigned() {
    assert_eq!(platform::gpio::BUS_ASSIGNED_PINS, &[0, 1, 11, 12, 13]);
    assert!(platform::gpio::is_bus_assigned(0)); // UART0 TX
    assert!(platform::gpio::is_bus_assigned(1)); // UART0 RX
    assert!(platform::gpio::is_bus_assigned(11)); // PIO0 WS2812
    assert!(platform::gpio::is_bus_assigned(12)); // I2C0 SDA
    assert!(platform::gpio::is_bus_assigned(13)); // I2C0 SCL
    assert!(!platform::gpio::is_bus_assigned(2));
    assert!(!platform::gpio::is_bus_assigned(5));
    assert!(!platform::gpio::is_bus_assigned(26));
    assert!(!platform::gpio::is_bus_assigned(30)); // Beyond range
}

#[test]
fn test_gpio_cli_skips_bus_assigned_pin() {
    let mock_resolver = ();
    let mut mock_write = MockWriter {
        buf: heapless::Vec::new(),
    };

    // Test Gpio read on an alternate function pin (GP12 - I2C0 SDA)
    {
        let mut writer = embedded_cli::writer::Writer::new(&mut mock_write);
        let res = futures::executor::block_on(platform::gpio::handle_gpio_cli(
            &mock_resolver,
            Some(platform::gpio::GpioSubcommand::Read),
            Some(12),
            None,
            &mut writer,
        ));
        assert!(res.is_ok());
    }
    let output_str = core::str::from_utf8(&mock_write.buf).unwrap();
    assert!(output_str.contains("GP12 is configured for alternate function"));
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
