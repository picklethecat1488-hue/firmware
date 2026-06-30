//! Sensor controller for the Time-of-Flight (ToF) proximity sensors.

#![deny(missing_docs)]

use model::interfaces::ProximitySensor;
use model::types::ProximityTelemetry;

/// A controller that coordinates readings from multiple proximity (ToF) sensors.
pub struct SensorController<N, E, W> {
    sensor_north: N,
    sensor_east: E,
    sensor_west: W,
    telemetry: ProximityTelemetry,
    periodic_enabled: bool,
}

impl<N: ProximitySensor, E: ProximitySensor, W: ProximitySensor> SensorController<N, E, W> {
    /// Creates a new SensorController managing North, East, and West proximity sensors.
    pub const fn new(sensor_north: N, sensor_east: E, sensor_west: W) -> Self {
        Self {
            sensor_north,
            sensor_east,
            sensor_west,
            telemetry: ProximityTelemetry::Triple(0, 0, 0),
            periodic_enabled: true,
        }
    }

    /// Gets the latest read proximity telemetry.
    pub fn telemetry(&self) -> ProximityTelemetry {
        self.telemetry
    }

    /// Gets whether periodic monitoring is enabled.
    pub fn is_periodic_enabled(&self) -> bool {
        self.periodic_enabled
    }

    /// Ticks the sensor control loop, updating proximity distances.
    #[allow(clippy::type_complexity)]
    pub fn update(&mut self) -> Result<(), SensorError<N::Error, E::Error, W::Error>> {
        let dist_north = self
            .sensor_north
            .read_distance_mm()
            .map_err(SensorError::North)?;

        let dist_east = self
            .sensor_east
            .read_distance_mm()
            .map_err(SensorError::East)?;

        let dist_west = self
            .sensor_west
            .read_distance_mm()
            .map_err(SensorError::West)?;

        self.telemetry = ProximityTelemetry::Triple(dist_north, dist_east, dist_west);

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::info!(
            "Sensor Controller: North={} mm, East={} mm, West={} mm",
            dist_north,
            dist_east,
            dist_west
        );

        Ok(())
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
    pub async fn run<MutexRaw: embassy_sync::blocking_mutex::raw::RawMutex, const SIZE: usize>(
        mut self,
        command_rx: embassy_sync::channel::Receiver<'static, MutexRaw, SensorCommand, SIZE>,
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

/// One-way commands sent to the Sensor Controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorCommand {
    /// Force proximity sensors check and print telemetry logs
    ReadSensors,
    /// Enable periodic automatic readings
    EnablePeriodic,
    /// Disable periodic automatic readings (runs only via manual commands)
    DisablePeriodic,
}

/// Errors returned by the sensor controller loop.
#[derive(Debug)]
pub enum SensorError<NE, EE, WE> {
    /// Error from the North proximity sensor.
    North(NE),
    /// Error from the East proximity sensor.
    East(EE),
    /// Error from the West proximity sensor.
    West(WE),
}

#[cfg(test)]
#[path = "sensor_controller_test.rs"]
mod tests;
