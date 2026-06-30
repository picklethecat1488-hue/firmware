//! Generic hardware peripheral interfaces/traits.

#![deny(missing_docs)]

pub mod motor;
pub use motor::Motor;

pub mod current_sensor;
pub use current_sensor::CurrentSensor;

pub mod fuel_gauge;
pub use fuel_gauge::FuelGauge;

pub mod power_sensor;
pub use power_sensor::PowerSensor;

pub mod proximity;
pub use proximity::ProximitySensor;

pub mod temperature;
pub use temperature::TemperatureSensor;

pub mod charger;
pub use charger::Charger;

pub mod led;
pub use led::LedDriver;
