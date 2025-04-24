use crate::{exec_simple, Error, ExecResult, Function, FunctionId, SmcCall, SmcParams};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MsgWait;
impl MsgWait {
    pub fn new() -> Self {
        Self {}
    }
}

impl Function for MsgWait {
    const ID: FunctionId = FunctionId::MsgWait;
    type ReturnType = SmcCall;

    fn exec(self) -> ExecResult<Self::ReturnType> {
        exec_simple(self, Ok)
    }
}

impl TryInto<SmcParams> for MsgWait {
    type Error = Error;

    fn try_into(self) -> Result<SmcParams, Self::Error> {
        Ok(SmcParams::default())
    }
}

impl TryFrom<SmcParams> for MsgWait {
    type Error = Error;

    fn try_from(_value: SmcParams) -> Result<Self, Self::Error> {
        // MsgWait doesn't carry data, so any SmcParams converts to it.
        // TryInto produces SmcParams::default().
        Ok(MsgWait)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::main()]
    fn test_msg_wait_round_trip() {
        let original_msg_wait = MsgWait::new();

        let params: SmcParams = original_msg_wait.try_into().unwrap();
        // Check that TryInto produces default SmcParams as expected
        assert_eq!(params, SmcParams::default());

        let new_msg_wait: MsgWait = params.try_into().unwrap();

        assert_eq!(original_msg_wait, new_msg_wait);
    }
}
