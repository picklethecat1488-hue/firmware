# 🔴 `[WORM-COMPOSTED-CHRYSALIS-991]` Transient INA219 current spike during 50% BLDC pump spin-up

- **UUID**: `660b9e1f-5a08-404c-afe5-7c1b2f87af99`
- **ID**: `WORM-COMPOSTED-CHRYSALIS-991`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `CONTROLLER`
- **Component**: `target`
- **Created**: `2026-11-16 03:05:37 UTC`

#### Description

Observed brief 184 mA startup transient on TMC6300 phase engagement prior to steady-state Hall lock.

#### Execution / Console Logs

```text
Host daemon session initialized at 2026-11-16 03:05:37 UTC. No logs recorded yet.
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-COMPOSTED-CHRYSALIS-991.log](attachments/WORM-COMPOSTED-CHRYSALIS-991/target_system_logs_WORM-COMPOSTED-CHRYSALIS-991.log) | Target system and defmt logs from host daemon session |
| `log` | [target_crash_log_WORM-COMPOSTED-CHRYSALIS-991.log](attachments/WORM-COMPOSTED-CHRYSALIS-991/target_crash_log_WORM-COMPOSTED-CHRYSALIS-991.log) | Target crash log retrieved via host_fs |
| `reference` | [target_telemetry_WORM-COMPOSTED-CHRYSALIS-991.json](attachments/WORM-COMPOSTED-CHRYSALIS-991/target_telemetry_WORM-COMPOSTED-CHRYSALIS-991.json) | Target telemetry export retrieved via host_fs |
| `reference` | [target_metadata_WORM-COMPOSTED-CHRYSALIS-991.txt](attachments/WORM-COMPOSTED-CHRYSALIS-991/target_metadata_WORM-COMPOSTED-CHRYSALIS-991.txt) | Filesystem metadata retrieved via host_fs |
