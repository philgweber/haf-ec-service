	.section ".vector_table","ax"
	.global vector_table
	.balign 0x800
vector_table:
curr_el_sp0_sync:
	b	.

	.balign 0x80
curr_el_sp0_irq:
	b	.

	.balign 0x80
curr_el_sp0_fiq:
	b	.

	.balign 0x80
curr_el_sp0_serror:
	b	.


	.balign 0x80
curr_el_spx_sync:
	b	.

	.balign 0x80
curr_el_spx_irq:
	b	.

	.balign 0x80
curr_el_spx_fiq:
	b	.

	.balign 0x80
curr_el_spx_serror:
	b	.


	.balign 0x80
lower_el_aarch64_sync:
	b	.

	.balign 0x80
lower_el_aarch64_irq:
	b	.

	.balign 0x80
lower_el_aarch64_fiq:
	b	.

	.balign 0x80
lower_el_aarch64_serror:
	b	.


	.balign 0x80
lower_el_aarch32_sync:
	b	.

	.balign 0x80
lower_el_aarch32_irq:
	b	.

	.balign 0x80
lower_el_aarch32_fiq:
	b	.

	.balign 0x80
lower_el_aarch32_serror:
	b	.
