	.section ".text.boot"
	.global hyp_console_log


hyp_console_log:
	// Copy input character into x2
	mov x2,x0

	// Load up parameters for SMC call x0-x17
	mov x0,#0xC4 // FFA_CONSOLE_LOG_64
	lsl x0,x0,#0x18
	add x0,x0,#0x8A
	mov x1,#1
	smc #0
	ret