MEMORY
{
    /* The second-stage bootloader (first 256 bytes of flash) */
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    
    /* The main application code area. 2MB total flash minus BOOT2 */
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    
    /* Total available RAM for RP2040 */
    RAM   : ORIGIN = 0x20000000, LENGTH = 264K
}
