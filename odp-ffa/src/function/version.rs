use crate::{ffa_smc, Error, ExecResult, Function, FunctionId, SmcParams};

macro_rules! ffa_version {
    ($major:expr, $minor:expr) => {
        ($major as u64) << 16 | ($minor as u64)
    };
}

#[derive(Clone, Debug, PartialEq)]
pub struct Version {
    major: u16,
    minor: u16,
}

impl Function for Version {
    const ID: FunctionId = FunctionId::Version;
    type ReturnType = Self;

    fn exec(self) -> ExecResult<Self::ReturnType> {
        let result = ffa_smc(self)?;
        if result[0] & (1 << 31) == 0 {
            Ok(Self {
                major: (result[0] >> 16) as u16,
                minor: (result[0] & 0xffff) as u16,
            })
        } else {
            Err(Error::InvalidFunctionId(result[0]))
        }
    }
}

impl TryInto<SmcParams> for Version {
    type Error = Error;

    fn try_into(self) -> Result<SmcParams, Self::Error> {
        Ok(SmcParams {
            x1: ffa_version!(self.major, self.minor),
            ..Default::default()
        })
    }
}

impl TryFrom<SmcParams> for Version {
    type Error = Error;

    fn try_from(value: SmcParams) -> Result<Self, Self::Error> {
        Ok(Version {
            major: (value.x1 >> 16) as u16,
            minor: (value.x1 & 0xFFFF) as u16,
        })
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl Version {
    const FFA_VERSION_MAJOR: u64 = 1;
    const FFA_VERSION_MINOR: u64 = 2;

    pub fn new() -> Self {
        Self {
            major: Self::FFA_VERSION_MAJOR as u16,
            minor: Self::FFA_VERSION_MINOR as u16,
        }
    }

    pub fn major(&self) -> u16 {
        self.major
    }

    pub fn minor(&self) -> u16 {
        self.minor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case::zero_values(0, 0)]
    #[case::max_values(u16::MAX, u16::MAX)]
    #[case::ffa_defined_version(Version::FFA_VERSION_MAJOR as u16, Version::FFA_VERSION_MINOR as u16)]
    #[case::typical_values(1, 2)]
    fn test_version_round_trip(#[case] major: u16, #[case] minor: u16) {
        let original_version = Version { major, minor };

        let params: SmcParams = original_version.clone().try_into().unwrap();
        let new_version: Version = params.try_into().unwrap();

        assert_eq!(original_version, new_version);
    }
}
