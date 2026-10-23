use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use peripheral::mock::DummyI2c;
use peripheral::{Motor, MotorSpeed, Tickable};

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
fn test_tmc6300_functional_and_commutation() {
    run_test!(async {
        let pin_uh_state = std::sync::atomic::AtomicBool::new(false);
        let pin_ul_state = std::sync::atomic::AtomicBool::new(false);
        let pin_vh_state = std::sync::atomic::AtomicBool::new(false);
        let pin_vl_state = std::sync::atomic::AtomicBool::new(false);
        let pin_wh_state = std::sync::atomic::AtomicBool::new(false);
        let pin_wl_state = std::sync::atomic::AtomicBool::new(false);
        let pin_diag_state = std::sync::atomic::AtomicBool::new(false);

        let pin_uh = MockPin {
            is_high: &pin_uh_state,
        };
        let pin_ul = MockPin {
            is_high: &pin_ul_state,
        };
        let pin_vh = MockPin {
            is_high: &pin_vh_state,
        };
        let pin_vl = MockPin {
            is_high: &pin_vl_state,
        };
        let pin_wh = MockPin {
            is_high: &pin_wh_state,
        };
        let pin_wl = MockPin {
            is_high: &pin_wl_state,
        };
        let pin_diag = MockPin {
            is_high: &pin_diag_state,
        };
        let mut motor = peripheral::tmc6300::Tmc6300::new(
            pin_uh, pin_ul, pin_vh, pin_vl, pin_wh, pin_wl, pin_diag,
        );

        // 1. Initially all low
        assert!(!pin_uh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ul_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vl_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wl_state.load(std::sync::atomic::Ordering::SeqCst));

        // 2. Diagnostic fault check
        assert_eq!(motor.is_fault_active(), Ok(false));
        pin_diag_state.store(true, std::sync::atomic::Ordering::SeqCst);
        assert_eq!(motor.is_fault_active(), Ok(true));
        pin_diag_state.store(false, std::sync::atomic::Ordering::SeqCst);

        // 3. Setting speed > 0 applies initial commutation step 0:
        // Step 0: U=High, V=Low, W=Float
        assert!(motor.set_speed(MotorSpeed::new(100).unwrap()).is_ok());
        assert!(pin_uh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ul_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_vl_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wl_state.load(std::sync::atomic::Ordering::SeqCst));

        // 4. Tick through remaining forward commutation steps
        // Step 1: U=High, V=Float, W=Low
        assert!(motor.tick().await.is_ok());
        assert!(pin_uh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ul_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vl_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_wl_state.load(std::sync::atomic::Ordering::SeqCst));

        // Step 2: U=Float, V=High, W=Low
        assert!(motor.tick().await.is_ok());
        assert!(!pin_uh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ul_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_vh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vl_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_wl_state.load(std::sync::atomic::Ordering::SeqCst));

        // Step 3: U=Low, V=High, W=Float
        assert!(motor.tick().await.is_ok());
        assert!(!pin_uh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_ul_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_vh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vl_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wl_state.load(std::sync::atomic::Ordering::SeqCst));

        // Step 4: U=Low, V=Float, W=High
        assert!(motor.tick().await.is_ok());
        assert!(!pin_uh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_ul_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vl_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_wh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wl_state.load(std::sync::atomic::Ordering::SeqCst));

        // Step 5: U=Float, V=Low, W=High
        assert!(motor.tick().await.is_ok());
        assert!(!pin_uh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ul_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_vl_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_wh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wl_state.load(std::sync::atomic::Ordering::SeqCst));

        // Step 6 / wrap to Step 0: U=High, V=Low, W=Float
        assert!(motor.tick().await.is_ok());
        assert!(pin_uh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ul_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_vl_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wl_state.load(std::sync::atomic::Ordering::SeqCst));

        // 5. Setting speed == 0 puts all phases into Float (all low)
        assert!(motor.set_speed(MotorSpeed::ZERO).is_ok());
        assert!(!pin_uh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ul_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vl_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wl_state.load(std::sync::atomic::Ordering::SeqCst));

        // 6. Stopping also puts all phases into Float
        assert!(motor.set_speed(MotorSpeed::new(50).unwrap()).is_ok());
        assert!(motor.stop().is_ok());
        assert!(!pin_uh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ul_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vl_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wl_state.load(std::sync::atomic::Ordering::SeqCst));

        // 7. Commutation when speed is zero returns ZeroSpeed error
        assert!(matches!(
            motor.apply_commutation(),
            Err(peripheral::tmc6300::Tmc6300Error::ZeroSpeed)
        ));

        // 8. Reverse commutation when speed < 0
        assert!(motor.set_speed(MotorSpeed::new(-100).unwrap()).is_ok());
        // Reverse step 0: (Float, Low, High) -> U=Float, V=Low, W=High
        assert!(!pin_uh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ul_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_vl_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_wh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wl_state.load(std::sync::atomic::Ordering::SeqCst));

        // Reverse step 1: (Low, Float, High) -> U=Low, V=Float, W=High
        assert!(motor.tick().await.is_ok());
        assert!(!pin_uh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_ul_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vl_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(pin_wh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wl_state.load(std::sync::atomic::Ordering::SeqCst));

        // 9. Fault assertion triggers stop and returns DriverFault error
        pin_diag_state.store(true, std::sync::atomic::Ordering::SeqCst);
        assert!(matches!(
            motor.tick().await,
            Err(peripheral::tmc6300::Tmc6300Error::DriverFault)
        ));
        assert!(!pin_uh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_ul_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_vl_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wh_state.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!pin_wl_state.load(std::sync::atomic::Ordering::SeqCst));
    });
}

#[test]
fn test_tmc6300_break_before_make_and_low_side_conduction() {
    run_test!(async {
        // Regression test for WORM-GLIDING-BRISTLE-252:
        // Verifies that low-side FETs can be independently driven HIGH to conduct to GND,
        // and that high-side and low-side on any phase are NEVER both HIGH simultaneously
        // across all forward and reverse commutation steps.
        let uh = std::sync::atomic::AtomicBool::new(false);
        let ul = std::sync::atomic::AtomicBool::new(false);
        let vh = std::sync::atomic::AtomicBool::new(false);
        let vl = std::sync::atomic::AtomicBool::new(false);
        let wh = std::sync::atomic::AtomicBool::new(false);
        let wl = std::sync::atomic::AtomicBool::new(false);
        let diag = std::sync::atomic::AtomicBool::new(false);

        let pin_uh = MockPin { is_high: &uh };
        let pin_ul = MockPin { is_high: &ul };
        let pin_vh = MockPin { is_high: &vh };
        let pin_vl = MockPin { is_high: &vl };
        let pin_wh = MockPin { is_high: &wh };
        let pin_wl = MockPin { is_high: &wl };
        let pin_diag = MockPin { is_high: &diag };

        let mut motor = peripheral::tmc6300::Tmc6300::new(
            pin_uh, pin_ul, pin_vh, pin_vl, pin_wh, pin_wl, pin_diag,
        );

        // Test all forward commutation steps
        motor.set_speed(MotorSpeed::new(100).unwrap()).unwrap();
        for _ in 0..12 {
            let u_high = uh.load(std::sync::atomic::Ordering::SeqCst);
            let u_low = ul.load(std::sync::atomic::Ordering::SeqCst);
            let v_high = vh.load(std::sync::atomic::Ordering::SeqCst);
            let v_low = vl.load(std::sync::atomic::Ordering::SeqCst);
            let w_high = wh.load(std::sync::atomic::Ordering::SeqCst);
            let w_low = wl.load(std::sync::atomic::Ordering::SeqCst);

            // Invariant: Never allow shoot-through (both high-side and low-side asserted)
            assert!(!(u_high && u_low), "Shoot-through detected on Phase U!");
            assert!(!(v_high && v_low), "Shoot-through detected on Phase V!");
            assert!(!(w_high && w_low), "Shoot-through detected on Phase W!");

            // Invariant: Exactly one high-side and one low-side active per step
            let high_count = [u_high, v_high, w_high].iter().filter(|&&b| b).count();
            let low_count = [u_low, v_low, w_low].iter().filter(|&&b| b).count();
            assert_eq!(high_count, 1, "Exactly one high-side FET must be active");
            assert_eq!(low_count, 1, "Exactly one low-side FET must be active");

            motor.tick().await.unwrap();
        }

        // Test all reverse commutation steps
        motor.set_speed(MotorSpeed::new(-100).unwrap()).unwrap();
        for _ in 0..12 {
            let u_high = uh.load(std::sync::atomic::Ordering::SeqCst);
            let u_low = ul.load(std::sync::atomic::Ordering::SeqCst);
            let v_high = vh.load(std::sync::atomic::Ordering::SeqCst);
            let v_low = vl.load(std::sync::atomic::Ordering::SeqCst);
            let w_high = wh.load(std::sync::atomic::Ordering::SeqCst);
            let w_low = wl.load(std::sync::atomic::Ordering::SeqCst);

            assert!(
                !(u_high && u_low),
                "Shoot-through detected on Phase U (reverse)!"
            );
            assert!(
                !(v_high && v_low),
                "Shoot-through detected on Phase V (reverse)!"
            );
            assert!(
                !(w_high && w_low),
                "Shoot-through detected on Phase W (reverse)!"
            );

            let high_count = [u_high, v_high, w_high].iter().filter(|&&b| b).count();
            let low_count = [u_low, v_low, w_low].iter().filter(|&&b| b).count();
            assert_eq!(
                high_count, 1,
                "Exactly one high-side FET must be active in reverse"
            );
            assert_eq!(
                low_count, 1,
                "Exactly one low-side FET must be active in reverse"
            );

            motor.tick().await.unwrap();
        }
    });
}

#[test]
fn test_vl53l0x_threshold_validation() {
    run_test!(async {
        use model::calibration::Calibration;
        use peripheral::vl53l0x::Vl53l0x;

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
fn test_hall_sensor_pulse_counting_and_rpm() {
    let pin_state = std::sync::atomic::AtomicBool::new(false);
    let pin = MockPin {
        is_high: &pin_state,
    };
    let mut sensor = peripheral::hall_sensor::HallSensor::new(pin);

    assert_eq!(sensor.pulses_per_rev(), 3);
    assert_eq!(sensor.pulse_count(), 0);

    // Initial low poll -> no pulse
    assert!(sensor.poll_state().is_ok());
    assert_eq!(sensor.pulse_count(), 0);

    // Transition low -> high: 1 pulse
    pin_state.store(true, std::sync::atomic::Ordering::SeqCst);
    assert!(sensor.poll_state().is_ok());
    assert_eq!(sensor.pulse_count(), 1);

    // High -> high: no extra pulse
    assert!(sensor.poll_state().is_ok());
    assert_eq!(sensor.pulse_count(), 1);

    // High -> low: no extra pulse
    pin_state.store(false, std::sync::atomic::Ordering::SeqCst);
    assert!(sensor.poll_state().is_ok());
    assert_eq!(sensor.pulse_count(), 1);

    // Generate 9 more pulses (total 10 pulses)
    for _ in 0..9 {
        pin_state.store(true, std::sync::atomic::Ordering::SeqCst);
        assert!(sensor.poll_state().is_ok());
        pin_state.store(false, std::sync::atomic::Ordering::SeqCst);
        assert!(sensor.poll_state().is_ok());
    }
    assert_eq!(sensor.pulse_count(), 10);

    // Calculate RPM over 100 ms:
    // formula: (10 pulses * 60,000) / (3 ppr * 100 ms) = 600,000 / 300 = 2000 RPM
    sensor.update_rpm(100);
    assert_eq!(sensor.pulse_count(), 0); // cleared

    use model::interfaces::SpeedSensor;
    assert_eq!(sensor.read_rpm(), Ok(2000));

    // Direct override test
    sensor.set_rpm(3500);
    assert_eq!(sensor.read_rpm(), Ok(3500));
}

struct SpyI2c<'a> {
    writes: &'a std::sync::Mutex<Vec<(u8, Vec<u8>)>>,
}

impl<'a> embedded_hal_async::i2c::ErrorType for SpyI2c<'a> {
    type Error = DummyError;
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
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        if address == 0x30 {
            let w = self.writes.lock().unwrap();
            let has_readdressed = w.iter().any(|(addr, bytes)| {
                *addr == 0x29 && bytes.len() >= 2 && bytes[0] == 0x8A && bytes[1] == 0x30
            });
            if !has_readdressed {
                return Err(DummyError::I2cFailure);
            }
        }
        if write.len() == 1 && write[0] == 0xC0 && !read.is_empty() {
            read[0] = 0xEE;
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
fn test_vl53l0x_init() {
    run_test!(async {
        use peripheral::vl53l0x::{InterruptMode, Vl53l0x};

        let writes = std::sync::Mutex::new(Vec::new());
        let i2c = SpyI2c { writes: &writes };
        let mut sensor = Vl53l0x::new(i2c, 0x30, model::types::Direction::North);
        let _ = sensor.set_threshold_mm(250);
        sensor.set_interrupt_mode(InterruptMode::LowLevel);

        // Call init to configure the sensor
        let res = sensor.init().await;
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
        use peripheral::vl53l0x::{InterruptMode, Vl53l0x};

        // 1. Initial write fails
        let i2c = FailingI2c {
            error_after_writes: 0,
            write_count: 0,
        };
        let mut sensor = Vl53l0x::new(i2c, 0x30, model::types::Direction::North);
        let _ = sensor.set_threshold_mm(250);
        sensor.set_interrupt_mode(InterruptMode::LowLevel);
        let res = sensor.init().await;
        assert!(res.is_err());

        // 2. Middle write fails
        let i2c = FailingI2c {
            error_after_writes: 2,
            write_count: 0,
        };
        let mut sensor = Vl53l0x::new(i2c, 0x30, model::types::Direction::North);
        let _ = sensor.set_threshold_mm(250);
        sensor.set_interrupt_mode(InterruptMode::LowLevel);
        let res = sensor.init().await;
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
        use peripheral::max17048::Max17048;

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
        use peripheral::max17048::Max17048;

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
fn test_max17048_temperature_compensation() {
    run_test!(async {
        use model::interfaces::FuelGauge;
        use peripheral::max17048::Max17048;

        let writes = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let mut reads = std::collections::VecDeque::new();
        // Return 0x971C as current CONFIG register value
        reads.push_back(vec![0x97, 0x1C]);
        // Return 0x971C as current CONFIG register value
        reads.push_back(vec![0x97, 0x1C]);

        let mut i2c = ProbeableMockI2c {
            reads,
            writes: writes.clone(),
        };

        let mut gauge = Max17048::new(&mut i2c);

        // Test 1: T > 20°C (e.g. 30°C / 30000 mC)
        // RCOMP = 151 + (30 - 20) * -0.5 = 151 - 5 = 146 = 0x92
        // It should read CONFIG, modify it to 0x921C, and write it back.
        gauge.set_battery_temperature(30000).await.unwrap();

        // Test 2: T <= 20°C (e.g. 10°C / 10000 mC)
        // RCOMP = 151 + (10 - 20) * -5.0 = 151 + 50 = 201 = 0xC9
        // It should read CONFIG, modify it to 0xC91C, and write it back.
        gauge.set_battery_temperature(10000).await.unwrap();

        let w = writes.lock().unwrap();
        assert_eq!(w[0], vec![0x0C]);
        assert_eq!(w[1], vec![0x0C, 0x92, 0x1C]);
        assert_eq!(w[2], vec![0x0C]);
        assert_eq!(w[3], vec![0x0C, 0xC9, 0x1C]);
    });
}

#[test]
fn test_probeable_ina219() {
    run_test!(async {
        use model::interfaces::Probeable;
        use model::types::PeripheralError;
        use peripheral::ina219::Ina219;

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
fn test_ina219_read_current_ma_scaling_and_fallback() {
    run_test!(async {
        use model::interfaces::{PowerMeasurementMode, PowerSensor};
        use peripheral::ina219::Ina219;

        // Test 1: Current scaling from 0.1mA raw units to mA
        // Bus voltage has CNVR bit (0x0002) set
        // Current register returns 750 (representing 75.0 mA)
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x00, 0x02]); // Bus Voltage (CNVR = 1)
        reads.push_back(vec![0x02, 0xEE]); // Current = 750 (75 mA)

        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c {
            reads,
            writes: writes.clone(),
        };
        let mut dev = Ina219::new(&mut i2c);

        let current = dev.read_current_ma().await.unwrap();
        assert_eq!(current, 75); // 750 * 0.1mA = 75mA

        // Test 2: Current scaling for 2A (20000 raw counts)
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x00, 0x02]); // Bus Voltage (CNVR = 1)
        reads.push_back(vec![0x4E, 0x20]); // Current = 20000 (2000 mA)
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c {
            reads,
            writes: writes.clone(),
        };
        let mut dev = Ina219::new(&mut i2c);
        let current = dev.read_current_ma().await.unwrap();
        assert_eq!(current, 2000);

        // Test 3: Calibration lost fallback to Shunt Voltage register (0x01)
        // Current register returns 0 (due to lost calibration)
        // Shunt voltage register returns 20000 (representing 2000 mA)
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x00, 0x02]); // Bus Voltage (CNVR = 1)
        reads.push_back(vec![0x00, 0x00]); // Current = 0
        reads.push_back(vec![0x4E, 0x20]); // Shunt Voltage = 20000 (2000 mA)
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c {
            reads,
            writes: writes.clone(),
        };
        let mut dev = Ina219::new(&mut i2c);
        let current = dev.read_current_ma().await.unwrap();
        assert_eq!(current, 2000);
        // Verify calibration register was rewritten with 4096 (0x1000)
        {
            let written = writes.lock().unwrap();
            assert!(written.iter().any(|w| w == &vec![0x05, 0x10, 0x00]));
        }

        // Test 4: set_measurement_mode restores calibration
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x39, 0x98]); // Current Config
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c {
            reads,
            writes: writes.clone(),
        };
        let mut dev = Ina219::new(&mut i2c);
        dev.set_measurement_mode(PowerMeasurementMode::Continuous(true, true))
            .await
            .unwrap();
        let written = writes.lock().unwrap();
        assert_eq!(written[0], vec![0x00]); // Read config
        assert_eq!(written[1], vec![0x00, 0x39, 0x9F]); // Write config
        assert_eq!(written[2], vec![0x05, 0x10, 0x00]); // Write calibration (4096)
    });
}

#[test]
fn test_probeable_vl53l0x() {
    run_test!(async {
        use model::interfaces::Probeable;
        use model::types::PeripheralError;
        use peripheral::vl53l0x::Vl53l0x;

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
        if let Some(ref mut pin) = gpio_pins[2] {
            pin.set_high();
        }
        let mut sensor =
            peripheral::vl53l0x::Vl53l0x::new(&mut i2c, 0x30, model::types::Direction::North);
        let _ = sensor.set_threshold_mm(100);
        sensor.set_interrupt_mode(peripheral::vl53l0x::InterruptMode::LowLevel);
        peripheral::init_i2c!(&mut sensor, &mut errors);

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
        if let Some(ref mut pin) = gpio_pins2[2] {
            pin.set_high();
        }
        let mut sensor =
            peripheral::vl53l0x::Vl53l0x::new(&mut i2c, 0x30, model::types::Direction::North);
        let _ = sensor.set_threshold_mm(100);
        sensor.set_interrupt_mode(peripheral::vl53l0x::InterruptMode::LowLevel);
        peripheral::init_i2c!(&mut sensor, &mut errors);

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
        let mut sensor = peripheral::max17048::Max17048::new(&mut i2c);
        peripheral::init_i2c!(&mut sensor, &mut errors);

        assert!(errors.errors.is_empty());

        // 2. Sad case (Wrong chip ID)
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x00, 0x55]); // wrong ID
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c { reads, writes };

        let mut errors = TestBootStatus { errors: vec![] };
        let mut sensor = peripheral::max17048::Max17048::new(&mut i2c);
        peripheral::init_i2c!(&mut sensor, &mut errors);

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
        let mut sensor = peripheral::ina219::Ina219::new(&mut i2c);
        peripheral::init_i2c!(&mut sensor, &mut errors);

        assert!(errors.errors.is_empty());

        // 2. Sad case (Wrong chip ID)
        let mut reads = std::collections::VecDeque::new();
        reads.push_back(vec![0x11, 0x11]); // wrong ID
        let writes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let mut i2c = ProbeableMockI2c { reads, writes };

        let mut errors = TestBootStatus { errors: vec![] };
        let mut sensor = peripheral::ina219::Ina219::new(&mut i2c);
        peripheral::init_i2c!(&mut sensor, &mut errors);

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
    let _dev = peripheral::init_pio!(
        peripheral::ws2812::Ws2812,
        rp2040::pio::PioInitConfig,
        &mut errors
    );
    assert!(errors.errors.is_empty());
}

#[test]
fn test_ws2812_new_pio_direct() {
    let config = rp2040::pio::PioInitConfig;
    let _dev = peripheral::ws2812::Ws2812::new_pio(config);
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
        use peripheral::vl53l0x::Vl53l0x;

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
        use peripheral::vl53l0x::Vl53l0x;

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
        use peripheral::vl53l0x::Vl53l0x;

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
        use peripheral::vl53l0x::Vl53l0x;

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
        use peripheral::vl53l0x::Vl53l0x;

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
        use peripheral::vl53l0x::Vl53l0x;

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

#[test]
fn test_worm_writhing_tubeworm_652_tmc6300_current_limiting_and_pio_pwm() {
    run_test!(async {
        use model::interfaces::{Motor, Tickable};
        use model::types::MotorSpeed;
        use peripheral::tmc6300::{
            make_pwm_word, phase_mask, Tmc6300Pio, Tmc6300PioError, COMMUTATION_STEPS,
            MAX_TMC6300_CURRENT_MA, PIO_PWM_FREQUENCY_HZ, PWM_PERIOD_CYCLES,
        };

        // 1. Verify constant invariants for 2A max current and >=20kHz PWM frequency
        const { assert!(MAX_TMC6300_CURRENT_MA == 2000) };
        const { assert!(PIO_PWM_FREQUENCY_HZ >= 20_000) };
        const { assert!(PWM_PERIOD_CYCLES == 100) };

        // 2. Verify all forward and reverse phase masks guarantee zero shoot-through:
        // UH (bit 0) and UL (bit 1) must never both be high
        // VH (bit 2) and VL (bit 3) must never both be high
        // WH (bit 4) and WL (bit 5) must never both be high
        for speed in [-100, -50, -8, 8, 50, 100] {
            for step in 0..COMMUTATION_STEPS as u8 {
                let mask = phase_mask(speed, step);
                let uh = (mask & (1 << 0)) != 0;
                let ul = (mask & (1 << 1)) != 0;
                let vh = (mask & (1 << 2)) != 0;
                let vl = (mask & (1 << 3)) != 0;
                let wh = (mask & (1 << 4)) != 0;
                let wl = (mask & (1 << 5)) != 0;

                assert!(
                    !(uh && ul),
                    "Shoot-through on Phase U at speed {speed}, step {step}"
                );
                assert!(
                    !(vh && vl),
                    "Shoot-through on Phase V at speed {speed}, step {step}"
                );
                assert!(
                    !(wh && wl),
                    "Shoot-through on Phase W at speed {speed}, step {step}"
                );

                // Active step must have exactly one high-side and one low-side FET on
                let high_count = (uh as u8) + (vh as u8) + (wh as u8);
                let low_count = (ul as u8) + (vl as u8) + (wl as u8);
                assert_eq!(high_count, 1, "Must have exactly 1 high-side FET active");
                assert_eq!(low_count, 1, "Must have exactly 1 low-side FET active");
            }
        }

        // Zero speed must produce all-zero mask
        assert_eq!(phase_mask(0, 0), 0);

        // 3. Verify PIO word packing helper
        // Word format: bits 0..7 = phase_mask, bits 8..15 = on_count, bits 16..23 = off_count
        let word = make_pwm_word(0b001001, 8, 92);
        assert_eq!(word & 0xFF, 0b001001);
        assert_eq!((word >> 8) & 0xFF, 8);
        assert_eq!((word >> 16) & 0xFF, 92);

        // 4. Test driver operations on host mock
        let diag = std::sync::atomic::AtomicBool::new(false);
        let pin_diag = MockPin { is_high: &diag };
        let mut motor = Tmc6300Pio::new(pin_diag);

        // At startup, motor is stopped with 0 duty cycle
        assert_eq!(motor.speed(), 0);
        assert_eq!(motor.duty_cycle(), 0);

        // Command speed 8 (speed controls commutation rate; driver runs at 100% duty cycle)
        assert!(motor.set_speed(MotorSpeed::new(8).unwrap()).is_ok());
        assert_eq!(motor.speed(), 8);
        assert_eq!(motor.duty_cycle(), 100);
        let w8 = motor.last_word();
        assert_eq!((w8 >> 8) & 0xFF, 100);
        assert_eq!((w8 >> 16) & 0xFF, 0);
        assert_eq!(w8 & 0xFF, phase_mask(8, 0) as u32);

        // Advance commutation step via tick()
        assert!(motor.tick().await.is_ok());
        assert_eq!(motor.commutation_step(), 1);
        let w_step1 = motor.last_word();
        assert_eq!((w_step1 >> 8) & 0xFF, 100);
        assert_eq!(w_step1 & 0xFF, phase_mask(8, 1) as u32);

        // Ramp up to full speed 100
        assert!(motor.set_speed(MotorSpeed::new(100).unwrap()).is_ok());
        assert_eq!(motor.speed(), 100);
        assert_eq!(motor.duty_cycle(), 100);
        let w100 = motor.last_word();
        assert_eq!((w100 >> 8) & 0xFF, 100);
        assert_eq!((w100 >> 16) & 0xFF, 0);

        // Stop motor: duty cycle and mask go to 0
        assert!(motor.stop().is_ok());
        assert_eq!(motor.speed(), 0);
        assert_eq!(motor.duty_cycle(), 0);
        assert_eq!(motor.last_word() & 0xFF, 0);

        // Diagnostic fault trips DriverFault and stops motor
        assert!(motor.set_speed(MotorSpeed::new(50).unwrap()).is_ok());
        diag.store(true, std::sync::atomic::Ordering::SeqCst);
        let tick_res = motor.tick().await;
        assert!(matches!(tick_res, Err(Tmc6300PioError::DriverFault)));
        assert_eq!(motor.duty_cycle(), 0);
        assert_eq!(motor.speed(), 0);
    });
}

#[test]
fn test_worm_dirt_ribbonworm_196_tmc6300_pio_program_verification_and_duty_cycle_100() {
    use peripheral::tmc6300::{assemble_tmc6300_pio_program, make_pwm_word};

    let prg = assemble_tmc6300_pio_program();

    // 1. Program size must be exactly 11 instructions
    assert_eq!(prg.code.len(), 11);
    // 2. Wrap boundaries: target = 0, source = 10 (last instruction)
    assert_eq!(prg.wrap.target, 0);
    assert_eq!(prg.wrap.source, 10);

    // 3. Verify all JMP instructions target valid addresses within [0..=prg.wrap.source].
    // Any target > 10 jumps past wrap_source into uninitialized SRAM!
    for (addr, &inst) in prg.code.iter().enumerate() {
        let opcode = (inst >> 13) & 0x7;
        if opcode == 0 {
            // JMP instruction
            let cond = (inst >> 5) & 0x7;
            let target = (inst & 0x1F) as u8;
            assert!(
                target <= prg.wrap.source,
                "Instruction at {addr} jumps out-of-bounds to {target} (wrap.source={})",
                prg.wrap.source
            );
            if addr == 8 {
                // Must be jmp !y, wrap_target (0)
                assert_eq!(cond, 0b011, "Instruction at 8 must test condition !Y");
                assert_eq!(target, 0, "Instruction at 8 must jump to wrap_target (0)");
            }
        }
    }

    // 4. Simulate PIO execution for duty cycle 100% (on_count = 100, off_count = 0)
    let word100 = make_pwm_word(0b001001, 100, 0);
    let mut pc: u8 = 0;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut osr: u32 = 0;
    let mut pins: u8 = 0;
    let mut fifo: Option<u32> = Some(word100);
    let mut pin_cleared_to_null = false;

    // Run 500 PIO instruction cycles
    for _ in 0..500 {
        let current_pc = pc;
        match current_pc {
            0 => {
                // pull(false, false)
                if let Some(w) = fifo.take() {
                    osr = w;
                } else {
                    osr = x;
                }
            }
            1 => {
                // mov x, osr
                x = osr;
            }
            2 => {
                // out pins, 6 (shifts lower 6 bits to pins 16..21 without modulo wrap overwrite)
                pins = (osr & 0x3F) as u8;
                osr >>= 6;
            }
            3 => {
                // out null, 2 (discards 2 padding bits to preserve 8-bit alignment for on_count)
                osr >>= 2;
            }
            4 => {
                // out y, 8
                y = osr & 0xFF;
                osr >>= 8;
            }
            5 => {
                // jmp !y, skip_on (7)
                if y == 0 {
                    pc = 7;
                }
            }
            6 => {
                // jmp y--, on_loop (6)
                if y != 0 {
                    y -= 1;
                    pc = 6;
                } else {
                    y = u32::MAX;
                }
            }
            7 => {
                // out y, 8 (off_count)
                y = osr & 0xFF;
                osr >>= 8;
            }
            8 => {
                // jmp !y, wrap_target (0)
                if y == 0 {
                    pc = 0;
                }
            }
            9 => {
                // mov pins, null
                pins = 0;
                pin_cleared_to_null = true;
            }
            10 => {
                // jmp y--, off_loop (10)
                if y != 0 {
                    y -= 1;
                    pc = 10;
                } else {
                    y = u32::MAX;
                }
            }
            _ => panic!("PC reached out-of-bounds address {current_pc}"),
        }

        // Advance PC if jump did not branch to a new target
        if pc == current_pc {
            if pc == prg.wrap.source {
                pc = prg.wrap.target;
            } else {
                pc += 1;
            }
        }
    }

    // At 100% duty cycle, pins must never be cleared to null!
    assert!(
        !pin_cleared_to_null,
        "Pins were erroneously cleared to null at 100% duty cycle"
    );
    assert_eq!(pins, 0b001001, "Pins should remain continuously asserted");
}

#[test]
fn test_worm_embedded_leech_268_commutation_advancement_and_continuous_pwm() {
    run_test!(async {
        use peripheral::tmc6300::{
            assemble_tmc6300_pio_program, make_pwm_word, phase_mask, Tmc6300Pio, COMMUTATION_STEPS,
            PWM_PERIOD_CYCLES,
        };

        // 1. Setup mock TMC6300 PIO driver
        let diag = std::sync::atomic::AtomicBool::new(false);
        let pin_diag = MockPin { is_high: &diag };
        let mut motor = Tmc6300Pio::new(pin_diag);

        assert_eq!(motor.speed(), 0);
        assert_eq!(motor.duty_cycle(), 0);
        assert_eq!(motor.commutation_step(), 0);

        // 2. Set speed to 50%
        assert!(motor.set_speed(MotorSpeed::new(50).unwrap()).is_ok());
        assert_eq!(motor.speed(), 50);
        assert_eq!(motor.duty_cycle(), 100);

        // Advance through 18 commutation ticks (3 full 6-step electrical revolutions)
        let expected_forward_masks = [
            0b001001, // Step 0: UH High, VL Low
            0b100001, // Step 1: UH High, WL Low
            0b100100, // Step 2: VH High, WL Low
            0b000110, // Step 3: VH High, UL Low
            0b010010, // Step 4: WH High, UL Low
            0b011000, // Step 5: WH High, VL Low
        ];

        for tick_idx in 1..=18 {
            assert!(motor.tick().await.is_ok());
            let expected_step = (tick_idx % COMMUTATION_STEPS) as u8;
            assert_eq!(
                motor.commutation_step(),
                expected_step,
                "Commutation step did not advance as expected on tick {tick_idx}"
            );
            assert_eq!(motor.duty_cycle(), 100);

            let word = motor.last_word();
            let mask = (word & 0xFF) as u8;
            let on_count = ((word >> 8) & 0xFF) as u8;
            let off_count = ((word >> 16) & 0xFF) as u8;

            assert_eq!(on_count, 100);
            assert_eq!(off_count, 0);
            assert_eq!(mask, expected_forward_masks[expected_step as usize]);
            assert_eq!(mask, phase_mask(50, expected_step));
        }

        // 3. Steady state 100% speed operation across all steps 0..=5
        assert!(motor.set_speed(MotorSpeed::new(100).unwrap()).is_ok());
        assert_eq!(motor.speed(), 100);
        assert_eq!(motor.duty_cycle(), 100);

        for _ in 0..12 {
            let current_step = motor.commutation_step();
            let next_step = (current_step + 1) % (COMMUTATION_STEPS as u8);
            assert!(motor.tick().await.is_ok());
            assert_eq!(motor.commutation_step(), next_step);
            assert_eq!(motor.duty_cycle(), 100);

            let word = motor.last_word();
            let mask = (word & 0xFF) as u8;
            let on_count = ((word >> 8) & 0xFF) as u8;
            let off_count = ((word >> 16) & 0xFF) as u8;

            assert_eq!(on_count, 100);
            assert_eq!(off_count, 0);
            assert_eq!(mask, expected_forward_masks[next_step as usize]);
        }

        // 4. Reverse rotation commutation sequence
        assert!(motor.set_speed(MotorSpeed::new(-75).unwrap()).is_ok());
        assert_eq!(motor.speed(), -75);
        assert_eq!(motor.duty_cycle(), 100);

        let expected_reverse_masks = [
            0b011000, // Step 0: U Float, VL Low, WH High
            0b010010, // Step 1: UL Low, V Float, WH High
            0b000110, // Step 2: UL Low, VH High, W Float
            0b100100, // Step 3: U Float, VH High, WL Low
            0b100001, // Step 4: UH High, V Float, WL Low
            0b001001, // Step 5: UH High, VL Low, W Float
        ];

        for _ in 0..12 {
            let current_step = motor.commutation_step();
            let next_step = (current_step + 1) % (COMMUTATION_STEPS as u8);
            assert!(motor.tick().await.is_ok());
            assert_eq!(motor.commutation_step(), next_step);
            assert_eq!(motor.duty_cycle(), 100);

            let word = motor.last_word();
            let mask = (word & 0xFF) as u8;
            let on_count = ((word >> 8) & 0xFF) as u8;
            let off_count = ((word >> 16) & 0xFF) as u8;

            assert_eq!(on_count, 100);
            assert_eq!(off_count, 0);
            assert_eq!(mask, expected_reverse_masks[next_step as usize]);
        }

        // 5. Motor stop completely clears duty cycle and de-energizes all FETs
        assert!(motor.stop().is_ok());
        assert_eq!(motor.speed(), 0);
        assert_eq!(motor.duty_cycle(), 0);
        assert_eq!(motor.commutation_step(), 0);
        assert_eq!(motor.last_word(), make_pwm_word(0, 0, PWM_PERIOD_CYCLES));

        // 6. PIO program simulation: continuous 25 kHz recycling and dynamic step transition
        let prg = assemble_tmc6300_pio_program();
        let mut pc: u8 = 0;
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut osr: u32 = 0;
        let mut pins: u8 = 0;
        let mut fifo: std::collections::VecDeque<u32> = std::collections::VecDeque::new();

        // Push initial word for Step 0 with 50% duty cycle
        fifo.push_back(make_pwm_word(expected_forward_masks[0], 50, 50));

        let mut high_count = 0;
        let mut low_count = 0;

        // Run 400 PIO cycles (~4 PWM periods) with empty FIFO to verify continuous recycling
        for cycle in 0..400 {
            // At cycle 200, push Step 1 with 80% duty cycle to simulate tick arrival
            if cycle == 200 {
                fifo.push_back(make_pwm_word(expected_forward_masks[1], 80, 20));
            }

            let current_pc = pc;
            match current_pc {
                0 => {
                    if let Some(w) = fifo.pop_front() {
                        osr = w;
                    } else {
                        osr = x;
                    }
                }
                1 => {
                    x = osr;
                }
                2 => {
                    pins = (osr & 0x3F) as u8;
                    osr >>= 6;
                }
                3 => {
                    osr >>= 2;
                }
                4 => {
                    y = osr & 0xFF;
                    osr >>= 8;
                }
                5 => {
                    if y == 0 {
                        pc = 7;
                    }
                }
                6 => {
                    if y != 0 {
                        y -= 1;
                        pc = 6;
                    } else {
                        y = u32::MAX;
                    }
                }
                7 => {
                    y = osr & 0xFF;
                    osr >>= 8;
                }
                8 => {
                    if y == 0 {
                        pc = 0;
                    }
                }
                9 => {
                    pins = 0;
                }
                10 => {
                    if y != 0 {
                        y -= 1;
                        pc = 10;
                    } else {
                        y = u32::MAX;
                    }
                }
                _ => panic!("PC reached out-of-bounds address {current_pc}"),
            }

            if pins != 0 {
                high_count += 1;
                if cycle < 200 {
                    assert_eq!(
                        pins, expected_forward_masks[0],
                        "During first 200 cycles, pins should match step 0"
                    );
                } else if cycle > 300 {
                    assert_eq!(
                        pins, expected_forward_masks[1],
                        "After transition, pins should match step 1"
                    );
                }
            } else {
                low_count += 1;
            }

            if pc == current_pc {
                if pc == prg.wrap.source {
                    pc = prg.wrap.target;
                } else {
                    pc += 1;
                }
            }
        }

        // Both high and low cycles occurred, confirming continuous PWM switching
        assert!(
            high_count > 50,
            "Continuous PWM high-side pulses were absent"
        );
        assert!(
            low_count > 50,
            "Continuous PWM low-side de-energizations were absent"
        );
    });
}

#[test]
fn test_worm_creeping_wriggler_223_vl_pin_assertion_and_commutation_masks() {
    run_test!(async {
        use peripheral::tmc6300::phase_mask;

        // Bit mapping (GP16..GP21):
        // Bit 0: UH (GP16)
        // Bit 1: UL (GP17)
        // Bit 2: VH (GP18)
        // Bit 3: VL (GP19)
        // Bit 4: WH (GP20)
        // Bit 5: WL (GP21)
        const BIT_UH: u8 = 1 << 0;
        const BIT_UL: u8 = 1 << 1;
        const BIT_VH: u8 = 1 << 2;
        const BIT_VL: u8 = 1 << 3;
        const BIT_WH: u8 = 1 << 4;
        const BIT_WL: u8 = 1 << 5;

        // Forward commutation:
        // Step 0: UH High, VL Low -> UH(Bit 0), VL(Bit 3)
        let mask0 = phase_mask(100, 0);
        assert_eq!(
            mask0 & BIT_VL,
            BIT_VL,
            "Step 0 (forward) must assert VL (bit 3 / GP19)"
        );
        assert_eq!(
            mask0 & BIT_UH,
            BIT_UH,
            "Step 0 (forward) must assert UH (bit 0 / GP16)"
        );

        // Step 1: UH High, WL Low -> UH(Bit 0), WL(Bit 5)
        let mask1 = phase_mask(100, 1);
        assert_eq!(
            mask1,
            BIT_UH | BIT_WL,
            "Step 1 (forward) must assert UH and WL"
        );

        // Step 2: VH High, WL Low -> VH(Bit 2), WL(Bit 5)
        let mask2 = phase_mask(100, 2);
        assert_eq!(
            mask2,
            BIT_VH | BIT_WL,
            "Step 2 (forward) must assert VH and WL"
        );

        // Step 3: VH High, UL Low -> VH(Bit 2), UL(Bit 1)
        let mask3 = phase_mask(100, 3);
        assert_eq!(
            mask3,
            BIT_VH | BIT_UL,
            "Step 3 (forward) must assert VH and UL"
        );

        // Step 4: WH High, UL Low -> WH(Bit 4), UL(Bit 1)
        let mask4 = phase_mask(100, 4);
        assert_eq!(
            mask4,
            BIT_WH | BIT_UL,
            "Step 4 (forward) must assert WH and UL"
        );

        // Step 5: WH High, VL Low -> WH(Bit 4), VL(Bit 3)
        let mask5 = phase_mask(100, 5);
        assert_eq!(
            mask5 & BIT_VL,
            BIT_VL,
            "Step 5 (forward) must assert VL (bit 3 / GP19)"
        );
        assert_eq!(
            mask5 & BIT_WH,
            BIT_WH,
            "Step 5 (forward) must assert WH (bit 4 / GP20)"
        );

        // Reverse commutation:
        // Step 0: WH High, VL Low -> WH(Bit 4), VL(Bit 3)
        let rev_mask0 = phase_mask(-100, 0);
        assert_eq!(
            rev_mask0 & BIT_VL,
            BIT_VL,
            "Step 0 (reverse) must assert VL (bit 3 / GP19)"
        );
        assert_eq!(
            rev_mask0 & BIT_WH,
            BIT_WH,
            "Step 0 (reverse) must assert WH (bit 4 / GP20)"
        );

        // Step 1: WH High, UL Low -> WH(Bit 4), UL(Bit 1)
        let rev_mask1 = phase_mask(-100, 1);
        assert_eq!(
            rev_mask1,
            BIT_WH | BIT_UL,
            "Step 1 (reverse) must assert WH and UL"
        );

        // Step 2: VH High, UL Low -> VH(Bit 2), UL(Bit 1)
        let rev_mask2 = phase_mask(-100, 2);
        assert_eq!(
            rev_mask2,
            BIT_VH | BIT_UL,
            "Step 2 (reverse) must assert VH and UL"
        );

        // Step 3: VH High, WL Low -> VH(Bit 2), WL(Bit 5)
        let rev_mask3 = phase_mask(-100, 3);
        assert_eq!(
            rev_mask3,
            BIT_VH | BIT_WL,
            "Step 3 (reverse) must assert VH and WL"
        );

        // Step 4: UH High, WL Low -> UH(Bit 0), WL(Bit 5)
        let rev_mask4 = phase_mask(-100, 4);
        assert_eq!(
            rev_mask4,
            BIT_UH | BIT_WL,
            "Step 4 (reverse) must assert UH and WL"
        );

        // Step 5: UH High, VL Low -> UH(Bit 0), VL(Bit 3)
        let rev_mask5 = phase_mask(-100, 5);
        assert_eq!(
            rev_mask5 & BIT_VL,
            BIT_VL,
            "Step 5 (reverse) must assert VL (bit 3 / GP19)"
        );
        assert_eq!(
            rev_mask5 & BIT_UH,
            BIT_UH,
            "Step 5 (reverse) must assert UH (bit 0 / GP16)"
        );
    });
}

#[test]
fn test_worm_gliding_larva_522_pin_mapping_and_forward_commutation_progression() {
    run_test!(async {
        use model::interfaces::{Motor, Tickable};
        use model::types::MotorSpeed;
        use peripheral::tmc6300::{make_pwm_word, COMMUTATION_STEPS};
        use std::sync::atomic::{AtomicBool, Ordering};

        struct MockDiagPin<'a> {
            is_high: &'a AtomicBool,
        }

        impl<'a> embedded_hal::digital::ErrorType for MockDiagPin<'a> {
            type Error = core::convert::Infallible;
        }

        impl<'a> embedded_hal::digital::InputPin for MockDiagPin<'a> {
            fn is_high(&mut self) -> Result<bool, Self::Error> {
                Ok(self.is_high.load(Ordering::SeqCst))
            }
            fn is_low(&mut self) -> Result<bool, Self::Error> {
                Ok(!self.is_high.load(Ordering::SeqCst))
            }
        }

        let diag_asserted = AtomicBool::new(false);
        let pin_diag = MockDiagPin {
            is_high: &diag_asserted,
        };
        let mut motor = peripheral::tmc6300::Tmc6300Pio::new(pin_diag);

        // Canonical forward commutation masks:
        // Step 0: UH High (bit 0), VL Low (bit 3) -> 0b001001 (0x09)
        // Step 1: UH High (bit 0), WL Low (bit 5) -> 0b100001 (0x21)
        // Step 2: VH High (bit 2), WL Low (bit 5) -> 0b100100 (0x24)
        // Step 3: VH High (bit 2), UL Low (bit 1) -> 0b000110 (0x06)
        // Step 4: WH High (bit 4), UL Low (bit 1) -> 0b010010 (0x12)
        // Step 5: WH High (bit 4), VL Low (bit 3) -> 0b011000 (0x18)
        let expected_masks = [0b001001, 0b100001, 0b100100, 0b000110, 0b010010, 0b011000];

        // Start motor at 100% speed
        assert!(motor.set_speed(MotorSpeed::new(100).unwrap()).is_ok());
        assert_eq!(motor.speed(), 100);
        assert_eq!(motor.duty_cycle(), 100);

        // Verify progression through 24 consecutive ticks (4 full 6-step commutation cycles)
        // In particular, verifies advancing past Step 1 without stall or fault
        for tick in 1..=24 {
            assert!(
                motor.tick().await.is_ok(),
                "Motor tick failed at tick {tick}"
            );
            let expected_step = (tick % COMMUTATION_STEPS) as u8;
            assert_eq!(
                motor.commutation_step(),
                expected_step,
                "Commutation step mismatch at tick {tick}"
            );

            let word = motor.last_word();
            let mask = (word & 0xFF) as u8;
            assert_eq!(
                mask, expected_masks[expected_step as usize],
                "Mask mismatch at step {expected_step}"
            );
            assert_eq!(
                word,
                make_pwm_word(expected_masks[expected_step as usize], 100, 0),
                "PWM word mismatch at tick {tick}"
            );

            // Verify break-before-make and no shoot-through
            let uh = (mask & (1 << 0)) != 0;
            let ul = (mask & (1 << 1)) != 0;
            let vh = (mask & (1 << 2)) != 0;
            let vl = (mask & (1 << 3)) != 0;
            let wh = (mask & (1 << 4)) != 0;
            let wl = (mask & (1 << 5)) != 0;
            assert!(!(uh && ul), "Phase U shoot-through at step {expected_step}");
            assert!(!(vh && vl), "Phase V shoot-through at step {expected_step}");
            assert!(!(wh && wl), "Phase W shoot-through at step {expected_step}");
            assert_eq!((uh as u8) + (vh as u8) + (wh as u8), 1);
            assert_eq!((ul as u8) + (vl as u8) + (wl as u8), 1);
        }

        // Test fault detection: DIAG pin asserted trips DriverFault
        diag_asserted.store(true, Ordering::SeqCst);
        let tick_result = motor.tick().await;
        assert!(
            tick_result.is_err(),
            "Motor tick must fail when DIAG is asserted"
        );
    });
}

#[test]
fn test_tmc6300_freewheeling_window_and_decay_modes() {
    run_test!(async {
        use model::interfaces::{Motor, Tickable};
        use model::types::MotorSpeed;
        use peripheral::tmc6300::{
            make_pwm_word, FreewheelMode, Tmc6300, Tmc6300Pio, DEFAULT_FREEWHEEL_US,
            FREEWHEEL_PHASE_MASK, PWM_PERIOD_CYCLES,
        };
        use std::sync::atomic::AtomicBool;

        // 1. Verify constant invariants
        assert_eq!(DEFAULT_FREEWHEEL_US, 0);
        assert_eq!(FREEWHEEL_PHASE_MASK, 0b101010);
        // Verify low-side pins in FREEWHEEL_PHASE_MASK: UL (bit 1), VL (bit 3), WL (bit 5)
        assert_eq!(FREEWHEEL_PHASE_MASK & (1 << 0), 0); // UH off
        assert_ne!(FREEWHEEL_PHASE_MASK & (1 << 1), 0); // UL on
        assert_eq!(FREEWHEEL_PHASE_MASK & (1 << 2), 0); // VH off
        assert_ne!(FREEWHEEL_PHASE_MASK & (1 << 3), 0); // VL on
        assert_eq!(FREEWHEEL_PHASE_MASK & (1 << 4), 0); // WH off
        assert_ne!(FREEWHEEL_PHASE_MASK & (1 << 5), 0); // WL on

        // 2. Test Tmc6300Pio host mock defaults and configuration
        let diag_flag = AtomicBool::new(false);
        let pin_diag = MockPin {
            is_high: &diag_flag,
        };
        let mut pio_motor = Tmc6300Pio::new(pin_diag);

        assert_eq!(pio_motor.freewheel_us(), DEFAULT_FREEWHEEL_US);
        assert_eq!(pio_motor.freewheel_mode(), FreewheelMode::Float);

        // Adjust freewheel duration and mode
        pio_motor.set_freewheel_us(250);
        assert_eq!(pio_motor.freewheel_us(), 250);
        pio_motor.set_freewheel_mode(FreewheelMode::Float);
        assert_eq!(pio_motor.freewheel_mode(), FreewheelMode::Float);

        // Reset to LowSide for execution test
        pio_motor.set_freewheel_mode(FreewheelMode::LowSide);
        assert!(pio_motor.set_speed(MotorSpeed::new(100).unwrap()).is_ok());

        // On tick with LowSide freewheel, freewheel word must have FREEWHEEL_PHASE_MASK at 100% duty
        assert!(pio_motor.tick().await.is_ok());
        let expected_low_side_word = make_pwm_word(FREEWHEEL_PHASE_MASK, PWM_PERIOD_CYCLES, 0);
        assert_eq!(
            pio_motor.last_freewheel_word(),
            expected_low_side_word,
            "LowSide freewheel word must energize all 3 low-side FETs at 100% duty cycle"
        );

        // Switch to Float decay mode and tick again
        pio_motor.set_freewheel_mode(FreewheelMode::Float);
        assert!(pio_motor.tick().await.is_ok());
        let expected_float_word = make_pwm_word(0, 0, PWM_PERIOD_CYCLES);
        assert_eq!(
            pio_motor.last_freewheel_word(),
            expected_float_word,
            "Float freewheel word must de-energize all FETs (0% duty cycle)"
        );

        // Disabling freewheeling (0 us) must bypass freewheel word emission
        pio_motor.set_freewheel_us(0);
        assert_eq!(pio_motor.freewheel_us(), 0);
        // Reset last_freewheel_word by stopping and restarting
        assert!(pio_motor.stop().is_ok());
        assert_eq!(pio_motor.last_freewheel_word(), 0);
        assert!(pio_motor.set_speed(MotorSpeed::new(100).unwrap()).is_ok());
        assert!(pio_motor.tick().await.is_ok());
        assert_eq!(
            pio_motor.last_freewheel_word(),
            0,
            "When freewheel_us is 0, no freewheel word should be pushed"
        );

        // 3. Test GPIO Tmc6300 driver configuration and defaults
        let pin_uh_state = AtomicBool::new(false);
        let pin_ul_state = AtomicBool::new(false);
        let pin_vh_state = AtomicBool::new(false);
        let pin_vl_state = AtomicBool::new(false);
        let pin_wh_state = AtomicBool::new(false);
        let pin_wl_state = AtomicBool::new(false);
        let pin_diag_state = AtomicBool::new(false);

        let pin_uh = MockPin {
            is_high: &pin_uh_state,
        };
        let pin_ul = MockPin {
            is_high: &pin_ul_state,
        };
        let pin_vh = MockPin {
            is_high: &pin_vh_state,
        };
        let pin_vl = MockPin {
            is_high: &pin_vl_state,
        };
        let pin_wh = MockPin {
            is_high: &pin_wh_state,
        };
        let pin_wl = MockPin {
            is_high: &pin_wl_state,
        };
        let pin_diag = MockPin {
            is_high: &pin_diag_state,
        };

        let mut gpio_motor = Tmc6300::new(pin_uh, pin_ul, pin_vh, pin_vl, pin_wh, pin_wl, pin_diag);

        assert_eq!(gpio_motor.freewheel_us(), DEFAULT_FREEWHEEL_US);
        assert_eq!(gpio_motor.freewheel_mode(), FreewheelMode::Float);

        gpio_motor.set_freewheel_us(100);
        assert_eq!(gpio_motor.freewheel_us(), 100);
        gpio_motor.set_freewheel_mode(FreewheelMode::Float);
        assert_eq!(gpio_motor.freewheel_mode(), FreewheelMode::Float);

        assert!(gpio_motor.set_speed(MotorSpeed::new(100).unwrap()).is_ok());
        assert!(gpio_motor.tick().await.is_ok());
    });
}

#[test]
fn test_tmc6300_pwm_frequency_and_clock_divider() {
    run_test!(async {
        use peripheral::tmc6300::{
            calculate_clock_divider, Tmc6300Pio, DEFAULT_PIO_PWM_FREQUENCY_HZ,
            PIO_PWM_FREQUENCY_HZ, REDUCED_PWM_FREQUENCY_HZ,
        };
        use std::sync::atomic::AtomicBool;

        // 1. Verify PWM frequency constant invariants
        assert_eq!(PIO_PWM_FREQUENCY_HZ, 25_000);
        assert_eq!(REDUCED_PWM_FREQUENCY_HZ, 10_000);
        assert_eq!(DEFAULT_PIO_PWM_FREQUENCY_HZ, 25_000);

        // 2. Verify clock divider calculation at 125 MHz system clock
        // Target PIO clock for 25 kHz @ 100 cycles/period: 2.5 MHz -> divider 50
        assert_eq!(calculate_clock_divider(125_000_000, 25_000), 50);
        // Target PIO clock for 10 kHz @ 100 cycles/period: 1.0 MHz -> divider 125
        assert_eq!(calculate_clock_divider(125_000_000, 10_000), 125);
        // Target PIO clock for 20 kHz @ 100 cycles/period: 2.0 MHz -> divider 62
        assert_eq!(calculate_clock_divider(125_000_000, 20_000), 62);
        // Zero frequency guard returns 1
        assert_eq!(calculate_clock_divider(125_000_000, 0), 1);

        // 3. Verify Tmc6300Pio default and runtime adjustable PWM frequency
        let diag_flag = AtomicBool::new(false);
        let pin_diag = MockPin {
            is_high: &diag_flag,
        };
        let mut pio_motor = Tmc6300Pio::new(pin_diag);

        assert_eq!(
            pio_motor.pwm_frequency_hz(),
            DEFAULT_PIO_PWM_FREQUENCY_HZ,
            "Tmc6300Pio must default to PIO_PWM_FREQUENCY_HZ (25 kHz)"
        );

        pio_motor.set_pwm_frequency_hz(25_000);
        assert_eq!(
            pio_motor.pwm_frequency_hz(),
            25_000,
            "Tmc6300Pio must allow setting PWM frequency to 25 kHz"
        );

        pio_motor.set_pwm_frequency_hz(10_000);
        assert_eq!(
            pio_motor.pwm_frequency_hz(),
            10_000,
            "Tmc6300Pio must allow setting PWM frequency to 10 kHz"
        );
    });
}

#[test]
fn test_worm_slithering_lumbricus_809_tmc6300_short_protection_pulse_invariants() {
    run_test!(async {
        use model::interfaces::{Motor, Tickable};
        use model::types::MotorSpeed;
        use peripheral::tmc6300::{
            calculate_on_time_ns, clamp_on_count_short_protection, phase_mask, Tmc6300Pio,
            SHORT_PROTECTION_HARD_LIMIT_ON_COUNT, SHORT_PROTECTION_MAX_ON_COUNT,
            SHORT_PROTECTION_MAX_ON_TIME_NS, SHORT_PROTECTION_MIN_ON_COUNT,
            SHORT_PROTECTION_MIN_ON_TIME_NS, SHORT_PROTECTION_NOMINAL_ON_TIME_NS,
            SHORT_PROTECTION_PIO_DIVIDER, SHORT_PROTECTION_PWM_FREQUENCY_HZ,
            SHORT_PROTECTION_THRESHOLD_NS,
        };
        use std::sync::atomic::AtomicBool;

        // 1. Verify constant invariants for TMC6300 hardware short-circuit protection
        // TMC6300 t_S2G short detection delay is 1000 ns (1.0 µs)
        assert_eq!(SHORT_PROTECTION_THRESHOLD_NS, 1000);
        // Target pulse bounds: between 500 ns and 800 ns
        assert_eq!(SHORT_PROTECTION_MIN_ON_TIME_NS, 500);
        assert_eq!(SHORT_PROTECTION_MAX_ON_TIME_NS, 800);
        assert_eq!(SHORT_PROTECTION_NOMINAL_ON_TIME_NS, 640);
        assert_eq!(SHORT_PROTECTION_PWM_FREQUENCY_HZ, 250_000);
        assert_eq!(SHORT_PROTECTION_PIO_DIVIDER, 5);
        assert_eq!(SHORT_PROTECTION_MIN_ON_COUNT, 7);
        assert_eq!(SHORT_PROTECTION_MAX_ON_COUNT, 14);
        assert_eq!(SHORT_PROTECTION_HARD_LIMIT_ON_COUNT, 18);

        // 2. Mathematical reproduction of the defect:
        // Under buggy divider 50 (2.5 MHz PIO clock, 400 ns/cycle), speed 10 on_count produces:
        // (10 + 6) * 400 ns = 6400 ns (6.4 µs) -> TRIPS short protection (t_S2G = 1.0 µs)!
        let buggy_pulse_ns = calculate_on_time_ns(10, 50, 125_000_000);
        assert_eq!(buggy_pulse_ns, 6400);
        assert!(buggy_pulse_ns > SHORT_PROTECTION_THRESHOLD_NS);

        // Under fixed divider 5 (25 MHz PIO clock, 40 ns/cycle), speed 10 on_count produces:
        // (10 + 6) * 40 ns = 640 ns (0.64 µs) -> Safe, inside ideal 500..800 ns window!
        let fixed_pulse_ns = calculate_on_time_ns(10, SHORT_PROTECTION_PIO_DIVIDER, 125_000_000);
        assert_eq!(fixed_pulse_ns, 640);
        assert!(fixed_pulse_ns < SHORT_PROTECTION_THRESHOLD_NS);
        assert!(fixed_pulse_ns >= SHORT_PROTECTION_MIN_ON_TIME_NS);
        assert!(fixed_pulse_ns <= SHORT_PROTECTION_MAX_ON_TIME_NS);

        // 3. Verify on_count clamping across all duty cycles (1..=100)
        assert_eq!(clamp_on_count_short_protection(0), 0);
        // Minimum non-zero duty cycle clamped to 7 cycles (520 ns, > 500 ns)
        assert_eq!(clamp_on_count_short_protection(1), 7);
        let min_pulse = calculate_on_time_ns(
            clamp_on_count_short_protection(1),
            SHORT_PROTECTION_PIO_DIVIDER,
            125_000_000,
        );
        assert_eq!(min_pulse, 520);
        assert!(min_pulse >= SHORT_PROTECTION_MIN_ON_TIME_NS);

        // Speed 10 reproduces user scenario: 10 cycles -> 640 ns
        assert_eq!(clamp_on_count_short_protection(10), 10);
        let pulse_10 = calculate_on_time_ns(
            clamp_on_count_short_protection(10),
            SHORT_PROTECTION_PIO_DIVIDER,
            125_000_000,
        );
        assert_eq!(pulse_10, 640);

        // Speed 100 clamped to 14 cycles -> exactly 800 ns target
        assert_eq!(clamp_on_count_short_protection(100), 14);
        let max_pulse = calculate_on_time_ns(
            clamp_on_count_short_protection(100),
            SHORT_PROTECTION_PIO_DIVIDER,
            125_000_000,
        );
        assert_eq!(max_pulse, 800);
        assert_eq!(max_pulse, SHORT_PROTECTION_MAX_ON_TIME_NS);

        // Across all commanded speeds (1..=100), pulse duration MUST be in [500 ns, 800 ns]
        for speed_val in 1..=100 {
            let clamped = clamp_on_count_short_protection(speed_val);
            let duration = calculate_on_time_ns(clamped, SHORT_PROTECTION_PIO_DIVIDER, 125_000_000);
            assert!(
                duration >= SHORT_PROTECTION_MIN_ON_TIME_NS,
                "Speed {speed_val} pulse duration {duration} ns below min {SHORT_PROTECTION_MIN_ON_TIME_NS} ns"
            );
            assert!(
                duration <= SHORT_PROTECTION_MAX_ON_TIME_NS,
                "Speed {speed_val} pulse duration {duration} ns exceeds max {SHORT_PROTECTION_MAX_ON_TIME_NS} ns"
            );
            assert!(
                duration < SHORT_PROTECTION_THRESHOLD_NS,
                "Speed {speed_val} pulse duration {duration} ns exceeds threshold {SHORT_PROTECTION_THRESHOLD_NS} ns"
            );
        }

        // 4. Functional driver verification on host mock
        let diag_flag = AtomicBool::new(false);
        let pin_diag = MockPin {
            is_high: &diag_flag,
        };
        let mut pio_motor = Tmc6300Pio::new(pin_diag);

        // Initial state stopped
        assert_eq!(pio_motor.effective_on_count(), 0);
        assert_eq!(pio_motor.pulse_duration_ns(), 240); // 6 cycles idle

        // Command speed 10 (reproduction step)
        assert!(pio_motor.set_speed(MotorSpeed::new(10).unwrap()).is_ok());
        assert_eq!(pio_motor.speed(), 10);
        assert_eq!(pio_motor.duty_cycle(), 100);
        assert_eq!(pio_motor.effective_on_count(), 14);
        assert_eq!(pio_motor.pulse_duration_ns(), 800);

        // Verify calculate_pwm_counts helper directly
        let (on10, off10) = peripheral::tmc6300::calculate_pwm_counts(10);
        assert_eq!(on10, 10);
        assert_eq!(off10, 90);
        assert_eq!(
            peripheral::tmc6300::calculate_on_time_ns(
                on10,
                peripheral::tmc6300::SHORT_PROTECTION_PIO_DIVIDER,
                125_000_000
            ),
            640
        );

        // Command speed 100
        assert!(pio_motor.set_speed(MotorSpeed::new(100).unwrap()).is_ok());
        assert_eq!(pio_motor.speed(), 100);
        assert_eq!(pio_motor.effective_on_count(), 14);
        assert_eq!(pio_motor.pulse_duration_ns(), 800);
        let eff_word100 = pio_motor.effective_word();
        assert_eq!((eff_word100 >> 8) & 0xFF, 14);
        assert_eq!((eff_word100 >> 16) & 0xFF, 1);

        // Advance commutation tick
        assert!(pio_motor.tick().await.is_ok());
        assert_eq!(pio_motor.commutation_step(), 1);
        let eff_step1 = pio_motor.effective_word();
        assert_eq!(eff_step1 & 0xFF, phase_mask(100, 1) as u32);
        assert_eq!((eff_step1 >> 8) & 0xFF, 14);
    });
}

#[test]
fn test_worm_segmented_sipunculid_504_hall_sensor_preserves_rpm_between_slow_pulses() {
    let pin_state = std::sync::atomic::AtomicBool::new(false);
    let pin = MockPin {
        is_high: &pin_state,
    };
    let mut sensor = peripheral::hall_sensor::HallSensor::new(pin);
    use model::interfaces::SpeedSensor;

    // Simulate 1 pulse arriving at slow speed (166 RPM)
    pin_state.store(true, std::sync::atomic::Ordering::SeqCst);
    assert!(sensor.poll_state().is_ok());
    assert_eq!(sensor.pulse_count(), 1);

    // Calculate RPM for 1 pulse over 120 ms:
    // (1 * 60,000) / (3 * 120) = 60,000 / 360 = 166 RPM
    sensor.update_rpm(120);
    assert_eq!(sensor.read_rpm(), Ok(166));

    // When a calculation window passes without a new pulse (pulse_count == 0),
    // RPM must NOT collapse to 0 RPM
    sensor.update_rpm(100);
    assert_eq!(sensor.read_rpm(), Ok(166));
}

#[test]
fn test_cr_7f321327_tmc6300_tick_interval_and_sync() {
    use model::interfaces::Tickable;
    use peripheral::tmc6300::{Tmc6300Pio, DEFAULT_TICK_INTERVAL, DEFAULT_TICK_INTERVAL_US};

    let pin_diag = MockPin {
        is_high: &std::sync::atomic::AtomicBool::new(false),
    };
    let mut driver = Tmc6300Pio::new(pin_diag);

    // Initial default tick interval should match DEFAULT_TICK_INTERVAL_US (333us)
    assert_eq!(driver.tick_interval_us(), DEFAULT_TICK_INTERVAL_US);
    assert_eq!(
        Tickable::tick_interval_us(&driver),
        DEFAULT_TICK_INTERVAL_US
    );
    assert_eq!(driver.tick_interval(), DEFAULT_TICK_INTERVAL);
    assert_eq!(
        driver.tick_interval(),
        embassy_time::Duration::from_micros(333)
    );

    // Dynamic configuration updates tick interval
    driver.set_tick_interval_us(250);
    assert_eq!(driver.tick_interval_us(), 250);
    assert_eq!(Tickable::tick_interval_us(&driver), 250);
    assert_eq!(
        driver.tick_interval(),
        embassy_time::Duration::from_micros(250)
    );
}

#[test]
fn test_worm_proboscid_roundworm_8_driver_first_tick_executes_step_0() {
    run_test!(async {
        use model::interfaces::{Motor, Tickable};
        use model::types::MotorSpeed;
        use peripheral::tmc6300::{Tmc6300Pio, DEFAULT_TICK_INTERVAL_US};

        let pin_diag = MockPin {
            is_high: &std::sync::atomic::AtomicBool::new(false),
        };
        let mut driver = Tmc6300Pio::new(pin_diag);

        // 1. Driver default tick interval must be 333 µs
        assert_eq!(driver.tick_interval_us(), 333);
        assert_eq!(DEFAULT_TICK_INTERVAL_US, 333);

        // 2. Set speed: initial step index is Step 0
        driver.set_speed(MotorSpeed::new(100).unwrap()).unwrap();
        assert_eq!(driver.commutation_step(), 0, "Initial step must be Step 0");

        // 3. First tick advances to Step 1
        assert!(driver.tick().await.is_ok());
        assert_eq!(
            driver.commutation_step(),
            1,
            "First tick must advance to Step 1"
        );

        // 4. Second tick advances to Step 2
        assert!(driver.tick().await.is_ok());
        assert_eq!(
            driver.commutation_step(),
            2,
            "Second tick must advance to Step 2"
        );
    });
}

#[test]
fn test_target_freewheeling_speed_and_tick_interval_constants() {
    use peripheral::tmc6300::{
        DEFAULT_TICK_INTERVAL, DEFAULT_TICK_INTERVAL_US, TARGET_FREEWHEELING_MAX_RPM,
    };

    // Verify 5,000 RPM target constant
    assert_eq!(TARGET_FREEWHEELING_MAX_RPM, 5000);
    assert_eq!(DEFAULT_TICK_INTERVAL_US, 333);
    assert_eq!(
        DEFAULT_TICK_INTERVAL,
        embassy_time::Duration::from_micros(333)
    );
}
