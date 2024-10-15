use crate::package::*;
use anyhow::Result;
use std::process::Command;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum CargoAction {
    Build,
    Clippy,
    Format,
    Test,
}

#[derive(Debug)]
pub(crate) struct Cargo<'a> {
    pub(crate) action: CargoAction,
    pub(crate) release: bool,
    pub(crate) features: &'a [String],
}

impl Cargo<'_> {
    pub(crate) fn cmd(self) -> Result<Command> {
        let mut cmd = Command::new("cargo");

        let action = match self.action {
            CargoAction::Build => "build",
            CargoAction::Clippy => "clippy",
            CargoAction::Format => "fmt",
            CargoAction::Test => "test",
        };

        cmd.arg(action);

        // Hardcoding the target here for now as we're only supporting a single
        // target
        cmd.args(["--target", "aarch64-unknown-none-softfloat"]);

        if !self.features.is_empty() {
            cmd.args([
                "--features",
                self.features
                    .into_iter()
                    .fold("".to_string(), |acc, feature| acc + "," + &feature)
                    .as_str(),
            ]);
        }

        if self.action != CargoAction::Format && self.action != CargoAction::Test {
            if self.release {
                cmd.arg("--release");
            }

            cmd.args(
                Package::all()
                    .iter()
                    .map(|package| package.as_arg())
                    .flatten()
                    .collect::<Vec<_>>(),
            );
        }

        Ok(cmd)
    }
}
