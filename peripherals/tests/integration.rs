use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use peripherals::mock::DummyI2c;
use peripherals::{Motor, MotorSpeed, Tickable};

macro_rules! run_test {
    ($body:expr) => {{
        #[embassy_executor::task]
        async fn test_task(tx: std::sync::mpsc::Sender<()>) {
            $body.await;
            let _ = tx.send(());
        }

        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let executor = Box::leak(Box::new(embassy_executor::Executor::new()));
            executor.run(|spawner| {
                spawner.spawn(test_task(tx)).unwrap();
            });
        });
        rx.recv().unwrap();
    }};
}

struct MockPin<'a> {
    is_high: &'a std::sync::atomic::AtomicBool,
}

impl<'a> ErrorType for MockPin<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> OutputPin for MockPin<'a> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.is_high
            .store(false, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.is_high
            .store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }
}

impl<'a> InputPin for MockPin<'a> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_high.load(std::sync::atomic::Ordering::SeqCst))
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_high.load(std::sync::atomic::Ordering::SeqCst))
    }
}

#[test]
fn test_l9110s_functional() {
    run_test!(async {
        let pin_ia_state = std::sync::atomic::AtomicBool::new(false);
        let pin_ib_state = std::sync::atomic::AtomicBool::new(false);
        let pin_ia = MockPin {
            is_high: &pin_ia_state,
        };
        let pin_ib = MockPin {
            is_high: &pin_ib_state,
        };
        let mut motor = peripherals::l9110s::L9110s::new(pin_ia, pin_ib);

        // 1. Initially both low
        assert!(!pin_ia_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ib_state.load(std::sync::atomic::Ordering::SeqCst));

        // 2. Setting speed > 0 drives pin_ia high and pin_ib low
        assert!(motor.set_speed(MotorSpeed::new(100).unwrap()).is_ok());
        assert!(pin_ia_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ib_state.load(std::sync::atomic::Ordering::SeqCst));

        // 3. Setting speed == 0 brakes both pins to low
        assert!(motor.set_speed(MotorSpeed::ZERO).is_ok());
        assert!(!pin_ia_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ib_state.load(std::sync::atomic::Ordering::SeqCst));

        // 4. Stopping brakes both pins to low
        assert!(motor.set_speed(MotorSpeed::new(50).unwrap()).is_ok());
        assert!(pin_ia_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ib_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(motor.stop().is_ok());
        assert!(!pin_ia_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ib_state.load(std::sync::atomic::Ordering::SeqCst));
    });
}

#[test]
fn test_vl53l0x_threshold_validation() {
    run_test!(async {
        use model::calibration::Calibration;
        use peripherals::vl53l0x::Vl53l0x;

        let mut sensor = Vl53l0x::new(DummyI2c, 0x30, model::types::Direction::North);
        // Default threshold is 300, cal_near is 0.

        // 1. Setting threshold to > cal_near + THRESHOLD_ERROR_MM should succeed.
        assert!(sensor.set_threshold_mm(250).is_ok());

        // 2. Setting threshold to <= cal_near + THRESHOLD_ERROR_MM should return an error.
        let mut s = Vl53l0x::new(DummyI2c, 0x30, model::types::Direction::North);
        assert!(s.set_threshold_mm(10).is_err());

        // 3. Setting calibration with threshold_mm > near + THRESHOLD_ERROR_MM should succeed.
        let mut cal = model::calibration::Vl53l0xCalibration::default();
        cal.sensors[model::types::Direction::North as usize] =
            model::calibration::TwoPointCalibration::new(50, 150);
        sensor.set_calibration(&cal);
        assert_eq!(
            sensor.calibration(),
            Some(model::calibration::TwoPointCalibration::new(50, 150))
        );

        // 4. Setting calibration with threshold_mm <= near + THRESHOLD_ERROR_MM should be ignored.
        let mut s = Vl53l0x::new(DummyI2c, 0x30, model::types::Direction::North);
        let _ = s.set_threshold_mm(100);
        let mut cal2 = model::calibration::Vl53l0xCalibration::default();
        cal2.sensors[model::types::Direction::North as usize] =
            model::calibration::TwoPointCalibration::new(90, 150);
        s.set_calibration(&cal2);
        assert!(s.calibration().is_none());
    });
}

#[test]
fn test_motor_duty_cycling_ticks() {
    run_test!(async {
        let pin_ia_state = std::sync::atomic::AtomicBool::new(false);
        let pin_ib_state = std::sync::atomic::AtomicBool::new(false);
        let pin_ia = MockPin {
            is_high: &pin_ia_state,
        };
        let pin_ib = MockPin {
            is_high: &pin_ib_state,
        };
        let mut motor = peripherals::l9110s::L9110s::new(pin_ia, pin_ib);

        // Set speed to 30 (30% duty cycle)
        assert!(motor.set_speed(MotorSpeed::new(30).unwrap()).is_ok());
        // Initial state set_speed drives active immediately
        assert!(pin_ia_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ib_state.load(std::sync::atomic::Ordering::SeqCst));

        // Tick 1 to 2: active (total 3 active ticks: 0, 1, 2)
        for _ in 1..=2 {
            assert!(motor.tick().await.is_ok());
            assert!(pin_ia_state.load(std::sync::atomic::Ordering::SeqCst));
            assert!(!pin_ib_state.load(std::sync::atomic::Ordering::SeqCst));
        }

        // Tick 3: becomes inactive (tick_counter reaches 3 >= threshold 3)
        assert!(motor.tick().await.is_ok());
        assert!(!pin_ia_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ib_state.load(std::sync::atomic::Ordering::SeqCst));

        // Ticks up to 9: inactive
        for _ in 4..=9 {
            assert!(motor.tick().await.is_ok());
            assert!(!pin_ia_state.load(std::sync::atomic::Ordering::SeqCst));
            assert!(!pin_ib_state.load(std::sync::atomic::Ordering::SeqCst));
        }

        // Tick 10: resets counter, becomes active again
        assert!(motor.tick().await.is_ok());
        assert!(pin_ia_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ib_state.load(std::sync::atomic::Ordering::SeqCst));
    });
}

struct SpyI2c<'a> {
    writes: &'a std::sync::Mutex<Vec<(u8, Vec<u8>)>>,
}

