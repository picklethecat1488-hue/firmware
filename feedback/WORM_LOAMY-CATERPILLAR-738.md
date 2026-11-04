# 🟢 `[WORM-LOAMY-CATERPILLAR-738]` Add motor telemetry

- **UUID**: `55993acb-1f3a-4436-a773-64b924983be3`
- **ID**: `WORM-LOAMY-CATERPILLAR-738`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_detector`
- **Created**: `2026-10-23 00:07:50 UTC`
- **Resolved**: `2026-10-23 00:13:54 UTC`

#### Description

Can you please add the motor telemetry we had discussed:

    #[derive(Copy, Clone, Debug, PartialEq, Eq, defmt::Format)]
    #[cfg_attr(feature = "cbor", derive(minicbor::Encode, minicbor::Decode))]
    #[cfg_attr(feature = "cbor", cbor(array))]
    pub struct MotorTelemetry {
        #[cfg_attr(feature = "cbor", n(0))]
        pub speed: MotorSpeed,
        #[cfg_attr(feature = "cbor", n(1))]
        pub measured_rpm: u32,
        #[cfg_attr(feature = "cbor", n(2))]
        pub estimated_rpm: u32,
        #[cfg_attr(feature = "cbor", n(3))]
        pub current_ma: i32,
        #[cfg_attr(feature = "cbor", n(4))]
        pub modeled_peak_current_ma: i32,
        #[cfg_attr(feature = "cbor", n(5))]
        pub modeled_torque_unm: u32,
    }

#### Resolution Notes

Added MotorTelemetry struct in model::types with speed, measured_rpm, estimated_rpm, current_ma, modeled_peak_current_ma, and modeled_torque_unm. Integrated into MotorStatus::Running, updated downstream telemetry pipelines, and added regression unit tests in model/tests/telemetry_tests.rs and tools/tests/test_worm_loamy_caterpillar_738_motor_telemetry.py.
