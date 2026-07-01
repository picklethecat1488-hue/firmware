//! Sensor controller for the Time-of-Flight (ToF) proximity sensors.

#![deny(missing_docs)]

use model::interfaces::ProximitySensor;

/// One-way commands sent to the Sensor Controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorCommand {
    /// Force proximity sensor check and print telemetry logs
    ReadSensors,
    /// Enable periodic automatic readings
    EnablePeriodic,
    /// Disable periodic automatic readings (runs only via manual commands)
    DisablePeriodic,
}

/// A controller that coordinates readings from a single proximity (ToF) sensor.
pub struct SensorController<
    'a,
    S,
    M: embassy_sync::blocking_mutex::raw::RawMutex = embassy_sync::blocking_mutex::raw::NoopRawMutex,
    Cmd = (),
> {
    sensor_id: u8,
    sensor: S,
    latest_distance: u16,
    periodic_enabled: bool,
    system_tx: Option<embassy_sync::channel::Sender<'a, M, Cmd, 4>>,
    make_cmd: Option<fn(u8, u16) -> Cmd>,
}

impl<'a, S: ProximitySensor>
    SensorController<'a, S, embassy_sync::blocking_mutex::raw::NoopRawMutex, ()>
{
    /// Creates a new SensorController managing a single proximity sensor.
    pub const fn new(sensor_id: u8, sensor: S) -> Self {
        Self {
            sensor_id,
            sensor,
            latest_distance: 1000,
            periodic_enabled: true,
            system_tx: None,
            make_cmd: None,
        }
    }
}

impl<
        'a,
        S: ProximitySensor,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Cmd: Clone + core::fmt::Debug,
    > SensorController<'a, S, M, Cmd>
{
    /// Creates a new SensorController with upstream system notification.
    pub fn new_with_fusion(
        sensor_id: u8,
        sensor: S,
        system_tx: embassy_sync::channel::Sender<'a, M, Cmd, 4>,
        make_cmd: fn(u8, u16) -> Cmd,
    ) -> Self {
        Self {
            sensor_id,
            sensor,
            latest_distance: 1000,
            periodic_enabled: true,
            system_tx: Some(system_tx),
            make_cmd: Some(make_cmd),
        }
    }

    /// Gets the current proximity telemetry reading.
    pub fn telemetry(&self) -> model::types::ProximityTelemetry {
        if self.latest_distance < 300 {
            model::types::ProximityTelemetry::InRange(self.latest_distance)
        } else {
            model::types::ProximityTelemetry::OutRange(self.latest_distance)
        }
    }

    /// Gets the latest read proximity telemetry distance.
    pub fn latest_distance(&self) -> u16 {
        self.latest_distance
    }

    /// Gets the sensor ID.
    pub fn sensor_id(&self) -> u8 {
        self.sensor_id
    }

    /// Gets whether periodic monitoring is enabled.
    pub fn is_periodic_enabled(&self) -> bool {
        self.periodic_enabled
    }

    /// Ticks the sensor control loop, updating proximity distance.
    pub fn update(&mut self) -> Result<u16, S::Error> {
        let dist = self.sensor.read_distance_mm()?;
        self.latest_distance = dist;

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::info!(
            "Sensor Controller (ID={}): distance={} mm",
            self.sensor_id,
            dist
        );

        if let (Some(tx), Some(make_cmd)) = (&self.system_tx, &self.make_cmd) {
            let cmd = make_cmd(self.sensor_id, dist);
            tx.try_send(cmd).unwrap();
        }

        Ok(dist)
    }

    /// Handles a SensorCommand.
    pub fn handle_command(&mut self, cmd: SensorCommand) {
        match cmd {
            SensorCommand::ReadSensors => {
                let _ = self.update();
            }
            SensorCommand::EnablePeriodic => {
                self.periodic_enabled = true;
            }
            SensorCommand::DisablePeriodic => {
                self.periodic_enabled = false;
            }
        }
    }

    /// Runs the controller's main run loop, executing periodic telemetry updates.
    pub async fn run(
        mut self,
        command_rx: embassy_sync::channel::Receiver<'static, M, SensorCommand, 4>,
    ) -> ! {
        loop {
            match embassy_time::with_timeout(
                embassy_time::Duration::from_millis(1000),
                command_rx.receive(),
            )
            .await
            {
                Ok(cmd) => {
                    self.handle_command(cmd);
                }
                Err(_timeout) => {
                    if self.periodic_enabled {
                        let _ = self.update();
                    }
                }
            }
        }
    }
}
