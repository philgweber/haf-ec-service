use crate::{exec_simple, Error, ExecResult, Function, FunctionId, SmcCall, SmcParams};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NormalWorldResume;
impl NormalWorldResume {
    pub fn new() -> Self {
        Self {}
    }
}

impl Function for NormalWorldResume {
    const ID: FunctionId = FunctionId::NormalWorldResume;
    type ReturnType = SmcCall;

    fn exec(self) -> ExecResult<Self::ReturnType> {
        exec_simple(self, Ok)
    }
}

impl TryInto<SmcParams> for NormalWorldResume {
    type Error = Error;

    fn try_into(self) -> Result<SmcParams, Self::Error> {
        Ok(SmcParams::default())
    }
}

impl TryFrom<SmcParams> for NormalWorldResume {
    type Error = Error;

    fn try_from(_value: SmcParams) -> Result<Self, Self::Error> {
        // No parameters used in resume
        Ok(NormalWorldResume)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::main()]
    fn test_normal_world_round_trip() {
        let original_normal_world = NormalWorldResume::new();

        let params: SmcParams = original_normal_world.try_into().unwrap();
        // Check that TryInto produces default SmcParams as expected
        assert_eq!(params, SmcParams::default());

        let new_normal_world: NormalWorldResume = params.try_into().unwrap();

        assert_eq!(original_normal_world, new_normal_world);
    }
}
