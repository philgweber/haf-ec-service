use anyhow::Result;
use std::{path::PathBuf, process::Command};

#[derive(PartialEq, PartialOrd, Debug)]
pub enum ObjcopyFormat {
    Binary,
}

#[derive(Debug)]
pub(crate) struct Objcopy {
    pub(crate) format: ObjcopyFormat,
    pub(crate) release: bool,
}

impl Objcopy {
    pub(crate) fn cmd(&self) -> Result<Command> {
        let mut cmd = Command::new("rust-objcopy");

        let format = match self.format {
            ObjcopyFormat::Binary => "binary",
        };

        cmd.args(["-O", format, "--gap-fill", "0x00", "--pad-to=0x4000000"]);

        let mut path = PathBuf::from("target/aarch64-unknown-none-softfloat");

        if self.release {
            path.push("release");
        } else {
            path.push("debug");
        }

        path.push("ec-sp");
        cmd.arg(path.as_path());

        path.pop();
        path.push("ec-sp.bin");
        cmd.arg(path.as_path());

        Ok(cmd)
    }
}
