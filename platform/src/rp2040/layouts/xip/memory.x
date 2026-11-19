MEMORY
{
    /* The second-stage bootloader (first 256 bytes of flash) */
    BOOT2            : ORIGIN = 0x10000000, LENGTH = 0x100

    /* Main application XIP area (leaves 4KB for .program_metadata + 320KB for metadata/fs/telemetry) */
    FLASH            : ORIGIN = 0x10000100, LENGTH = 1724K - 0x100

    /* Fixed-address immutable ProgramMetadata / ProjectMetadata descriptor sector (4KB) */
    PROGRAM_METADATA : ORIGIN = 0x101AF000, LENGTH = 4K

    /* Total available RAM for RP2040 */
    RAM              : ORIGIN = 0x20000000, LENGTH = 264K
}
