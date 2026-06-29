MEMORY
{
    /* The second-stage bootloader (first 256 bytes of flash) */
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    
    /* The main application code area. 1.75MB flash (leaving 256KB at the end for storage) */
    FLASH : ORIGIN = 0x10000100, LENGTH = 1792K - 0x100
    
    /* Total available RAM for RP2040 */
    RAM   : ORIGIN = 0x20000000, LENGTH = 264K
}

_storage_start = 0x1C0000;
_storage_end = 0x200000;
