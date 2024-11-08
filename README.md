# Hafnium EC Service in Rust
This is sample implementation of the EC service which runs under a separate SP partition in Hafnium.

It is written in Rust and has dependencies on FF-A, MU UEFI, Hafnium and TFA

## Customizations
Update memory.x to match the memory map from your dts file
```
	load-address = <0x0 0x20400000>;
	entrypoint-offset = <0x10000>;
	image-size = <0x0 0x40000>;
```
From memory.x this needs to match
```
MEMORY
{
    FLASH (rx) : ORIGIN = 0x20410000, LENGTH = 0x30000
    DRAM (rwx) : ORIGIN = 0x20440000, LENGTH = 0x100000
}
```

## Build Instructions
cargo xtask build

## Testing for Submission
cargo +nightly fmt
cargo +nightly clippy
cargo hack --feature-powerset check
cargo deny check
