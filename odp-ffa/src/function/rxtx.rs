use crate::{exec_simple, Error, ExecResult, Function, FunctionId, SmcParams};
use core::ptr;

#[repr(align(4096))]
pub struct Aligned4K(pub [u8; 4096]);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct RxTxMap {
    tx_address: *mut u8,
    rx_address: *mut u8,
    page_count: u32,
}

impl RxTxMap {
    pub fn new(tx_address: &mut Aligned4K, rx_address: &mut Aligned4K, page_count: u32) -> Self {
        Self {
            tx_address : tx_address.0.as_ptr() as *mut u8,
            rx_address : rx_address.0.as_ptr() as *mut u8,
            page_count,
        }
    }
    pub fn set_tx_buffer(&self, src: &[u8]) {
        assert!(src.len() <= 4096);
        
        // SAFETY: We're copying into a properly aligned and sized buffer.
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), self.tx_address, src.len());
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
            x1: self.tx_address as u64,
            x2: self.rx_address as u64,
            x3: self.page_count as u64,
            ..Default::default()
        })
    }
}

impl TryFrom<SmcParams> for RxTxMap {
    type Error = Error;

    fn try_from(value: SmcParams) -> Result<Self, Self::Error> {
        Ok(RxTxMap {
            tx_address: value.x1 as *mut u8,
            rx_address: value.x2 as *mut u8,
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
