# IHV Embedded Controller Secure Partition

An example secure partition reference for independet hardware vendor that uses haf-ec-service.

- `aarch64-paging` for page table management.
- `aarch64-rt` for the entry point and exception handling.

## Building

```
cargo build
cargo objcopy -- -O binary target/aarch64-unknown-none/debug/ihv1-ec-sp.bin
```
