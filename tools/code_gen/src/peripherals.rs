use rinja::Template;
use serde::Deserialize;

/// Represents either a single bus name or a list of bus names.
#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum BusConfig {
    /// A single bus configuration.
    Single(String),
    /// A list of multiple bus configurations.
    Multiple(Vec<String>),
}

impl BusConfig {
    /// Returns the bus configurations as a vector of strings.
    pub fn to_vec(&self) -> Vec<String> {
        match self {
            Self::Single(s) => vec![s.clone()],
            Self::Multiple(v) => v.clone(),
        }
    }
}

/// Initialization configuration properties for a peripheral.
#[derive(Deserialize, Clone, Default)]
pub struct InitConfig {
    /// Flag indicating if the peripheral has a reset method.
    #[serde(default)]
    pub reset: bool,
    /// Flag indicating if the peripheral has an init method.
    #[serde(default)]
    pub init: bool,
    /// Flag indicating if the peripheral has a probeable/identification check.
    #[serde(default)]
    pub probe: bool,
}

/// Metadata configuration for a single peripheral parsed from `peripherals.toml`.
#[derive(Deserialize, Clone)]
pub struct Peripheral {
    /// Name of the peripheral.
    pub name: String,
    /// Module name under the peripherals crate.
    #[serde(default)]
    pub module: Option<String>,
    /// Struct name of the driver.
    #[serde(default)]
    pub struct_name: Option<String>,
    /// Bus configuration (single string or array of strings).
    #[serde(default)]
    pub bus: Option<BusConfig>,
    /// Flag indicating if the peripheral has a PIO block.
    #[serde(default)]
    pub has_pio: bool,
    /// Initialization configuration properties.
    #[serde(default)]
    pub init: InitConfig,
}

impl Peripheral {
    /// Gets the resolved module name.
    pub fn module_name(&self) -> String {
        self.module
            .clone()
            .unwrap_or_else(|| self.name.to_lowercase())
    }

    /// Gets the resolved struct name.
    pub fn struct_name_resolved(&self) -> String {
        self.struct_name
            .clone()
            .unwrap_or_else(|| self.name.clone())
    }

    /// Gets the list of supported buses.
    pub fn buses(&self) -> Vec<String> {
        self.bus.as_ref().map(|b| b.to_vec()).unwrap_or_default()
    }

    /// Returns true if this peripheral supports multiple buses.
    pub fn is_multi_bus(&self) -> bool {
        self.buses().len() > 1
    }

    /// Gets the macro suffix/name for the given bus.
    pub fn macro_name(&self, bus: &str) -> String {
        if self.is_multi_bus() {
            format!("{}_{}", self.module_name(), bus)
        } else {
            self.module_name()
        }
    }

    /// Returns true if this peripheral supports the given bus.
    pub fn has_bus(&self, name: &str) -> bool {
        self.buses().iter().any(|b| b == name)
    }
}

/// The outer configuration structure for peripherals.toml.
#[derive(Deserialize, Clone)]
pub struct PeripheralConfig {
    /// List of configured peripherals.
    pub peripherals: Vec<Peripheral>,
}

/// Template structure for peripheral sample generator.
#[derive(Template)]
#[template(path = "peripheral_sample.rs.jinja")]
pub struct PeripheralSampleTemplate {
    /// Name of the peripheral to show a sample for.
    pub name: String,
    /// Flag indicating if the Probeable trait should be implemented.
    pub has_probeable: bool,
    /// Flag indicating if the LedDriver trait should be implemented.
    pub has_led_driver: bool,
    /// Flag indicating if the FuelGauge trait should be implemented.
    pub has_fuel_gauge: bool,
    /// Flag indicating if the Tickable trait should be implemented.
    pub has_tickable: bool,
    /// Flag indicating if the ChargeStatus trait should be implemented.
    pub has_charge_status: bool,
    /// Flag indicating if the ProximitySensor trait should be implemented.
    pub has_proximity_sensor: bool,
}

/// Template structure for peripheral initializers generator.
#[derive(Template)]
#[template(path = "peripheral_initializers.rs.jinja")]
pub struct PeripheralInitializersTemplate {
    /// List of configured peripherals.
    pub peripherals: Vec<Peripheral>,
}

/// Generates macro definitions for initializing the peripherals from a TOML string representation.
pub fn generate_peripheral_initializers(toml_str: &str) -> String {
    let config: PeripheralConfig =
        toml::from_str(toml_str).expect("Failed to parse peripherals config TOML");

    let template = PeripheralInitializersTemplate {
        peripherals: config.peripherals,
    };
    template
        .render()
        .expect("Failed to render peripherals template")
}
