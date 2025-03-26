// Copyright 2025 Microsoft Corporation
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "none" {
        println!("cargo:rustc-cfg=feature=\"baremetal\"");
        println!("cargo:rustc-link-arg=-Timage.ld");
        println!("cargo:rustc-link-arg=-Tplatform/ihv1-sp/linker/ihv1.ld");
        println!("cargo:rerun-if-changed=platform/ihv1-sp/linker/ihv1.ld");
        println!("cargo:rerun-if-changed=linker/image.ld");
    }
}
