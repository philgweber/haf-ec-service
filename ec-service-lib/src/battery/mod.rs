use ffa::msg::FfaMsg;
use ffa::{FfaError, FfaFunctionId};

pub type Result<T> = core::result::Result<T, ffa::FfaError>;

#[derive(Default)]
struct GenericRsp {
    _status: i64,
}

#[derive(Default)]
pub struct Battery {}

impl Battery {
    pub fn new() -> Self {
        Self::default()
    }

    fn ffa_msg_send_direct_req2(&self, msg: &FfaMsg) -> Result<FfaMsg> {
        let cmd = msg.extract_u8_at_index(0);
        println!("Received Battery command 0x{:x}", cmd);

        // Create new generic rsp packet swap destination and source
        let mut rsp = FfaMsg {
            function_id: FfaFunctionId::FfaMsgSendDirectResp2.into(),
            source_id: msg.destination_id,
            destination_id: msg.source_id,
            uuid: msg.uuid,
            ..Default::default()
        };
        rsp.struct_to_args64(&GenericRsp { _status: 0x0 });
        Ok(rsp)
    }

    // Handles messages sent to the EC Firmware management service
    pub(crate) fn exec(self, msg: &FfaMsg) -> Result<FfaMsg> {
        let id = FfaFunctionId::from(msg.function_id);

        match id {
            FfaFunctionId::FfaMsgSendDirectReq2 => self.ffa_msg_send_direct_req2(msg),
            _ => {
                println!("Unhandled FfaFunctionId in Battery: {:?}", id);
                Err(FfaError::InvalidParameters)
            }
        }
    }
}
