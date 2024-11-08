ENTRY(_start);

/* 0x2041_0000..0x2044_0000: Flash memory */
/* 0x2044_0000..0x2054_ffff: Secure SRAM */
/* 0x4000_0000..: RAM */

MEMORY
{
    FLASH (rx) : ORIGIN = 0x20410000, LENGTH = 0x40000
    DRAM (rwx) : ORIGIN = 0x20500000, LENGTH = 0x100000
}

SECTIONS
{
    .text :
    {
	KEEP(*(.text.boot));
	KEEP(*(.text._start_arguments));
        KEEP(*(.text .text.*));
    } > FLASH

    .rodata :
    {
        KEEP(*(.rodata .rodata.*));
    } > FLASH

    .data :
    {
	_data = .;
	KEEP(*(.data .data.*));
	_edata = .;
    } > DRAM AT > FLASH

    .bss (NOLOAD) :
    { 
	. = ALIGN(16);
	_bss = .;
	*(.sbss)
	*(.sbss.*)
	*(.bss)
	*(.bss.*)
	*(COMMON)
	. = ALIGN(16);
	_ebss = .;
    } > DRAM

    /* Stack and heap */
    .heap (NOLOAD) :
   {
         . = ALIGN(16);
        _heap = .;
        . += 64*1024;
        . = ALIGN(16);
        _eheap = .;
    } > DRAM

    .stack (NOLOAD) :
    {
        _estack = .;
        . += 64*1024;
        . = ALIGN(16);
        _stack = .;
    } > DRAM

    _end = .;

    .stack_sizes (INFO) :
    {
        KEEP(*(.stack_sizes));
    }

    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*)
	    	/* Unused exception related info that only wastes space */
		*(.ARM.exidx.*);
		*(.ARM.extab.*);
    }
}
