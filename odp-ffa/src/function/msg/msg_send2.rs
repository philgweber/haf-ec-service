use crate::{exec_simple, Error, ExecResult, Function, FunctionId, SmcParams};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MsgSend2 {
    pub sender_id: u16,
    pub flags: u32,
}

impl MsgSend2 {
    pub fn new(sender_id: u16, flags: u32) -> Self {
        Self { sender_id, flags }
    }
}

impl Function for MsgSend2 {
    const ID: FunctionId = FunctionId::MsgSend2;
    type ReturnType = ();

    fn exec(self) -> ExecResult<Self::ReturnType> {
        exec_simple(self, |_| Ok(()))
    }
}

impl TryInto<SmcParams> for MsgSend2 {
    type Error = Error;

    fn try_into(self) -> Result<SmcParams, Self::Error> {
        Ok(SmcParams {
            x1: (self.sender_id as u64) << 16,
            x2: self.flags as u64,
            ..Default::default()
        })
    }
}

impl TryFrom<SmcParams> for MsgSend2 {
    type Error = Error;

    fn try_from(value: SmcParams) -> Result<Self, Self::Error> {
        Ok(MsgSend2 {
            sender_id: (value.x1 >> 16) as u16,
            flags: value.x2 as u32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::zero_values(0, 0)]
    #[case::max_sender_id(u16::MAX, 0)]
    #[case::max_flags(123, u32::MAX)]
    #[case::typical_values(0xABCD, 0x12345678)]
    fn test_msg_send2_round_trip(#[case] sender_id: u16, #[case] flags: u32) {
        let original_msg = MsgSend2 { sender_id, flags };

        let params: SmcParams = original_msg.try_into().unwrap();
        let new_msg: MsgSend2 = params.try_into().unwrap();

        assert_eq!(original_msg, new_msg);
    }
}
