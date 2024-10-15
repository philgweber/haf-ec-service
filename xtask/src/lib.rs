use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::process::Command;

mod cargo;
mod objcopy;
mod package;
mod qemu;

use cargo::*;
use objcopy::*;
use qemu::*;

#[derive(Parser, Debug)]
#[command(
    name = "xtask",
    about = "Tool for running common tasks within loader-rs",
    author,
    version
)]
pub struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand, Debug)]
enum CliCommand {
    Build(Options),
    Clippy(Options),
    Format,
    Run(Options),
    Test,
}

#[derive(Parser, Debug)]
struct Options {
    #[arg(short, long, default_value_t)]
    release: bool,

    #[arg(short, long)]
    gdb: bool,

    #[arg(short = 'F', long, value_delimiter = ',')]
    features: Vec<String>,
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            CliCommand::Build(opts) => self.build(opts).await,
            CliCommand::Clippy(opts) => self.clippy(opts).await,
            CliCommand::Format => self.format().await,
            CliCommand::Run(opts) => self.run_qemu(opts).await,
            CliCommand::Test => self.test().await,
        }
    }

    async fn build(&self, opts: &Options) -> Result<()> {
        let cargo = Cargo {
            action: CargoAction::Build,
            release: opts.release,
            features: &opts.features,
        };

        let cmd = cargo.cmd().context("Failed to run cargo command")?;
        self.run_cmd(cmd).await?;

        self.objcopy(opts).await
    }

    async fn objcopy(&self, opts: &Options) -> Result<()> {
        let objcopy = Objcopy {
            format: ObjcopyFormat::Binary,
            release: opts.release,
        };

        let cmd = objcopy.cmd().context("Failed to run objcopy command")?;
        self.run_cmd(cmd).await
    }

    async fn clippy(&self, opts: &Options) -> Result<()> {
        let cargo = Cargo {
            action: CargoAction::Clippy,
            release: opts.release,
            features: &vec![],
        };

        let cmd = cargo.cmd().context("Failed to run cargo command")?;
        self.run_cmd(cmd).await
    }

    async fn format(&self) -> Result<()> {
        let cargo = Cargo {
            action: CargoAction::Format,
            release: false,
            features: &vec![],
        };

        let cmd = cargo.cmd().context("Failed to run cargo command")?;
        self.run_cmd(cmd).await
    }

    async fn run_qemu(&self, opts: &Options) -> Result<()> {
        let qemu = Qemu {
            release: opts.release,
            gdb: opts.gdb,
        };

        let cmd = qemu.cmd().context("Failed to run qemu command")?;
        self.run_cmd(cmd).await
    }

    async fn test(&self) -> Result<()> {
        let cargo = Cargo {
            action: CargoAction::Test,
            release: false,
            features: &vec![],
        };

        let cmd = cargo.cmd().context("Failed to run cargo command")?;
        self.run_cmd(cmd).await
    }

    async fn run_cmd(&self, mut cmd: Command) -> Result<()> {
        let status = cmd.status().context("Cargo produced a failure status")?;

        if status.success() {
            Ok(())
        } else {
            bail!("Cargo command failed with status {status}")
        }
    }
}
