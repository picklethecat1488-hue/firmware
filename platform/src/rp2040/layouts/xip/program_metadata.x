Reset_Handler = Reset;

SECTIONS
{
    .program_metadata : ALIGN(4)
    {
        KEEP(*(.program_metadata .program_metadata.*));
    } > PROGRAM_METADATA

    .program_metadata_elf 0 (INFO) :
    {
        KEEP(*(.program_metadata_elf .program_metadata_elf.*));
    }
}
