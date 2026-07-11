use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use peripherals::mock::DummyI2c;
use peripherals::{Motor, MotorSpeed, Tickable};

struct MockPin<'a> {
    is_high: &'a core::cell::Cell<bool>,
}

impl<'a> ErrorType for MockPin<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> OutputPin for MockPin<'a> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.is_high.set(false);
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.is_high.set(true);
        Ok(())
    }
}

impl<'a> InputPin for MockPin<'a> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_high.get())
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_high.get())
    }
}

#[test]
fn test_l9110s_functional() {
    let pin_ia_state = core::cell::Cell::new(false);
    let pin_ib_state = core::cell::Cell::new(false);
    let pin_ia = MockPin {
        is_high: &pin_ia_state,
    };
    let pin_ib = MockPin {
        is_high: &pin_ib_state,
    };
    let mut motor = peripherals::l9110s::L9110s::new(pin_ia, pin_ib);

    // 1. Initially both low
    assert!(!pin_ia_state.get());
    assert!(!pin_ib_state.get());

    // 2. Setting speed > 0 drives pin_ia high and pin_ib low
    assert!(motor.set_speed(MotorSpeed::new(100).unwrap()).is_ok());
    assert!(pin_ia_state.get());
    assert!(!pin_ib_state.get());

    // 3. Setting speed == 0 brakes both pins to low
    assert!(motor.set_speed(MotorSpeed::ZERO).is_ok());
    assert!(!pin_ia_state.get());
    assert!(!pin_ib_state.get());

    // 4. Stopping brakes both pins to low
    assert!(motor.set_speed(MotorSpeed::new(50).unwrap()).is_ok());
    assert!(pin_ia_state.get());
    assert!(!pin_ib_state.get());
    assert!(motor.stop().is_ok());
    assert!(!pin_ia_state.get());
    assert!(!pin_ib_state.get());
}

#[test]
fn test_vl53l0x_threshold_validation() {
    use model::calibration::{Calibration, CalibrationType};
    use peripherals::vl53l0x::Vl53l0x;

    let mut sensor = Vl53l0x::new(DummyI2c, 0x30);
    // Default threshold is 300, cal_near is 0.

    // 1. Setting threshold to > cal_near + THRESHOLD_ERROR_MM should succeed.
    assert!(sensor.set_threshold_mm(250).is_ok());

    // 2. Setting threshold to <= cal_near + THRESHOLD_ERROR_MM should return an error.
    let mut s = Vl53l0x::new(DummyI2c, 0x30);
    assert!(s.set_threshold_mm(10).is_err());

    // 3. Setting calibration with threshold_mm > near + THRESHOLD_ERROR_MM should succeed.
    sensor.set_calibration(CalibrationType::ProximityCal(
        model::calibration::TwoPointCalibration::new(50, 150),
    ));

    // 4. Setting calibration with threshold_mm <= near + THRESHOLD_ERROR_MM should be ignored.
    let mut s = Vl53l0x::new(DummyI2c, 0x30);
    let _ = s.set_threshold_mm(100);
    s.set_calibration(CalibrationType::ProximityCal(
        model::calibration::TwoPointCalibration::new(90, 150),
    ));
    assert_eq!(s.calibration().low, 0);
}

#[test]
fn test_motor_duty_cycling_ticks() {
    let pin_ia_state = core::cell::Cell::new(false);
    let pin_ib_state = core::cell::Cell::new(false);
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
    assert!(pin_ia_state.get());
    assert!(!pin_ib_state.get());

    // Tick 1 to 2: active (total 3 active ticks: 0, 1, 2)
    for _ in 1..=2 {
        assert!(motor.tick().is_ok());
        assert!(pin_ia_state.get());
        assert!(!pin_ib_state.get());
    }

    // Tick 3: becomes inactive (tick_counter reaches 3 >= threshold 3)
    assert!(motor.tick().is_ok());
    assert!(!pin_ia_state.get());
    assert!(!pin_ib_state.get());

    // Ticks up to 9: inactive
    for _ in 4..=9 {
        assert!(motor.tick().is_ok());
        assert!(!pin_ia_state.get());
        assert!(!pin_ib_state.get());
    }

    // Tick 10: resets counter, becomes active again
    assert!(motor.tick().is_ok());
    assert!(pin_ia_state.get());
    assert!(!pin_ib_state.get());
}

struct SpyI2c<'a> {
    writes: &'a std::cell::RefCell<Vec<(u8, Vec<u8>)>>,
}

impl<'a> embedded_hal::i2c::ErrorType for SpyI2c<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> embedded_hal::i2c::I2c for SpyI2c<'a> {
    fn read(&mut self, _address: u8, _read: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        self.writes.borrow_mut().push((address, write.to_vec()));
        Ok(())
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

#[test]
fn test_vl53l0x_init() {
    use peripherals::vl53l0x::{InterruptMode, Vl53l0x};

    let writes = std::cell::RefCell::new(Vec::new());
    let i2c = SpyI2c { writes: &writes };
    let mut sensor = Vl53l0x::new(i2c, 0x29);

    // Call init to change address to 0x30, threshold to 250, and interrupt to LowLevel
    let res = sensor.init(0x30, 250, InterruptMode::LowLevel);
    assert!(res.is_ok());

    // Verify properties are updated
    assert_eq!(sensor.threshold_mm(), 250);

    // Verify written values
    let w = writes.borrow();
    assert_eq!(w.len(), 5);

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
}
