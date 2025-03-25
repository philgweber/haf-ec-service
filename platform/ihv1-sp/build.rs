// Copyright 2025 Microsoft Corporation
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

fn main() {
    println!("cargo:rustc-link-arg=-Timage.ld");
    println!("cargo:rustc-link-arg=-Tlinker/ihv1.ld");
    println!("cargo:rerun-if-changed=linker/ihv1.ld");
    println!("cargo:rerun-if-changed=linker/image.ld");
    println!("cargo:rerun-if-changed=src/main.rs");
}
