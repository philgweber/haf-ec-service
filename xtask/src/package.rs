use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum Package {
    Loader,
}

impl Package {
    pub(crate) fn all() -> Vec<Package> {
        vec![Self::Loader]
    }

    pub(crate) fn as_arg(&self) -> [&str; 2] {
        match self {
            Self::Loader => ["--package", "ec-sp"],
        }
    }

    pub(crate) fn as_pflash(&self, release: bool) -> [String; 2] {
        let mut path = PathBuf::from("target/aarch64-unknown-none-softfloat");

        if release {
            path.push("release");
        } else {
            path.push("debug");
        }

        match self {
            Self::Loader => {
                path.push("ec-sp.bin");
            }
        }

        [
            "-drive".into(),
            format!(
                "if=pflash,format=raw,file={}",
                path.as_path().to_str().unwrap()
            ),
        ]
    }
}
