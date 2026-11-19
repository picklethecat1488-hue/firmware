//! Carrier Board 2.0 Hardware Support Package & Bringup Library.
//!
//! Provides hardware constants, pinout mappings, memory configurations,
//! controller orchestration, and interactive bringup shell configuration for the
//! Carrier Board 2.0 architecture powered by the NXP MCX N947 dual-core microcontroller.

#![deny(missing_docs)]

pub use board::carrier_board::{
    get_boot_reason, AlertPinType, BatteryDevice, Board, BoardPeripherals, ChargerDevice,
    FlashDevice, LedDevice, McxTempSensor, MutexRaw, TempSensorDevice,
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use board::carrier_board::handle_panic;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global `OnceLock` static holding the initialized Carrier Board 2.0 BSP.
pub static BOARD: platform::OnceLock<Board<'static>> = platform::OnceLock::new();

/// System command channel for Carrier Board 2.0.
pub static SYSTEM_CHANNEL: controller::SystemChannel<MutexRaw, 16> =
    controller::SystemChannel::new();
/// Thermal controller command channel for Carrier Board 2.0.
pub static THERMAL_CHANNEL: controller::ThermalChannel<MutexRaw, 4> =
    controller::ThermalChannel::new();
/// Thermal update action channel for Carrier Board 2.0.
pub static THERMAL_ACTION_CHANNEL: embassy_sync::channel::Channel<
    MutexRaw,
    controller::types::ThermalUpdateAction,
    4,
> = embassy_sync::channel::Channel::new();
/// Battery controller command channel for Carrier Board 2.0.
pub static BATTERY_CHANNEL: controller::BatteryChannel<MutexRaw, 4> =
    controller::BatteryChannel::new();
/// LED controller command channel for Carrier Board 2.0.
pub static LED_CHANNEL: controller::LedChannel<MutexRaw, 4> = controller::LedChannel::new();
/// Telemetry event channel for Carrier Board 2.0.
pub static TELEMETRY_CHANNEL: controller::TelemetryChannel<
    MutexRaw,
    { controller::telemetry_controller::CHANNEL_CAPACITY },
> = controller::TelemetryChannel::new();

/// Feature set implementing [`controller::SystemFeatureSet`] for Carrier Board 2.0.
#[allow(clippy::type_complexity)]
pub struct CarrierBoardFeatureSet {
    /// Active system feature tuple (Battery, LED, Thermal).
    pub features: (
        controller::BatteryFeatureConfig<MutexRaw>,
        controller::LedFeatureConfig<MutexRaw>,
        controller::ThermalFeatureConfig<MutexRaw>,
    ),
}

impl controller::SystemFeatureSet<MutexRaw, 3> for CarrierBoardFeatureSet {
    type Features = (
        controller::BatteryFeatureConfig<MutexRaw>,
        controller::LedFeatureConfig<MutexRaw>,
        controller::ThermalFeatureConfig<MutexRaw>,
    );

    fn features(&self) -> &Self::Features {
        &self.features
    }

    fn inactivity_timeout_seconds(&self) -> u32 {
        30
    }
}

/// Create the default [`CarrierBoardFeatureSet`] bound to Carrier Board channels.
pub fn create_default_feature_set() -> CarrierBoardFeatureSet {
    let mut thermal_feat = controller::ThermalFeatureConfig::new(Some(THERMAL_CHANNEL.sender()));
    thermal_feat.overheating_temp_milli_c = 70_000;
    thermal_feat.critical_temp_milli_c = 85_000;
    CarrierBoardFeatureSet {
        features: (
            controller::BatteryFeatureConfig::new(
                Some(BATTERY_CHANNEL.sender()),
                platform::BatteryManager::new(10, 5, 20, 50, 80),
            ),
            controller::LedFeatureConfig::new(Some(LED_CHANNEL.sender()), 50),
            thermal_feat,
        ),
    }
}

/// Thermal controller type alias for Carrier Board 2.0.
pub type ThermalControllerType =
    controller::thermal_controller::ThermalController<'static, MutexRaw, TempSensorDevice>;
/// Battery controller type alias for Carrier Board 2.0.
pub type BatteryControllerType = controller::battery_controller::BatteryController<
    'static,
    MutexRaw,
    BatteryDevice,
    ChargerDevice,
    AlertPinType,
>;
/// LED controller type alias for Carrier Board 2.0.
pub type LedControllerType = controller::led_controller::LedController<LedDevice>;
/// System controller type alias for Carrier Board 2.0.
pub type SystemControllerType = controller::SystemController<MutexRaw, CarrierBoardFeatureSet, 3>;

/// Collection of Core 0 controllers for Carrier Board 2.0.
pub struct Core0Controllers {
    /// On-die and ambient thermal controller.
    pub thermal: ThermalControllerType,
    /// Fuel gauge and charger battery controller.
    pub battery: BatteryControllerType,
    /// LP5009 RGB LED controller.
    pub led: LedControllerType,
    /// Top-level system lifecycle controller.
    pub system: SystemControllerType,
}

/// Combined controller topology for Carrier Board 2.0.
pub struct Controllers {
    /// Core 0 controller set.
    pub core0: Core0Controllers,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Initialize Carrier Board 2.0 subcontrollers from the initialized BSP and peripherals.
pub async fn init_controllers(
    board: &'static Board<'static>,
    peripherals: BoardPeripherals,
) -> Controllers {
    let thermal = controller::thermal_controller::ThermalController::new_with_shutdown_and_trap(
        &board.temp_sensor,
        THERMAL_ACTION_CHANNEL.sender(),
    );
    let battery = controller::battery_controller::BatteryController::new_with_system_and_alert(
        &board.battery,
        &board.charger,
        SYSTEM_CHANNEL.sender(),
        peripherals.fuel_gauge_alert_pin,
    );
    let led = controller::led_controller::LedController::new(peripherals.led_driver);
    let system = controller::SystemController::new(
        create_default_feature_set(),
        TELEMETRY_CHANNEL.sender(),
        get_boot_reason(),
    );

    Controllers {
        core0: Core0Controllers {
            thermal,
            battery,
            led,
            system,
        },
    }
}

/// Interactive bringup shell type configuration for Carrier Board 2.0.
pub struct CarrierBoardShellConfig;

controller::impl_shell_config! {
    CarrierBoardShellConfig {
        I2c = peripheral::mock::DummyI2c,
        Motor = peripheral::mock::DummyMotor,
        Flash = FlashDevice,
        TempSensor = McxTempSensor,
        ThermalCtrl = ThermalControllerType,
        BatteryCtrl = BatteryControllerType,
        SensorCtrl = crate::MockSensorController,
        MotorCtrl = controller::motor_controller::MotorChannelSender,
        SystemCtrl = SystemControllerType,
        LedCtrl = LedControllerType,
    }

    fn trigger_core_panic(
        _resolver: &controller::shell_controller::ShellController<'_, Self>,
        core_id: u32,
    ) -> Result<(), &'static str> {
        platform::core_monitor::trigger_core_panic(core_id)
    }
}

/// Configured layout and static buffer for Carrier Board filesystem partition diagnostics.
pub struct FilesystemStorageConfig {
    /// Partition range mapped for internal `sequential_storage::map` filesystem.
    pub partition: controller::MapFilesystem,
    /// Static mutable buffer for filesystem and panic log operations.
    pub buffer: &'static mut [u8],
}

/// Return the Carrier Board 2.0 filesystem storage configuration from `board.toml`.
pub fn get_filesystem_config() -> FilesystemStorageConfig {
    FilesystemStorageConfig {
        partition: controller::MapFilesystem(Board::FS_PARTITION_START..Board::FS_PARTITION_END),
        buffer: Board::fs_buf(),
    }
}

/// Bind interactive `ShellControllerPointers` for Carrier Board 2.0 bringup shell commands.
#[macro_export]
macro_rules! declare_carrier_board_shell_pointers {
    ($fs_buffer:expr, $controllers:expr, $board:expr, $pointers_name:ident) => {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        let temp_sensor_ptr = $board
            .temp_sensor
            .lock()
            .await
            .0
            .as_mut()
            .map(|sensor| sensor as *mut $crate::carrier_board::McxTempSensor)
            .unwrap_or(core::ptr::null_mut());

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        let mut flash_dev = board::carrier_board::CarrierFlashDevice;
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        let flash_ptr = &mut flash_dev as *mut $crate::carrier_board::FlashDevice;

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        let flash_partitions = [
            controller::NamedPartition {
                name: "metadata",
                partition: controller::FlashPartition {
                    flash_ptr,
                    start_address: $crate::carrier_board::Board::METADATA_PARTITION_START,
                    end_address: $crate::carrier_board::Board::METADATA_PARTITION_END,
                },
                kind: controller::PartitionKind::Map,
            },
            controller::NamedPartition {
                name: "panic",
                partition: controller::FlashPartition {
                    flash_ptr,
                    start_address: $crate::carrier_board::Board::FS_PARTITION_START,
                    end_address: $crate::carrier_board::Board::FS_PARTITION_END,
                },
                kind: controller::PartitionKind::Map,
            },
            controller::NamedPartition {
                name: "telemetry",
                partition: controller::FlashPartition {
                    flash_ptr,
                    start_address: $crate::carrier_board::Board::TELEMETRY_PARTITION_START,
                    end_address: $crate::carrier_board::Board::TELEMETRY_PARTITION_END,
                },
                kind: controller::PartitionKind::Queue,
            },
        ];

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        let thermals = [controller::NamedDevice {
            name: "default",
            device: &mut $controllers.core0.thermal as *mut _,
        }];
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        let batteries = [controller::NamedDevice {
            name: "default",
            device: &mut $controllers.core0.battery as *mut _,
        }];
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        let leds = [controller::NamedDevice {
            name: "default",
            device: &mut $controllers.core0.led as *mut _,
        }];
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        let system_ctrls = [controller::NamedDevice {
            name: "default",
            device: &mut $controllers.core0.system as *mut _,
        }];
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        let temp_sensors_array = [controller::NamedDevice {
            name: "cpu",
            device: temp_sensor_ptr,
        }];
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        let temp_sensors = if !temp_sensor_ptr.is_null() {
            &temp_sensors_array[..]
        } else {
            &[]
        };

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        let $pointers_name = controller::shell_controller::ShellControllerPointers::<
            $crate::carrier_board::CarrierBoardShellConfig,
        > {
            thermals: &thermals[..],
            batteries: &batteries[..],
            leds: &leds[..],
            system_ctrls: &system_ctrls[..],
            flash_partitions: &flash_partitions[..],
            temp_sensors,
            fs_buffer: $fs_buffer,
            ..Default::default()
        };

        #[cfg(not(all(target_arch = "arm", target_os = "none")))]
        let $pointers_name = controller::shell_controller::ShellControllerPointers::<
            $crate::carrier_board::CarrierBoardShellConfig,
        >::default();
    };
}

pub use declare_carrier_board_shell_pointers as declare_shell_pointers;

/// Canonical name of the carrier board hardware platform.
pub const BOARD_NAME: &str = "Carrier Board 2.0";

/// Hardware revision string.
pub const BOARD_REVISION: &str = "Rev 2.0";

/// Host microcontroller architecture identifier.
pub const MCU_ARCH: &str = "NXP MCX N947 Dual-Core Arm Cortex-M33";

/// Internal dual-bank flash memory size in bytes (2 MB).
pub const FLASH_SIZE_BYTES: usize = 2 * 1024 * 1024;

/// Internal SRAM memory size with ECC in bytes (512 KB).
pub const SRAM_SIZE_BYTES: usize = 512 * 1024;

/// External serial SLC NAND flash memory size in bytes (1 Gb / 128 MB).
pub const EXT_NAND_SIZE_BYTES: usize = 128 * 1024 * 1024;

/// Core 0 application processor clock speed in Hertz (150 MHz).
pub const CORE0_CLOCK_HZ: u32 = 150_000_000;

/// Core 1 real-time coprocessor clock speed in Hertz (150 MHz).
pub const CORE1_CLOCK_HZ: u32 = 150_000_000;

/// 7-bit I2C address for the ADI MAX17048 fuel gauge IC on Core I2C0.
pub const I2C0_ADDR_MAX17048: u8 = 0x36;

/// Functional alias for the fuel gauge I2C address.
pub const I2C0_ADDR_FUEL_GAUGE: u8 = I2C0_ADDR_MAX17048;

/// 7-bit I2C address for the TI LP5009 9-channel RGB LED driver on Core I2C0.
pub const I2C0_ADDR_LP5009: u8 = 0x14;

/// 7-bit I2C address for the Azoteq IQS7222A capacitive touch controller on Touch I2C1.
pub const I2C1_ADDR_IQS7222A: u8 = 0x44;

/// Default bringup baud rate for FTDI USB-to-UART console port (FC1 UART0).
pub const CONSOLE_BAUD_RATE: u32 = 115_200;

/// High-speed operational baud rate for FTDI USB-to-UART port (FC1 UART0) and telemetry streaming (1 Mb/s).
pub const HIGH_SPEED_UART_BAUD_RATE: u32 = 1_000_000;

/// High-speed baud rate for u-blox NINA-B312 BLE module interface (1 Mb/s).
pub const BLE_UART_BAUD_RATE: u32 = 1_000_000;

/// Pin configuration mapping for Carrier Board 2.0 (NXP MCX N947 VFBGA-184).
///
/// Physical pinball definitions and signal names mapped according to Table 93.
pub mod pins {
    /// BQ24074 charge status active-low interrupt pin (Ball G5 / WUU0_IN15).
    pub const CHG_STAT: &str = "G5";

    /// BQ24074 power good wake pin (Ball M10 / VBAT_WAKEUP).
    pub const CHG_PGOOD: &str = "M10";

    /// TPS22918 audio amplifier power gate enable pin (Ball L4).
    pub const PWR_EN_AUDIO: &str = "L4";

    /// TPS22918 sensor subsystem power gate enable pin (Ball L5).
    pub const PWR_EN_SENSORS: &str = "L5";

    /// TPS22918 debug bridge power gate enable pin (Ball M4).
    pub const PWR_EN_DEBUG: &str = "M4";

    /// MAX17048 fuel gauge alert interrupt pin (Ball G4).
    pub const FUEL_ALERT: &str = "G4";

    /// IQS7222A capacitive touch data ready interrupt pin (Ball C4).
    pub const CAP_INT: &str = "C4";

    /// Piezo transducer PWM output pin (Ball D15 / PWM0_X0).
    pub const PIEZO_PWM: &str = "D15";
}

/// Static description container for carrier board hardware components.
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoardComponent {
    /// Reference designator (e.g. "U1", "U8").
    pub designator: &'static str,
    /// Manufacturer and component part number.
    pub part_number: &'static str,
    /// Functional role of the component in the system.
    pub description: &'static str,
    /// Primary bus or interface connection.
    pub interface: &'static str,
}

/// List of canonical components populated on the Carrier Board 2.0 assembly.
pub const CANONICAL_COMPONENTS: &[BoardComponent] = &[
    BoardComponent {
        designator: "U1",
        part_number: "NXP MCXN947VDF",
        description: "Dual-core Arm Cortex-M33 MCU with eIQ Neutron NPU",
        interface: "Host Controller",
    },
    BoardComponent {
        designator: "U8",
        part_number: "Winbond W25N01GVZEIG",
        description: "1Gb (128MB) SLC Serial NAND Flash Memory",
        interface: "FlexSPI Port A (Quad-SPI 100MHz)",
    },
    BoardComponent {
        designator: "U3",
        part_number: "TI BQ24074RGTR",
        description: "1.5A Li-Ion Battery Charger with Dynamic Power Path",
        interface: "GPIO (/CHG, /PGOOD)",
    },
    BoardComponent {
        designator: "U7",
        part_number: "ADI MAX17048G+T10",
        description: "1-Cell Li+ ModelGauge Fuel Gauge IC",
        interface: "Core I2C0 (0x36)",
    },
    BoardComponent {
        designator: "U6",
        part_number: "TI LP5009RUKR",
        description: "9-Channel Constant-Current Logarithmic RGB LED Driver",
        interface: "Core I2C0 (0x14)",
    },
    BoardComponent {
        designator: "U9",
        part_number: "FTDI FT232RNQ-REEL",
        description: "High-Speed USB 2.0 to UART Serial Bridge",
        interface: "FC1 UART0 (Console / 1Mb/s Telemetry)",
    },
    BoardComponent {
        designator: "U2",
        part_number: "Azoteq IQS7222A001QNR",
        description: "ProxFusion Capacitive Touch & Proximity Controller",
        interface: "Touch I2C1 (0x44)",
    },
    BoardComponent {
        designator: "U4",
        part_number: "ADI MAX98357AETE+",
        description: "3.2W Class-D Mono Audio Amplifier",
        interface: "PDM0 Audio",
    },
    BoardComponent {
        designator: "U11",
        part_number: "u-blox NINA-B312-02B",
        description: "Bluetooth Low Energy 5.0 Transceiver Module",
        interface: "High-Speed UART (1Mb/s)",
    },
];

/// Canonical Carrier Board 2.0 ASCII logo banner defined in the hardware project.
pub const CARRIER_BOARD_BANNER: &str = r#"
+------------------------------------------------------------------+
|                                                                  |
|                             /\                                   |
|                | \         /  \         / |                      |
|                |  \       / /\ \       /  |                      |
|                |   \     / /  \ \     /   |                      |
|                |    \   /_/ /\ \_\   /    |                      |
|                 \    \    / /\ \    /    /                       |
|                  \    \  /_/  \_\  /    /                        |
|                   \    \          /    /                         |
|                    \    \   /\   /    /                          |
|                     \    \_/  \_/    /                           |
|                      \              /                            |
|                       \____________/                             |
|                                                                  |
|                ANTIGRAVITY // CARRIER BOARD 2.0                  |
|                 NXP MCX N947 DUAL CORTEX-M33                     |
+------------------------------------------------------------------+
"#;

/// Returns the canonical Hello World bringup greeting for Carrier Board 2.0.
pub fn hello_world() -> &'static str {
    "Hello, World! Carrier Board 2.0 bringup shell initialized."
}

/// Formats and returns an overview banner string for host diagnostics.
pub fn get_bringup_banner() -> &'static str {
    CARRIER_BOARD_BANNER
}

/// Performs a diagnostic self-check verifying that all hardware configuration parameters are consistent.
pub fn run_bringup_self_test() -> bool {
    // Verify memory bounds
    if FLASH_SIZE_BYTES != 2 * 1024 * 1024 {
        return false;
    }
    if SRAM_SIZE_BYTES != 512 * 1024 {
        return false;
    }
    if EXT_NAND_SIZE_BYTES != 128 * 1024 * 1024 {
        return false;
    }

    // Verify I2C addresses are valid 7-bit identifiers
    if I2C0_ADDR_MAX17048 > 0x7F || I2C0_ADDR_LP5009 > 0x7F || I2C1_ADDR_IQS7222A > 0x7F {
        return false;
    }

    // Verify canonical components exist
    !CANONICAL_COMPONENTS.is_empty()
}
