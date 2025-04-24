# Hafnium Rust Bindings

This crate provides a partial set of Rust bindings for interacting with the Hafnium hypervisor. It is designed to offer a type-safe wrapper for making Hypervisor Calls (HVCs) into Hafnium from EL1 applications.

## Scope

**This crate is incomplete and does not cover the entire Hafnium API.** It currently provides functionality related to interrupt management (`hf_interrupt_set`, `hf_interrupt_get`, `hf_interrupt_deactivate`).

## Usage

The primary interface is through the `hf_call` function (and its wrappers), which directly translates to an `hvc` instruction when compiled for `aarch64` in a `no_std` environment.

For example, to enable an interrupt:

```rust
use hafnium::{hf_interrupt_set, InterruptId, InterruptType};

// ...

let interrupt_id = InterruptId(123); // Example interrupt ID
match hf_interrupt_set(interrupt_id, InterruptType::Irq, true) {
    Ok(()) => println!("Interrupt enabled successfully."),
    Err(e) => eprintln!("Failed to enable interrupt: {}", e),
}
```

## Building

This crate can be built for `aarch64-unknown-none` target. When not built for this specific target, the `hf_call` function will print its arguments to the console instead of making an actual hypervisor call, allowing for easier development and testing on other platforms.
