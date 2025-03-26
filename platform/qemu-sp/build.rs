// Copyright 2025 Microsoft Corporation
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

fn main() {
    println!("cargo:rustc-link-arg=-Timage.ld");
    println!("cargo:rustc-link-arg=-Tlinker/qemu.ld");
    println!("cargo:rerun-if-changed=linker/qemu.ld");
    println!("cargo:rerun-if-changed=linker/image.ld");
    println!("cargo:rerun-if-changed=src/main.rs");
}
