// Copyright 2025 Microsoft Corporation
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "none" {
        println!("cargo:rustc-cfg=feature=\"baremetal\"");
        println!("cargo:rustc-link-arg=-Timage.ld");
        println!("cargo:rustc-link-arg=-Tplatform/qemu-sp/linker/qemu.ld");
        println!("cargo:rerun-if-changed=platform/qemu-sp/linker/qemu.ld");
        println!("cargo:rerun-if-changed=linker/image.ld");
    }
}
