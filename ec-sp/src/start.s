// Load the address of a symbol into a register, PC-relative.
//
// The symbol must lie within +/- 4 GiB of the Program Counter.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add	\register, \register, #:lo12:\symbol
.endm

	.extern sp_main

	.section ".text.boot"
	.global _start
	.global _ffa_console_log
_start:
	// Only proceed if the core executes in EL1. Park it otherwise.
	mrs	x0, CurrentEL
	cmp	x0, {CONST_CURRENTEL_EL1}
	b.ne	parking_loop

	// read cpu id, stop slave cores
	mrs     x1, MPIDR_EL1
	and     x0, x0, {CONST_CORE_ID_MASK}
	ldr	x1, BOOT_CORE_ID

	// If $this is not the boot core, park it
	cmp	x0, x1
	b.ne	parking_loop
	
    // Initialize exception vector right away
	ADR_REL x0, exception_vector
	msr VBAR_EL1, x0

	// Initialize DRAM
	ADR_REL	x0, _bss
	ADR_REL	x1, _ebss

bss_init_loop:
	// Clear BSS
	cmp	x0, x1
	b.eq	prepare_rust
	stp	xzr, xzr, [x0], #16
	b	bss_init_loop

prepare_rust:
	// Set the Secure EL1 stack pointer
	ADR_REL	x0, _stack
	mov sp, x0

	// Jump to Rust
	b	sp_main

parking_loop:
	wfe
	b parking_loop

	.balign 128
exception_vector:
	b exception_vector
	.balign 128
	b exception_vector
	.balign 128
	b exception_vector
	.balign 128
	b exception_vector
	b exception_vector
	.balign 128
	b exception_vector
	.balign 128
	b exception_vector
	.balign 128
	b exception_vector


	.size _start, . - _start
	.type _start, function
