use crate::{exec_simple, Error, ExecResult, Function, FunctionId, SmcParams};

#[derive(Clone, Debug, PartialEq)]
pub struct Yield {
    pub vcpu_id: u16,
    pub endpoint_id: u16,
    pub timeout: u64,
}

impl Function for Yield {
    const ID: FunctionId = FunctionId::MsgYield;
    type ReturnType = ();

    fn exec(self) -> ExecResult<Self::ReturnType> {
        exec_simple(self, |_| Ok(()))
    }
}

impl TryInto<SmcParams> for Yield {
    type Error = Error;

    fn try_into(self) -> Result<SmcParams, Self::Error> {
        Ok(SmcParams {
            x1: ((self.endpoint_id as u64) << 16) | (self.vcpu_id as u64),
            x2: (self.timeout as u32) as u64,
            x3: (self.timeout >> 32),
            ..Default::default()
        })
    }
}

impl TryFrom<SmcParams> for Yield {
    type Error = Error;

    fn try_from(value: SmcParams) -> Result<Self, Self::Error> {
        Ok(Yield {
            vcpu_id: (value.x1 & 0xFFFF) as u16,
            endpoint_id: ((value.x1 >> 16) & 0xFFFF) as u16,
            timeout: (value.x3 << 32) | value.x2,
        })
    }
}

impl Yield {
    pub fn new(timeout: u64) -> Self {
        Yield {
            vcpu_id: 0,
            endpoint_id: 0,
            timeout,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::zero_values(0, 0, 0)]
    #[case::max_ids(u16::MAX, u16::MAX, 0)]
    #[case::max_timeout(1, 2, u64::MAX)]
    #[case::typical_values(0x12, 0x34, 0x1234567890ABCDEF)]
    fn test_yield_round_trip(#[case] vcpu_id: u16, #[case] endpoint_id: u16, #[case] timeout: u64) {
        let original_yield = Yield {
            vcpu_id,
            endpoint_id,
            timeout,
        };

        let params: SmcParams = original_yield.clone().try_into().unwrap();
        let new_yield: Yield = params.try_into().unwrap();

        assert_eq!(original_yield.vcpu_id, new_yield.vcpu_id);
        assert_eq!(original_yield.endpoint_id, new_yield.endpoint_id);
        assert_eq!(original_yield.timeout, new_yield.timeout);
        assert_eq!(original_yield, new_yield);
    }
}
