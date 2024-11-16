use ffa::msg::FfaMsg;
use ffa::FfaDirectMsg;
use ffa::FfaFunctionId;

use crate::error;
//use crate::panic;

pub type Result<T> = core::result::Result<T, error::Error>;

// Protocol CMD definitions for FwMgmt
const EC_CAP_GET_FW_STATE: u8 = 0x1;
const EC_CAP_GET_SVC_LIST: u8 = 0x2;
const EC_CAP_GET_BID: u8 = 0x3;

#[derive(Default)]
struct FwStateRsp {
    _fw_version: u16,
    _secure_state: u8,
    _boot_status: u8,
}

#[derive(Default)]
struct ServiceListRsp {
    _debug_mask: u16,
    _battery_mask: u8,
    _fan_mask: u8,
    _thermal_mask: u8,
    _hid_mask: u8,
    _key_mask: u16,
}

#[derive(Default)]
struct GetBidRsp {
    _bid: u64,
}

#[derive(Default)]
pub struct FwMgmt {}

impl FwMgmt {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_fw_state(&self) -> FwStateRsp {
        FwStateRsp {
            _fw_version: 0x0100,
            _secure_state: 0x0,
            _boot_status: 0x1,
        }
    }

    fn get_svc_list(&self) -> ServiceListRsp {
        ServiceListRsp {
            _debug_mask: 0x1,
            _battery_mask: 0x1,
            _fan_mask: 0x1,
            _thermal_mask: 0x1,
            _hid_mask: 0x0,
            _key_mask: 0x7,
        }
    }

    fn get_bid(&self) -> GetBidRsp {
        GetBidRsp { _bid: 0xdead0001 }
    }

    fn ffa_msg_send_direct_req2(&self, msg: &FfaMsg) -> Result<FfaDirectMsg> {
        let cmd = msg.extract_u8_at_index(0);
        println!("Received FwMgmt command 0x{:x}", cmd);

        // Create new generic rsp packet swap destination and source
        let mut rsp = FfaDirectMsg::new(
            FfaFunctionId::FfaMsgSendDirectResp2,
            Some(msg.destination_id()),
            Some(msg.source_id()),
            Some(msg.uuid()),
            Some([0; 14]),
        );
        match cmd {
            EC_CAP_GET_FW_STATE => {
                rsp.struct_to_args64(&self.get_fw_state());
                Ok(rsp)
            }
            EC_CAP_GET_SVC_LIST => {
                rsp.struct_to_args64(&self.get_svc_list());
                Ok(rsp)
            }
            EC_CAP_GET_BID => {
                rsp.struct_to_args64(&self.get_bid());
                Ok(rsp)
            }
            _ => panic!("Unknown FwMgmt Command"),
        }
    }

    // Handles messages sent to the EC Firmware management service
    pub(crate) fn exec(self, msg: &FfaMsg) -> Result<FfaDirectMsg> {
        let id = FfaFunctionId::from(msg.function_id() as u64);

        match id {
            FfaFunctionId::FfaMsgSendDirectReq2 => self.ffa_msg_send_direct_req2(msg),
            _ => panic!("Unhandled FfaFunctionId in FwMgmt"),
        }
    }
}
