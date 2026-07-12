//! Standalone interactive hardware bringup serial console shell.
//!
//! Provides a real-time command interface over UART0 for sending one-way commands
//! to controllers (fountain, thermal, power) using the embedded-cli parser.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]
#![allow(static_mut_refs)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
use cat_detector as app;

#[cfg(all(target_arch = "arm", target_os = "none"))]
use {embassy_executor::Spawner, embedded_cli::cli::CliBuilder};

#[cfg(all(target_arch = "arm", target_os = "none"))]
use controller::shell_controller::{ShellController, ShellControllerPointers};

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    app::handle_panic_with_sizes::<
        { app::FLASH_SIZE },
        { app::STACK_TOP },
        { app::FLASH_START },
        { app::FLASH_END },
        { app::FLASH_WRITE_SIZE },
        { app::FLASH_ERASE_SIZE },
    >(info);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
use core::fmt::Write as FmtWrite;

#[cfg(all(target_arch = "arm", target_os = "none"))]
controller::declare_shell_commands! {
    CatDetectorCli (CatDetectorCliProcessor) {
        Battery,
        Thermal,
        Motor,
        Sensor,
        Fs,
        System,
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
type I2cBus =
    embassy_rp::i2c::I2c<'static, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Blocking>;

#[cfg(all(target_arch = "arm", target_os = "none"))]
type MotorDevice =
    peripherals::l9110s::L9110s<embassy_rp::gpio::Flex<'static>, embassy_rp::gpio::Flex<'static>>;

#[cfg(all(target_arch = "arm", target_os = "none"))]
type FlashDevice = embassy_rp::flash::Flash<
    'static,
    embassy_rp::peripherals::FLASH,
    embassy_rp::flash::Blocking,
    { app::FLASH_SIZE },
>;

/// Main application entry point for the bringup shell.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let _ = spawner;
    let p = embassy_rp::init(Default::default());

    // Configure hardware stack guard using Cortex-M MPU
    app::configure_mpu_stack_guard();

    // Initialize board peripherals using the unified board configuration
    let board = app::Board::init(p);

    let writer = firmware_lib::rtt::RttTxWriter;

    let mut cli: embedded_cli::cli::Cli<
        firmware_lib::rtt::RttTxWriter,
        core::convert::Infallible,
        _,
        _,
    > = CliBuilder::default()
        .writer(writer)
        .prompt("\r\nshell> ")
        .build()
        .map_err(|_| ())
        .unwrap();

    // Print welcome text using the CLI's internal writer
    let banner = r#"
       |\      _,,,---,,_
 Zzz   /,`.-'`'    -.  ;-;;,_
      |,4-  ) )-,_. ,\ (  `'-'
     '---''(_/--'  `-'\_)  
"#;
    let _ = cli.write(|writer| {
        let _ = core::writeln!(writer, "{}", banner);
        let _ = core::writeln!(writer, "Type 'help' to print usage.");
        Ok(())
    });

    // Declare statically to avoid stack allocation and stack overflow
    static mut FS_BUF: [u8; 4096] = [0u8; 4096];

    // Initialize board peripherals and subcontrollers
    app::init_controllers(board).await;

    // Initialize the modular panic handler
    let panic_flash = unsafe { app::PANIC_FLASH.as_mut().unwrap() };
    let fs_buf = unsafe { &mut FS_BUF };
    app::init_panic_handler(
        panic_flash,
        app::STORAGE_PARTITION_START..app::STORAGE_PARTITION_END,
        fs_buf,
    );

    let temp_sensor_ptr = {
        let mut guard = app::SHARED_TEMP_SENSOR.lock().await;
        if let Some(ref mut sensor) = guard.0 {
            sensor as *mut cat_detector::Rp2040TempSensor
        } else {
            core::ptr::null_mut()
        }
    };

    let thermals = unsafe {
        &[controller::NamedDevice {
            name: "default",
            device: app::THERMAL_CTRL.as_mut().unwrap() as *mut _,
        }]
    };

    let batteries = unsafe {
        &[controller::NamedDevice {
            name: "default",
            device: app::BATTERY_CTRL.as_mut().unwrap() as *mut _,
        }]
    };

    let board_i2c_ptr = app::SHARED_I2C.lock(|cell| {
        let mut borrow = cell.borrow_mut();
        if let Some(ref mut i2c) = borrow.0 {
            i2c as *mut _ as *mut _
        } else {
            core::ptr::null_mut()
        }
    });

    let board_motor_ptr = unsafe { &mut app::MOTOR_CTRL.as_mut().unwrap().motor as *mut _ };

    let i2c_buses = &[controller::NamedDevice {
        name: "default",
        device: board_i2c_ptr,
    }];
    let motors = &[controller::NamedDevice {
        name: "default",
        device: board_motor_ptr,
    }];
    let flash_partitions = unsafe {
        &[controller::NamedPartition {
            name: "default",
            partition: controller::FlashPartition {
                flash_ptr: app::PANIC_FLASH.as_mut().unwrap() as *mut _,
                start_address: app::STORAGE_PARTITION_START,
                end_address: app::STORAGE_PARTITION_END,
            },
        }]
    };
    let temp_sensors: &[controller::NamedDevice<_>] = if !temp_sensor_ptr.is_null() {
        &[controller::NamedDevice {
            name: "default",
            device: temp_sensor_ptr,
        }]
    } else {
        &[]
    };
    let sensors = unsafe {
        &[
            controller::NamedDevice {
                name: "north",
                device: app::SENSOR_CTRL_NORTH.as_mut().unwrap() as *mut _,
            },
            controller::NamedDevice {
                name: "east",
                device: app::SENSOR_CTRL_EAST.as_mut().unwrap() as *mut _,
            },
            controller::NamedDevice {
                name: "west",
                device: app::SENSOR_CTRL_WEST.as_mut().unwrap() as *mut _,
            },
        ]
    };

    let motor_ctrls = unsafe {
        &[controller::NamedDevice {
            name: "default",
            device: app::MOTOR_CTRL.as_mut().unwrap() as *mut _,
        }]
    };

    #[cfg(all(target_arch = "arm", target_os = "none"))]
    type ThermalControllerType = controller::thermal_controller::ThermalController<
        'static,
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        app::TempSensorDevice,
        app::SystemCommand,
    >;

    #[cfg(all(target_arch = "arm", target_os = "none"))]
    type BatteryControllerType = controller::battery_controller::BatteryController<
        'static,
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        app::BatteryDevice,
        app::ChargerDevice,
        app::AlertPinType,
        app::SystemCommand,
    >;

    #[cfg(all(target_arch = "arm", target_os = "none"))]
    type SensorControllerType = controller::sensor_controller::SensorController<
        'static,
        app::ProximitySensorDevice,
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        app::DataReadyPinType,
        app::SystemCommand,
        controller::sensor_controller::ProximityReader,
    >;

    #[cfg(all(target_arch = "arm", target_os = "none"))]
    type MotorControllerType =
        controller::motor_controller::MotorController<MotorDevice, app::CurrentSensorDevice>;

    struct AppConfig;
    controller::impl_shell_config! {
        AppConfig {
            MutexRaw: embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            I2c = I2cBus,
            Motor = MotorDevice,
            Flash = FlashDevice,
            TempSensor = cat_detector::Rp2040TempSensor,
            ThermalCtrl = ThermalControllerType,
            BatteryCtrl = BatteryControllerType,
            SensorCtrl = SensorControllerType,
            MotorCtrl = MotorControllerType,
        }
    }

    let pointers = ShellControllerPointers::<AppConfig> {
        i2c_buses,
        motors,
        flash_partitions,
        temp_sensors,
        sensors,
        motor_ctrls,
        thermals,
        batteries,
        fs_buffer: unsafe { &mut FS_BUF },
        ..Default::default()
    };

    let mut processor = ShellController::<AppConfig>::new(pointers);

    // Run the main input loop feeding bytes to the embedded-cli processor over RTT
    let mut local_proc = CatDetectorCliProcessor::new(&mut processor);
    firmware_lib::rtt::run_rtt_shell_loop::<CatDetectorCli, _, _, _>(&mut cli, &mut local_proc);
}

/// Dummy host entry point to satisfy Cargo compilation requirements.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
