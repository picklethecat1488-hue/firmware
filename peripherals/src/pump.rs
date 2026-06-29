use embedded_hal::digital::OutputPin;

/// Interface for controlling a fluid pump.
pub trait Pump {
    /// Error type returned by the physical hardware.
    type Error;

    /// Sets the pump speed (0 to 255).
    fn set_speed(&mut self, speed: u8) -> Result<(), Self::Error>;

    /// Stops the pump completely.
    fn stop(&mut self) -> Result<(), Self::Error>;
}

/// A generic platform-agnostic GPIO implementation of a Pump.
pub struct GpioPump<P: OutputPin> {
    pin: P,
}

impl<P: OutputPin> GpioPump<P> {
    /// Creates a new generic GPIO pump.
    pub const fn new(pin: P) -> Self {
        Self { pin }
    }
}

impl<P: OutputPin> Pump for GpioPump<P> {
    type Error = P::Error;

    /// Sets pump speed. Since this is GPIO, speed > 0 sets Pin High, and 0 sets Pin Low.
    fn set_speed(&mut self, speed: u8) -> Result<(), Self::Error> {
        if speed > 0 {
            self.pin.set_high()
        } else {
            self.pin.set_low()
        }
    }

    /// Stops the pump by pulling the GPIO pin Low.
    fn stop(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }
}
