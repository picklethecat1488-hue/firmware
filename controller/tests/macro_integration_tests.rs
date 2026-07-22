use controller::battery_controller::{BatteryCommand, BatteryController};
use controller::led_controller::LedController;
use controller::motor_controller::{MotorCommand, MotorController};
use controller::sensor_controller::{SensorCommand, SensorController};
use controller::thermal_controller::{ThermalCommand, ThermalController};
use controller::{SystemCommand, SystemController};
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex};
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use model::interfaces::NoTick;

use model::types::{BootReason, ChargeState, Direction, SystemLedState, SystemStatus};
use peripherals::mock::{
    DummyCurrentSensor, MockBattery, MockCharger, MockLed, MockMotor, MockProximitySensor,
};

// 1. Mock wrappers for interrupt pins
struct MockPin;
impl controller::battery_controller::BatteryAlertPin for MockPin {
    async fn wait_for_alert(&mut self) {
        embassy_time::Timer::after_millis(10).await;
    }
}
impl controller::sensor_controller::DataReadyPin for MockPin {
    async fn wait_for_data_ready(&mut self) {
        embassy_time::Timer::after_millis(10).await;
    }
}

// 2. Mock Flash storage device
struct TestFlash {
    data: [u8; 1024 * 64],
}

impl TestFlash {
    fn new() -> Self {
        Self {
            data: [0xFF; 1024 * 64],
        }
    }
}

impl embedded_storage_async::nor_flash::ErrorType for TestFlash {
    type Error = core::convert::Infallible;
}

impl embedded_storage_async::nor_flash::ReadNorFlash for TestFlash {
    const READ_SIZE: usize = 1;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        bytes.copy_from_slice(&self.data[offset as usize..offset as usize + bytes.len()]);
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl embedded_storage_async::nor_flash::NorFlash for TestFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 4096;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.data[offset as usize..offset as usize + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        self.data[from as usize..to as usize].fill(0xFF);
        Ok(())
    }
}

impl embedded_storage_async::nor_flash::MultiwriteNorFlash for TestFlash {}

// 3. Dummy Feature Set for testing SystemController
#[allow(clippy::type_complexity)]
pub struct DummyFeatureSet<MutexRaw: RawMutex + 'static, const N: usize> {
    pub features: (
        controller::MotorFeatureConfig<MutexRaw, N>,
        controller::BatteryFeatureConfig<MutexRaw, N>,
        controller::ProximityFeatureConfig<MutexRaw, N>,
        controller::LedFeatureConfig<MutexRaw, N>,
        controller::ThermalFeatureConfig<MutexRaw, N>,
    ),
}

impl<MutexRaw: RawMutex + 'static, const N: usize> controller::SystemFeatureSet<MutexRaw, N>
    for DummyFeatureSet<MutexRaw, N>
{
    type Features = (
        controller::MotorFeatureConfig<MutexRaw, N>,
        controller::BatteryFeatureConfig<MutexRaw, N>,
        controller::ProximityFeatureConfig<MutexRaw, N>,
        controller::LedFeatureConfig<MutexRaw, N>,
        controller::ThermalFeatureConfig<MutexRaw, N>,
    );

    fn features(&self) -> &Self::Features {
        &self.features
    }

    fn inactivity_timeout_seconds(&self) -> u32 {
        30
    }
}

// Global channels for static integration test runs
static SYSTEM_CHANNEL: Channel<CriticalSectionRawMutex, SystemCommand, 4> = Channel::new();
static MOTOR_CHANNEL: Channel<CriticalSectionRawMutex, MotorCommand, 4> = Channel::new();
static SENSOR_NORTH_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
static SENSOR_EAST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
static SENSOR_WEST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
static BATTERY_CHANNEL: Channel<CriticalSectionRawMutex, BatteryCommand, 4> = Channel::new();
static THERMAL_CHANNEL: Channel<CriticalSectionRawMutex, ThermalCommand, 4> = Channel::new();
static LED_CHANNEL: Channel<CriticalSectionRawMutex, SystemLedState, 4> = Channel::new();
static TELEMETRY_CHANNEL: Channel<
    CriticalSectionRawMutex,
    model::telemetry::TelemetryRecord,
    { controller::telemetry_controller::CHANNEL_CAPACITY },