impl<'a> embedded_hal_async::i2c::ErrorType for SpyI2c<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> embedded_hal_async::i2c::I2c for SpyI2c<'a> {
    async fn read(&mut self, _address: u8, _read: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        self.writes.lock().unwrap().push((address, write.to_vec()));
        Ok(())
    }
    async fn write_read(
        &mut self,
        _address: u8,
        _write: &[u8],
        _read: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn test_vl53l0x_init() {
    run_test!(async {
        use peripherals::vl53l0x::{InterruptMode, Vl53l0x};

        let writes = std::sync::Mutex::new(Vec::new());
        let i2c = SpyI2c { writes: &writes };
        let mut sensor = Vl53l0x::new(i2c, 0x29, model::types::Direction::North);

        // Call init to change address to 0x30, threshold to 250, and interrupt to LowLevel
        let res = sensor.init(0x30, 250, InterruptMode::LowLevel).await;
        assert!(res.is_ok());

        // Verify properties are updated
        assert_eq!(sensor.threshold_mm(), 250);

        // Verify written values
        let w = writes.lock().unwrap();
        assert_eq!(w.len(), 8);

        // 1. Address change (write to 0x29): register 0x8A -> 0x30
        assert_eq!(w[0], (0x29, vec![0x8A, 0x30]));

        // 2. Low threshold (write to 0x30): register 0x0E -> 250 (0x00, 0xFA)
        assert_eq!(w[1], (0x30, vec![0x0E, 0x00, 0xFA]));

        // 3. High threshold (write to 0x30): register 0x0C -> 250 + 50 = 300 (0x01, 0x2C)
        assert_eq!(w[2], (0x30, vec![0x0C, 0x01, 0x2C]));

        // 4. Interrupt mode config (write to 0x30): register 0x0A -> 0x01 (LowLevel)
        assert_eq!(w[3], (0x30, vec![0x0A, 0x01]));

        // 5. Interrupt clear (write to 0x30): register 0x0B -> 0x01
        assert_eq!(w[4], (0x30, vec![0x0B, 0x01]));

        // 6. Sequence steps configuration (write to 0x30): register 0x01 -> 0xFF
        assert_eq!(w[5], (0x30, vec![0x01, 0xFF]));

        // 7. Timing budget configuration (write to 0x30): register 0x71 -> 0x54, 0x36
        assert_eq!(w[6], (0x30, vec![0x71, 0x54, 0x36]));

        // 8. Signal rate limit check (write to 0x30): register 0x44 -> 0x00, 0x06
        assert_eq!(w[7], (0x30, vec![0x44, 0x00, 0x06]));
    });
}

struct FailingI2c {
    error_after_writes: usize,
    write_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DummyError {
    I2cFailure,
}

impl embedded_hal_async::i2c::Error for DummyError {
    fn kind(&self) -> embedded_hal_async::i2c::ErrorKind {
        embedded_hal_async::i2c::ErrorKind::Bus
    }
}

impl embedded_hal_async::i2c::ErrorType for FailingI2c {
    type Error = DummyError;
}

impl embedded_hal_async::i2c::I2c for FailingI2c {
    async fn read(&mut self, _address: u8, _read: &mut [u8]) -> Result<(), Self::Error> {
        Err(DummyError::I2cFailure)
    }
    async fn write(&mut self, _address: u8, _write: &[u8]) -> Result<(), Self::Error> {
        self.write_count += 1;
        if self.write_count > self.error_after_writes {
            Err(DummyError::I2cFailure)
        } else {
            Ok(())
        }
    }
    async fn write_read(
        &mut self,
        _address: u8,
        _write: &[u8],
        _read: &mut [u8],
    ) -> Result<(), Self::Error> {
        Err(DummyError::I2cFailure)
    }
    async fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Err(DummyError::I2cFailure)
    }
}

#[test]
fn test_vl53l0x_i2c_error_propagation() {
    run_test!(async {
        use peripherals::vl53l0x::{InterruptMode, Vl53l0x};

        // 1. Initial write fails
        let i2c = FailingI2c {
            error_after_writes: 0,
            write_count: 0,
        };
        let mut sensor = Vl53l0x::new(i2c, 0x29, model::types::Direction::North);
        let res = sensor.init(0x30, 250, InterruptMode::LowLevel).await;
        assert!(res.is_err());

        // 2. Middle write fails
        let i2c = FailingI2c {
            error_after_writes: 2,
            write_count: 0,
        };
        let mut sensor = Vl53l0x::new(i2c, 0x29, model::types::Direction::North);
        let res = sensor.init(0x30, 250, InterruptMode::LowLevel).await;
        assert!(res.is_err());
    });
}

struct Max17048MockI2c {
    crate_val: i16,
    status_val: u16,
}

impl embedded_hal_async::i2c::ErrorType for Max17048MockI2c {
    type Error = core::convert::Infallible;
}

impl embedded_hal_async::i2c::I2c for Max17048MockI2c {
    async fn read(&mut self, _address: u8, _read: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn write(&mut self, _address: u8, _write: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn write_read(
        &mut self,
        _address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        assert_eq!(write.len(), 1);
        let reg = write[0];
        if reg == 0x16 {
            let bytes = self.crate_val.to_be_bytes();
            read[0] = bytes[0];
            read[1] = bytes[1];
        } else if reg == 0x1A {
            let bytes = self.status_val.to_be_bytes();
            read[0] = bytes[0];
            read[1] = bytes[1];
        }
        Ok(())
    }
    async fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn test_max17048_charge_state() {
    run_test!(async {
        use model::interfaces::ChargeStatus;
        use model::types::ChargeState;
        use peripherals::max17048::Max17048;

        // 1. CRATE > 0, no faults -> Charging
        let mut i2c = Max17048MockI2c {
            crate_val: 10,
            status_val: 0,
        };
        let mut gauge = Max17048::new(&mut i2c);
        assert_eq!(
            gauge.get_charge_state().await.unwrap(),
            ChargeState::Charging
        );

        // 2. CRATE <= 0, no faults -> DoneOrStandbyOrUnplugged
        let mut i2c = Max17048MockI2c {
            crate_val: 0,
            status_val: 0,
        };
        let mut gauge = Max17048::new(&mut i2c);
        assert_eq!(
            gauge.get_charge_state().await.unwrap(),
            ChargeState::DoneOrStandbyOrUnplugged
        );

        let mut i2c = Max17048MockI2c {
            crate_val: -5,
            status_val: 0,
        };
        let mut gauge = Max17048::new(&mut i2c);
        assert_eq!(
            gauge.get_charge_state().await.unwrap(),
            ChargeState::DoneOrStandbyOrUnplugged
        );

        // 3. VH (Voltage High) bit active (bit 10 set: status & (1 << 10) != 0) -> RecoverableFault
        let mut i2c = Max17048MockI2c {
            crate_val: 15,
            status_val: 1 << 10,
        };
        let mut gauge = Max17048::new(&mut i2c);
        assert_eq!(
            gauge.get_charge_state().await.unwrap(),
            ChargeState::RecoverableFault
        );

        // 4. VL (Voltage Low) bit active (bit 11 set: status & (1 << 11) != 0) -> NonRecoverableFault
        let mut i2c = Max17048MockI2c {
            crate_val: 15,
            status_val: 1 << 11,
        };
        let mut gauge = Max17048::new(&mut i2c);
        assert_eq!(
            gauge.get_charge_state().await.unwrap(),
            ChargeState::NonRecoverableFault
        );
    });
}

struct ProbeableMockI2c {
    reads: std::collections::VecDeque<Vec<u8>>,
    writes: std::sync::Arc<std::sync::Mutex<Vec<Vec<u8>>>>,
}

impl embedded_hal_async::i2c::ErrorType for ProbeableMockI2c {
    type Error = core::convert::Infallible;
}

impl embedded_hal_async::i2c::I2c for ProbeableMockI2c {
    async fn read(&mut self, _address: u8, _read: &mut [u8]) -> Result<(), Self::Error> {
        if let Some(r) = self.reads.pop_front() {
            _read.copy_from_slice(&r);
        }
        Ok(())
    }
    async fn write(&mut self, _address: u8, _write: &[u8]) -> Result<(), Self::Error> {
        self.writes.lock().unwrap().push(_write.to_vec());
        Ok(())
    }
    async fn write_read(
        &mut self,
        _address: u8,
        _write: &[u8],
        _read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.writes.lock().unwrap().push(_write.to_vec());
        if let Some(r) = self.reads.pop_front() {
            _read.copy_from_slice(&r);
        }
        Ok(())
    }
    async fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn test_probeable_max17048() {
    run_test!(async {
        use model::interfaces::Probeable;
        use model::types::PeripheralError;
        use peripherals::max17048::Max17048;

        // 1. Success case
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x00, 0x12]);
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c {
            reads,
            writes: writes.clone(),
        };
        let mut dev = Max17048::new(&mut i2c);

        let id = dev.read_chip_id().await.unwrap();
        assert_eq!(id, 0x0012);
        assert_eq!(writes.lock().unwrap()[0], vec![0x18]);

        assert!(dev.reset().await.is_ok());
        assert_eq!(writes.lock().unwrap()[1], vec![0xFE, 0x54, 0x00]);

        // 2. Mismatch case
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x00, 0x22]);
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c {
            reads,
            writes: writes.clone(),
        };
        let mut dev = Max17048::new(&mut i2c);
        assert_eq!(
            dev.read_chip_id().await.unwrap_err(),
            PeripheralError::DeviceNotFound(0x0022)
        );
    });
}

#[test]
fn test_probeable_ina219() {
    run_test!(async {
        use model::interfaces::Probeable;
        use model::types::PeripheralError;
        use peripherals::ina219::Ina219;

        // 1. Success case
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x39, 0x9F]);
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c {
            reads,
            writes: writes.clone(),
        };
        let mut dev = Ina219::new(&mut i2c);

        let id = dev.read_chip_id().await.unwrap();
        assert_eq!(id, 0x399F);
        assert_eq!(writes.lock().unwrap()[0], vec![0x00]);

        assert!(dev.reset().await.is_ok());
        assert_eq!(writes.lock().unwrap()[1], vec![0x00, 0x80, 0x00]);

        // 2. Mismatch case
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x12, 0x34]);
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c {
            reads,
            writes: writes.clone(),
        };
        let mut dev = Ina219::new(&mut i2c);
        assert_eq!(
            dev.read_chip_id().await.unwrap_err(),
            PeripheralError::DeviceNotFound(0x1234)
        );
    });
}

