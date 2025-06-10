use crate::{exec_simple, Error, ExecResult, Function, FunctionId, SmcParams};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct RxTxMap {
    tx_address: u64,
    rx_address: u64,
    page_count: u32,
}

impl RxTxMap {
    pub fn new(tx_address: u64, rx_address: u64, page_count: u32) -> Self {
        Self {
            tx_address,
            rx_address,
            page_count,
        }
    }
}

impl Function for RxTxMap {
    const ID: FunctionId = FunctionId::RxTxMap;
    type ReturnType = ();

    fn exec(self) -> ExecResult<Self::ReturnType> {
        exec_simple(self, |_| Ok(()))
    }
}

impl TryInto<SmcParams> for RxTxMap {
    type Error = Error;

    fn try_into(self) -> Result<SmcParams, Self::Error> {
        Ok(SmcParams {
            x1: self.tx_address,
            x2: self.rx_address,
            x3: self.page_count as u64,
            ..Default::default()
        })
    }
}

impl TryFrom<SmcParams> for RxTxMap {
    type Error = Error;

    fn try_from(value: SmcParams) -> Result<Self, Self::Error> {
        Ok(RxTxMap {
            tx_address: value.x1,
            rx_address: value.x2,
            page_count: value.x3 as u32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case::zero_values(0, 0, 0)]
    #[case::max_u64_addresses(u64::MAX, u64::MAX, 0)]
    #[case::max_page_count(1, 2, u32::MAX)]
    #[case::typical_values(0x80000000, 0x90000000, 256)]
    fn test_rxtx_map_round_trip(#[case] tx_address: u64, #[case] rx_address: u64, #[case] page_count: u32) {
        let original_map = RxTxMap {
            tx_address,
            rx_address,
            page_count,
        };

        let params: SmcParams = original_map.clone().try_into().unwrap();
        let new_map: RxTxMap = params.try_into().unwrap();

        assert_eq!(original_map, new_map);
    }
}
