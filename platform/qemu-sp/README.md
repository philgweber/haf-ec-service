# QEMU Embedded Controller Secure Partition

An implementation of embedded controller secure partition for QEMU haf-ec-service.

- `aarch64-paging` for page table management.
- `aarch64-rt` for the entry point and exception handling.

## Building

```
cargo build --target=aarch64-unknown-none
cargo objcopy -- -O binary target/aarch64-unknown-none/debug/qemu-ec-sp.bin
```
