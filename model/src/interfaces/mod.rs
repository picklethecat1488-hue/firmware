//! Generic hardware peripheral interfaces/traits.

#![deny(missing_docs)]

pub mod motor;
pub use motor::Motor;

pub mod fuel_gauge;
pub use fuel_gauge::FuelGauge;

pub mod power_sensor;
pub use power_sensor::{PowerMeasurementMode, PowerSensor};

pub mod proximity;
pub use proximity::ProximitySensor;

pub mod temperature;
pub use temperature::TemperatureSensor;

pub mod charge_status;
pub use charge_status::ChargeStatus;

pub mod led;
pub use led::LedDriver;
