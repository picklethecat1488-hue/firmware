SECTIONS
{
    .program_metadata : ALIGN(4)
    {
        KEEP(*(.program_metadata .program_metadata.*));
    } > PROGRAM_METADATA
}