#[test]
fn test_probeable_vl53l0x() {
    run_test!(async {
        use model::interfaces::Probeable;
        use model::types::PeripheralError;
        use peripherals::vl53l0x::Vl53l0x;

        // 1. Success case
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0xEE]);
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c {
            reads,
            writes: writes.clone(),
        };
        let mut dev = Vl53l0x::new(&mut i2c, 0x29, model::types::Direction::North);

        let id = dev.read_chip_id().await.unwrap();
        assert_eq!(id, 0xEE);
        assert_eq!(writes.lock().unwrap()[0], vec![0xC0]);

        assert!(dev.reset().await.is_ok());

        // 2. Mismatch case
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x99]);
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c {
            reads,
            writes: writes.clone(),
        };
        let mut dev = Vl53l0x::new(&mut i2c, 0x29, model::types::Direction::North);
        assert_eq!(
            dev.read_chip_id().await.unwrap_err(),
            PeripheralError::DeviceNotFound(0x99)
        );
    });
}

struct TestBootStatus {
    errors: Vec<model::types::PeripheralError>,
}

impl model::interfaces::BootStatus for TestBootStatus {
    fn record_error(&mut self, error: model::types::PeripheralError) {
        self.errors.push(error);
    }
}

#[test]
fn test_macro_init_vl53l0x() {
    run_test!(async {
        // 1. Happy case
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0xEE]); // chip ID
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c { reads, writes };

        let pin_state = std::sync::atomic::AtomicBool::new(false);
        let mut gpio_pins: [Option<MockPin>; 30] = Default::default();
        gpio_pins[2] = Some(MockPin {
            is_high: &pin_state,
        });

        let mut errors = TestBootStatus { errors: vec![] };
        peripherals::init_vl53l0x!(
            &mut i2c,
            &mut gpio_pins,
            "ToF Test",
            2,
            0x30,
            100,
            &mut errors
        );

        assert!(errors.errors.is_empty());
        assert!(pin_state.load(std::sync::atomic::Ordering::SeqCst));

        // 2. Sad case (Wrong chip ID)
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x44]); // wrong ID
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c { reads, writes };

        let pin_state2 = std::sync::atomic::AtomicBool::new(false);
        let mut gpio_pins2: [Option<MockPin>; 30] = Default::default();
        gpio_pins2[2] = Some(MockPin {
            is_high: &pin_state2,
        });

        let mut errors = TestBootStatus { errors: vec![] };
        peripherals::init_vl53l0x!(
            &mut i2c,
            &mut gpio_pins2,
            "ToF Test",
            2,
            0x30,
            100,
            &mut errors
        );

        assert_eq!(errors.errors.len(), 1);
        assert_eq!(
            errors.errors[0],
            model::types::PeripheralError::DeviceNotFound(0x44)
        );
    });
}

