use crate::package::Package;
use anyhow::{bail, Result};
use std::{env, path::PathBuf, process::Command};

pub(crate) struct Qemu {
    pub(crate) release: bool,
    pub(crate) gdb: bool,
}

impl Qemu {
    pub(crate) fn cmd(&self) -> Result<Command> {
        let app = if cfg!(windows) {
            "qemu-system-aarch64.exe"
        } else {
            "qemu-system-aarch64"
        };

        let mut cmd = Command::new(app);

        if cfg!(windows) {
            // Does the binary exist?
            let mut path = PathBuf::from(r"C:\Program Files\qemu");

            path.push(app);

            if !path.exists() {
                bail!("Unable find `{}'", app);
            }

            // Append qemu install directory to the PATH in case the
            // user has not done so.
            let mut path = env::var_os("PATH").unwrap_or_default();
            path.push(r";C:\Program Files\qemu");
            cmd.env("PATH", path);
        } else if !cfg!(windows) {
            // Does the binary exist?
            let mut path1 = PathBuf::from("/bin");
            let mut path2 = PathBuf::from("/usr/bin");
            let mut path3 = PathBuf::from("/sbin");
            let mut path4 = PathBuf::from("/usr/sbin");

            path1.push(app);
            path2.push(app);
            path3.push(app);
            path4.push(app);

            if !path1.exists() && !path2.exists() && !path3.exists() && !path4.exists() {
                bail!("Unable find `{}'", app);
            }
        }

        cmd.args(["-machine", "virt,virtualization=on"]);
        cmd.args(["-cpu", "cortex-a76"]);
        cmd.args(["-m", "2g"]);
        cmd.args(["-display", "none"]);
        cmd.args([
            "-chardev",
            "stdio,id=char0,mux=on,logfile=serial.log,signal=off",
        ]);
        cmd.args(["-serial", "chardev:char0"]);
        cmd.args(["-mon", "chardev=char0"]);

        if self.gdb {
            println!("GDB requested. Server will be started at localhost:1234");

            cmd.arg("-S");
            cmd.arg("-s");
        }

        cmd.args(Package::Loader.as_pflash(self.release));

        Ok(cmd)
    }
}
