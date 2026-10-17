# 🟢 `[WORM-TUNNELING-INCHWORM-510]` FS doesn’t work

- **UUID**: `392a47ce-f137-404d-94cb-3f7af58334bc`
- **ID**: `WORM-TUNNELING-INCHWORM-510`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `host_daemon`
- **Created**: `2026-10-17 01:34:30 UTC`
- **Resolved**: `2026-10-17 01:54:29 UTC`

#### Description

The FS workstation does not work and the example command usage is wrong (see screenshot below), when opening the FS view, we should wait until the host_cli process is terminated and probe is available, and vice versa for opening the host_fs view. Additionally, host_fs and host_cli views should have entirely separate command workstations; right now, CLI output is interleaved with FS output (see traceback below)

#### Execution / Console Logs

```text
Finished 8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles`release` profile [optimized + debuginfo]8;; target(s) in 0.07s
     Running `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/release/cat_detector_shell --dump`
    Finished 8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles`release` profile [optimized + debuginfo]8;; target(s) in 0.11s
     Finished in 7.45s
RTT connected. Active channels:
  - Up Channel: "defmt" (logs)
  - Up Channel: "cli" (raw output)
  - Down Channel: "cli" (raw input)
shell> 
       |\      _,,,---,,_
 Zzz   /,`.-'`'    -.  ;-;;,_
      |,4-  ) )-,_. ,\ (  `'-'
     '---''(_/--'  `-'\_)  
Type 'help' to print usage.
shell> Draining buffered defmt logs:
0.001471 INFO MPU stack guard configured at 0x2003c000
0.021860 INFO MPU stack guard configured at 0x2000c500
Error: unexpected EOF during frame decoding (possible target/ELF mismatch or truncated log buffer)
RTT connected. Active channels:
  - Up Channel: "defmt" (logs)
  - Up Channel: "cli" (raw output)
  - Down Channel: "cli" (raw input)
shell> 
       |\      _,,,---,,_
 Zzz   /,`.-'`'    -.  ;-;;,_
      |,4-  ) )-,_. ,\ (  `'-'
     '---''(_/--'  `-'\_)  
Type 'help' to print usage.
shell> 
RTT Session Started (Press Ctrl+C to exit):
Interactive RTT console active. Type commands and press Enter.
shell> help
help
Commands:
  battery  Battery commands (battery status, battery test-alert)
  thermal  Thermal commands (thermal status, thermal mcu)
  motor    Motor commands (motor speed, motor rpm-speed, motor stop, motor current, motor rpm, motor calibrate, motor safety-bypass)
  sensor   Sensor commands (sensor status, sensor cal_near, sensor cal_far, sensor cal_xtalk)
  fs       Filesystem commands (fs format, fs ls)
  system   System commands (system activity)
  i2c      I2C commands (i2c scan)
  gpio     GPIO commands (gpio status, gpio read, gpio set)
  led      LED commands (led off, led green, led blue, led yellow, led orange, led blink-four, led blink-slow, led brightness)
  core     Core monitor commands (core status, core crash)
shell> fs ls
Listing directory...
No files found (directory empty).
shell> 
    Finished 8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles`release` profile [optimized + debuginfo]8;; target(s) in 0.09s
     Running `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/release/cat_detector_shell --dump`
    Finished 8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles`release` profile [optimized + debuginfo]8;; target(s) in 0.12s
     Finished in 7.45s
RTT connected. Active channels:
  - Up Channel: "defmt" (logs)
  - Up Channel: "cli" (raw output)
  - Down Channel: "cli" (raw input)
shell> 
       |\      _,,,---,,_
 Zzz   /,`.-'`'    -.  ;-;;,_
      |,4-  ) )-,_. ,\ (  `'-'
     '---''(_/--'  `-'\_)  
Type 'help' to print usage.
shell> Draining buffered defmt logs:
0.001458 INFO MPU stack guard configured at 0x2003c000
0.021853 INFO MPU stack guard configured at 0x2000c500
Error: unexpected EOF during frame decoding (possible target/ELF mismatch or truncated log buffer)
RTT connected. Active channels:
  - Up Channel: "defmt" (logs)
  - Up Channel: "cli" (raw output)
  - Down Channel: "cli" (raw input)
shell> 
       |\      _,,,---,,_
 Zzz   /,`.-'`'    -.  ;-;;,_
      |,4-  ) )-,_. ,\ (  `'-'
     '---''(_/--'  `-'\_)  
Type 'help' to print usage.
shell> 
RTT Session Started (Press Ctrl+C to exit):
Interactive RTT console active. Type commands and press Enter.
fs> help 
Host utility for querying sequential-storage filesystem dumps from RP2040
Usage: host_fs [OPTIONS] <COMMAND>
Commands:
  ls                List all files in the partition along with their data types
  cat               Print file contents
  export-telemetry  Export telemetry.rrd to a CSV file for plotting or visualization (e.g. in Rerun)
  crash-log         Read and decode all stored crash dumps into human-readable backtraces
  cp                Copy a file to or from the device flash partition
  rm                Remove a file or multiple files from the partition (defaults to '*' which clears all files)
  format            Erase and format the sequential-storage partition on the device
  help              Print this message or the help of the given subcommand(s)
Options:
  -d, --dump <DUMP>                  Path to the binary flash dump file (e.g. flash_dump.bin). If not specified, direct attachment to the device via probe-rs is used
  -e, --elf <ELF>                    Path to the ELF binary containing project metadata and symbols
      --offset <OFFSET>              Force a specific flash partition offset in bytes (e.g. "0x1C0000" or "1835008")
      --size <SIZE>                  Force a specific flash partition size in bytes (e.g. 262144)
  -o, --openocd-host <OPENOCD_HOST>  Connect to an existing OpenOCD GDB server session (e.g. "localhost:3333" or "127.0.0.1")
  -h, --help                         Print help
  -V, --version                      Print version
fs> ls 
Error: Custom { kind: Other, error: "Failed to open probe: Usb(Custom { kind: Other, error: \"error while taking control over USB device: could not open interface for exclusive access (error 0xe00002c5)\" })" }
fs>
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `screenshot` | [screenshot_paste_1792200935540_x027h.png](attachments/WORM-TUNNELING-INCHWORM-510/screenshot_paste_1792200935540_x027h.png) | Pasted screenshot screenshot_paste_1792200935540_x027h.png |

#### Resolution Notes

Guaranteed probe exclusivity by stopping RTT streaming before FS commands and on switching to FS mode, separated CLI and FS output queues and terminal DOM views, and updated command placeholders.