#[test]
fn test_macro_init_max17048() {
    run_test!(async {
        // 1. Happy case
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x00, 0x10]); // VRESET matching (val & 0x00F0) == 0x0010
        reads.push_back(vec![0x00, 0x00]); // STATUS register with RI clear (0x0000)
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c { reads, writes };

        let mut errors = TestBootStatus { errors: vec![] };
        peripherals::init_max17048!(&mut i2c, &mut errors);

        assert!(errors.errors.is_empty());

        // 2. Sad case (Wrong chip ID)
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x00, 0x55]); // wrong ID
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c { reads, writes };

        let mut errors = TestBootStatus { errors: vec![] };
        peripherals::init_max17048!(&mut i2c, &mut errors);

        assert_eq!(errors.errors.len(), 1);
        assert_eq!(
            errors.errors[0],
            model::types::PeripheralError::DeviceNotFound(0x55)
        );
    });
}

#[test]
fn test_macro_init_ina219() {
    run_test!(async {
        // 1. Happy case
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x39, 0x9F]); // expected INA219 chip ID
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c { reads, writes };

        let mut errors = TestBootStatus { errors: vec![] };
        peripherals::init_ina219!(&mut i2c, &mut errors);

        assert!(errors.errors.is_empty());

        // 2. Sad case (Wrong chip ID)
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x11, 0x11]); // wrong ID
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c { reads, writes };

        let mut errors = TestBootStatus { errors: vec![] };
        peripherals::init_ina219!(&mut i2c, &mut errors);

        assert_eq!(errors.errors.len(), 1);
        assert_eq!(
            errors.errors[0],
            model::types::PeripheralError::DeviceNotFound(0x1111)
        );
    });
}

