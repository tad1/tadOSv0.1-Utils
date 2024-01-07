__rpi_phys_dram_start_addr = 0;
__rpi_phys_binary_load_addr = 0x200000;

ENTRY(__rpi_phys_binary_load_addr)

PHDRS{
    segment_boot_core_stack PT_LOAD FLAGS(6);
    segment_code            PT_LOAD FLAGS(5);
    segment_data            PT_LOAD FLAGS(6);
}

SECTIONS{
    . = __rpi_phys_dram_start_addr;
    . += __rpi_phys_binary_load_addr;

    .text : {
        KEEP(*(.text.main))
        *(.text*)
    } : segment_code

    .rodata : ALIGN(8) { *(.rodata*) } :segment_code

    .data : { *(.data*) } :segment_data

    .bss (NOLOAD) : ALIGN(16){
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        __bss_end_exclusive = .;
    } :segment_data

    .got : { *(.got*) }
    ASSERT(SIZEOF(.got) == 0, "Relocation support not expected")

    /DISCARD/ : { *(.comment*) }

}