> = Channel::new();
static TELEMETRY_CONSUMER_CHANNEL: Channel<
    CriticalSectionRawMutex,
    model::telemetry::TelemetryRecord,
    { controller::telemetry_controller::CHANNEL_CAPACITY },
> = Channel::new();
static FILESYSTEM_CHANNEL: Channel<
    CriticalSectionRawMutex,
    controller::filesystem_controller::FsRequest,
    16,
> = Channel::new();
static GESTURE_CHANNEL: Channel<CriticalSectionRawMutex, model::types::Gesture, 4> = Channel::new();
static THERMAL_ACTION_CHANNEL: Channel<
    CriticalSectionRawMutex,
    controller::types::ThermalUpdateAction,
    4,
> = Channel::new();

#[embassy_executor::task]
async fn test_control_task_all() {
    embassy_time::Timer::after_millis(30).await;
    SYSTEM_CHANNEL
        .send(SystemCommand::BatteryUpdate {
            state_of_charge: 85,
            charger_state: ChargeState::DoneOrStandbyOrUnplugged,
        })
        .await;

    let mut success = false;
    let start = std::time::Instant::now();
    while start.elapsed() < std::time::Duration::from_secs(10) {
        while let Ok(record) = TELEMETRY_CHANNEL.try_receive() {
            if let model::telemetry::TelemetryRecord::System(SystemStatus::Active) = record {
                success = true;
            }
        }
        if success {
            break;
        }
        embassy_time::Timer::after_millis(5).await;
    }

    if success {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

#[test]
fn test_spawn_all_controllers_configuration() {
    let mock_motor = MockMotor::new();
    let mock_led = MockLed::new();
    let mock_tof_north = MockProximitySensor::new(1000);
    let mock_tof_east = MockProximitySensor::new(1000);
    let mock_tof_west = MockProximitySensor::new(1000);
    let mock_battery = Box::leak(Box::new(Mutex::new(MockBattery::new(3700, 25000))));
    let mock_charger = Box::leak(Box::new(Mutex::new(MockCharger::new(
        ChargeState::DoneOrStandbyOrUnplugged,
    ))));
    let mock_temp = Box::leak(Box::new(Mutex::new(MockBattery::new(3700, 25000))));

    let motor_ctrl = MotorController::new(NoTick::new(mock_motor), DummyCurrentSensor);
    let led_ctrl = LedController::new(mock_led);
    let battery_ctrl = BatteryController::new_with_system_and_alert(
        mock_battery,
        mock_charger,
        SYSTEM_CHANNEL.sender(),
        MockPin,
    );
    let thermal_ctrl =
        ThermalController::new_with_shutdown_and_trap(mock_temp, THERMAL_ACTION_CHANNEL.sender());
    let sensor_ctrl_north = SensorController::new_with_fusion_and_interrupt(
        controller::types::SensorMetadata {
            direction: Direction::North,
        },
        mock_tof_north,
        SYSTEM_CHANNEL.sender(),
        MockPin,
        300,
    );
    let sensor_ctrl_east = SensorController::new_with_fusion_and_interrupt(
        controller::types::SensorMetadata {
            direction: Direction::East,
        },
        mock_tof_east,
        SYSTEM_CHANNEL.sender(),
        MockPin,
        300,
    );
    let sensor_ctrl_west = SensorController::new_with_fusion_and_interrupt(
        controller::types::SensorMetadata {
            direction: Direction::West,
        },
        mock_tof_west,
        SYSTEM_CHANNEL.sender(),
        MockPin,
        300,
    );

    let feature_set = DummyFeatureSet {
        features: (
            controller::MotorFeatureConfig::new(
                Some(MOTOR_CHANNEL.sender()),
                model::types::MotorSpeed::MAX,
            ),
            controller::BatteryFeatureConfig::new(
                Some(BATTERY_CHANNEL.sender()),
                platform::BatteryManager::new(10, 2, 20, 21, 80),
            ),
            controller::ProximityFeatureConfig::new(
                &[
                    SENSOR_NORTH_CHANNEL.sender(),
                    SENSOR_EAST_CHANNEL.sender(),
                    SENSOR_WEST_CHANNEL.sender(),
                ],
                20,
                300,
                controller::GestureAction::TogglePower,
                Some(TELEMETRY_CHANNEL.sender()),
            ),
            controller::LedFeatureConfig::new(Some(LED_CHANNEL.sender())),
            controller::ThermalFeatureConfig::new(Some(THERMAL_CHANNEL.sender())),
        ),
    };
    let system_ctrl =
        SystemController::new(feature_set, TELEMETRY_CHANNEL.sender(), BootReason::Unknown);

    let fs_buf = Box::leak(vec![0u8; 4096].into_boxed_slice());
    let fs_controller = controller::filesystem_controller::FilesystemController::new(
        controller::filesystem_controller::ProfilingFlash::new(TestFlash::new()),
        0..1024 * 64,
        fs_buf,
    );

    let client =
        controller::filesystem_controller::FilesystemClient::new(FILESYSTEM_CHANNEL.sender());
    let telemetry_ctrl = Box::leak(Box::new(
        controller::telemetry_controller::TelemetryController::new(client),
    ));

    use embassy_executor::Executor;
    let executor = Box::leak(Box::new(Executor::new()));

    executor.run(|spawner: embassy_executor::Spawner| {
        controller::spawn_controllers! {
            spawner,
            telemetry: TELEMETRY_CHANNEL,
            controllers: {
                Thermal(thermal_ctrl, THERMAL_CHANNEL), generics: (peripherals::mock::MockBattery),
                Battery(battery_ctrl, BATTERY_CHANNEL), generics: (peripherals::mock::MockBattery, peripherals::mock::MockCharger, MockPin, SystemCommand),
                Motor(motor_ctrl, MOTOR_CHANNEL), generics: (model::interfaces::NoTick<MockMotor>, DummyCurrentSensor),
                Sensor(sensor_ctrl_north, SENSOR_NORTH_CHANNEL), generics: (MockProximitySensor, MockPin, SystemCommand),
                Sensor(sensor_ctrl_east, SENSOR_EAST_CHANNEL), generics: (MockProximitySensor, MockPin, SystemCommand),
                Sensor(sensor_ctrl_west, SENSOR_WEST_CHANNEL), generics: (MockProximitySensor, MockPin, SystemCommand),
                Led(led_ctrl, LED_CHANNEL), generics: (MockLed),
                System(system_ctrl, SYSTEM_CHANNEL, GESTURE_CHANNEL, THERMAL_ACTION_CHANNEL), generics: (controller::SystemController<CriticalSectionRawMutex, DummyFeatureSet<CriticalSectionRawMutex, 4>, 4, 64>),
                Filesystem(fs_controller, FILESYSTEM_CHANNEL), generics: (controller::filesystem_controller::ProfilingFlash<TestFlash>),
                Telemetry(telemetry_ctrl, TELEMETRY_CONSUMER_CHANNEL), generics: (1024, { controller::telemetry_controller::CHANNEL_CAPACITY }),
            }
        }

        spawner.spawn(test_control_task_all()).unwrap();
    });
}

#[embassy_executor::task]
async fn test_control_task_single() {
    embassy_time::Timer::after_millis(10).await;
    LED_CHANNEL.send(SystemLedState::SolidGreen).await;
    embassy_time::Timer::after_millis(10).await;
    std::process::exit(0);
}

#[test]
fn test_spawn_single_controller_configuration() {
    let mock_led = MockLed::new();
    let led_ctrl = LedController::new(mock_led);

    use embassy_executor::Executor;
    let executor = Box::leak(Box::new(Executor::new()));

    executor.run(|spawner: embassy_executor::Spawner| {
        // Test spawning only a single controller without explicit telemetry channel (it defaults to DUMMY_TELEMETRY_CHANNEL)
        controller::spawn_controllers! {
            spawner,
            controllers: {
                Led(led_ctrl, LED_CHANNEL), generics: (MockLed),
            }
        }

        spawner.spawn(test_control_task_single()).unwrap();
    });
}
