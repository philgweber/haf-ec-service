# Hafnium EC Service in Rust

## Overview
This is sample implementation of the EC service which runs under a separate SP partition in Hafnium.

It is written in Rust and has dependencies on FF-A, MU UEFI, Hafnium and TFA

## Feature Status
The following components are available within this crate.
```
xtask       - Wrapper around cargo process for bare metal compilation
ec-sp       - EC secure partition souce code, memory map and build files
build.rs    - Creates memory map files from input memory.x
memory.x    - Memory map for linking output binaries
start.s     - Initial entry point in assembly jumps to sp_main
exception.s - Exception handling code
main.rs     - Main entry point for SP
panic.rs    - Panic handling code for RUST panic
fw_mgmt     - Implements the EC firmware management service
notify      - Implements the EC notification service
thermal     - Implements the EC thermal service
```

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
`cargo xtask build`

## Build Validation
Before submission the following commands should all be validated to make sure there files are correctly formatted any only valid crates and features are used.
```
cargo +nightly fmt
cargo +nightly clippy
cargo hack --feature-powerset check
cargo deny check
```
