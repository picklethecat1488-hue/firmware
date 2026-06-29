//! Cat Detector target application for Raspberry Pi Pico.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod target {
    use controller::fountain_controller::FountainController;
    use embassy_executor::Spawner;
    use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    use embassy_sync::mutex::Mutex;
    use peripherals::mock::{DummyInputPin, DummyWaterSensor, MockBattery};
    use peripherals::pump::GpioPump;
    use {defmt_rtt as _, panic_probe as _};

    // Define raw statically allocated Mutex for thread-safe/multi-core peripheral sharing
    static SHARED_BATTERY: Mutex<CriticalSectionRawMutex, MockBattery> =
        Mutex::new(MockBattery::new(3700, 25000));

    use controller::battery_controller::BatteryController;
    use controller::thermal_controller::ThermalController;

    /// Main application entry point for the Cat Detector.
    #[embassy_executor::main]
    async fn main(spawner: Spawner) {
        defmt::info!("Initializing hardware for cat detector...");
        let p = embassy_rp::init(Default::default());

        // Initialize board peripherals using the unified board configuration
        let mut board = cat_detector::Board::init(p);

        // Extract the pump control pin from the board configuration array
        let pump_pin = board.gpio_pins[cat_detector::LED_PIN as usize]
            .take()
            .expect("Pump pin must be available");

        let pump = GpioPump::new(pump_pin);
        let sensor = DummyWaterSensor { pin: DummyInputPin };

        let controller = FountainController::new(pump, sensor);

        let thermal_ctrl = ThermalController::new(&SHARED_BATTERY);
        let power_ctrl = BatteryController::new(&SHARED_BATTERY);

        // Spawn controllers selectively and concurrently using separate macros
        controller::run_thermal_task!(
            spawner,
            thermal_task,
            thermal_ctrl,
            cat_detector::THERMAL_CHANNEL.receiver(),
            MockBattery
        );

        controller::run_battery_task!(
            spawner,
            power_task,
            power_ctrl,
            cat_detector::BATTERY_CHANNEL.receiver(),
            MockBattery
        );

        controller::run_fountain_task!(
            spawner,
            fountain_task,
            controller,
            cat_detector::FOUNTAIN_CHANNEL.receiver(),
            GpioPump<embassy_rp::gpio::Flex<'static>>,
            DummyWaterSensor
        );
    }
}

/// Host main entry point for testing and compilation verification.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
