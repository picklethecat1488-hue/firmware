MEMORY
{
    /* Bootloader resides at 0x00000000..0x00010000 (64KB) */
    /* Main application NOR flash (leaves 4KB for .program_metadata + 128KB for metadata/keystore) */
    FLASH : ORIGIN = 0x00010000, LENGTH = 1852K
    /* Dedicated 4KB immutable .program_metadata descriptor sector for O(1) host_fs board discovery */
    PROGRAM_METADATA : ORIGIN = 0x001DF000, LENGTH = 4K
    /* 512KB internal ECC SRAM */
    RAM   : ORIGIN = 0x20000000, LENGTH = 512K
}