#[test]
#[allow(unused_mut)]
fn test_macro_init_ws2812() {
    let mut errors = TestBootStatus { errors: vec![] };
    let _dev = peripherals::init_ws2812!((), (), &mut errors);
    assert!(errors.errors.is_empty());
}

struct Vl53l0xTestI2c {
    interrupt_status: u8,
    range_status: u8,
    distance: u16,
}

impl embedded_hal_async::i2c::ErrorType for Vl53l0xTestI2c {
    type Error = core::convert::Infallible;
}

impl embedded_hal_async::i2c::I2c for Vl53l0xTestI2c {
    async fn read(&mut self, _address: u8, _read: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn write(&mut self, _address: u8, _write: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn write_read(
        &mut self,
        _address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        if write[0] == 0x13 {
            read[0] = self.interrupt_status;
        } else if write[0] == 0x14 {
            read[0] = self.range_status;
        } else if write[0] == 0x1E {
            let bytes = self.distance.to_be_bytes();
            read[0] = bytes[0];
            read[1] = bytes[1];
        }
        Ok(())
    }
    async fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn test_vl53l0x_measurement_timeout() {
    run_test!(async {
        use model::interfaces::ProximitySensor;
        use peripherals::vl53l0x::Vl53l0x;

        let i2c = Vl53l0xTestI2c {
            interrupt_status: 0, // Never ready
            range_status: 1,
            distance: 100,
        };
        let mut sensor = Vl53l0x::new(i2c, 0x30, model::types::Direction::North);
        let res = sensor.read_distance_mm().await;
        assert!(matches!(
            res,
            Err(model::types::PeripheralError::DeviceNotAvailable)
        ));
    });
}

#[test]
fn test_vl53l0x_measurement_success() {
    run_test!(async {
        use model::interfaces::ProximitySensor;
        use peripherals::vl53l0x::Vl53l0x;

        let i2c = Vl53l0xTestI2c {
            interrupt_status: 4, // Ready
            range_status: 1,     // Valid
            distance: 120,
        };
        let mut sensor = Vl53l0x::new(i2c, 0x30, model::types::Direction::North);
        let res = sensor.read_distance_mm().await;
        assert_eq!(res.unwrap(), model::types::SensorReading::Proximity(120));
    });
}

#[test]
fn test_vl53l0x_measurement_invalid_range() {
    run_test!(async {
        use model::interfaces::ProximitySensor;
        use peripherals::vl53l0x::Vl53l0x;

        let i2c = Vl53l0xTestI2c {
            interrupt_status: 4, // Ready
            range_status: 0,     // Invalid (valid bit is 0)
            distance: 120,
        };
        let mut sensor = Vl53l0x::new(i2c, 0x30, model::types::Direction::North);
        let res = sensor.read_distance_mm().await;
        assert_eq!(res.unwrap(), model::types::SensorReading::Invalid);
    });
}

#[test]
fn test_vl53l0x_measurement_max_range() {
    run_test!(async {
        use model::interfaces::ProximitySensor;
        use peripherals::vl53l0x::Vl53l0x;

        let i2c = Vl53l0xTestI2c {
            interrupt_status: 4,        // Ready
            range_status: (4 << 3) | 1, // Valid, max range (4)
            distance: 120,
        };
        let mut sensor = Vl53l0x::new(i2c, 0x30, model::types::Direction::North);
        let res = sensor.read_distance_mm().await;
        assert_eq!(res.unwrap(), model::types::SensorReading::Invalid);
    });
}

#[test]
fn test_vl53l0x_measurement_tcc_fail_as_invalid() {
    run_test!(async {
        use model::interfaces::ProximitySensor;
        use peripherals::vl53l0x::Vl53l0x;

        let i2c = Vl53l0xTestI2c {
            interrupt_status: 4,        // Ready
            range_status: (8 << 3) | 1, // Valid, TCC fail (8)
            distance: 120,
        };
        let mut sensor = Vl53l0x::new(i2c, 0x30, model::types::Direction::North);
        let res = sensor.read_distance_mm().await;
        assert_eq!(res.unwrap(), model::types::SensorReading::Invalid);
    });
}

#[test]
fn test_vl53l0x_measurement_min_range() {
    run_test!(async {
        use model::interfaces::ProximitySensor;
        use peripherals::vl53l0x::Vl53l0x;

        let i2c = Vl53l0xTestI2c {
            interrupt_status: 4,        // Ready
            range_status: (3 << 3) | 1, // Valid, min range (3)
            distance: 120,
        };
        let mut sensor = Vl53l0x::new(i2c, 0x30, model::types::Direction::North);
        let res = sensor.read_distance_mm().await;
        assert_eq!(
            res.unwrap(),
            model::types::SensorReading::Proximity(Vl53l0x::<Vl53l0xTestI2c>::MIN_RANGE_MM)
        );
    });
}
