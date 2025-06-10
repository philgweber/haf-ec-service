use crate::{exec_simple, Error, ExecResult, Function, SmcParams};

use crate::{FunctionId, SmcCall};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct MemRetrieveReq {
    total_length: u64,
    frag_length: u64,
    tx_address: u64,
    page_count: u64,
}

impl MemRetrieveReq {
    pub fn new() -> Self {
        Self {
            total_length: 0x40,
            frag_length: 0x40,
            tx_address: 0,
            page_count: 0,
        }
    }
}

impl Function for MemRetrieveReq {
    type ReturnType = SmcCall;
    const ID: FunctionId = FunctionId::MemRetrieveReq;

    fn exec(self) -> ExecResult<Self::ReturnType> {
        exec_simple(self, Ok)
    }
}

impl TryInto<SmcParams> for MemRetrieveReq {
    type Error = Error;

    fn try_into(self) -> Result<SmcParams, Self::Error> {
        Ok(SmcParams {
            x1: self.total_length,
            x2: self.frag_length,
            x3: self.tx_address,
            x4: self.page_count,
            ..Default::default()
        })
    }
}

impl TryFrom<SmcParams> for MemRetrieveReq {
    type Error = Error;

    fn try_from(value: SmcParams) -> Result<Self, Self::Error> {
        Ok(MemRetrieveReq {
            total_length: value.x1,
            frag_length: value.x2,
            tx_address: value.x3,
            page_count: value.x4,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_retrieve_req_round_trip() {
        let original_req = MemRetrieveReq {
            total_length: 1024,
            frag_length: 512,
            tx_address: 0x80000000,
            page_count: 2,
        };

        let params: SmcParams = original_req.clone().try_into().unwrap();
        let new_req: MemRetrieveReq = params.try_into().unwrap();

        assert_eq!(original_req, new_req);
    }
}
