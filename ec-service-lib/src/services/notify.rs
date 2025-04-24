use crate::{Result, Service};
use log::{debug, error};
use odp_ffa::{ErrorCode, MsgSendDirectReq2, MsgSendDirectResp2, Payload, RegisterPayload};
use uuid::{uuid, Uuid};

// Protocol CMD definitions for Notification
const NFY_QUERY: u8 = 0x0;
const NFY_SETUP: u8 = 0x1;
const NFY_DESTROY: u8 = 0x2;

#[derive(Default)]
struct NfyQueryRsp {
    status: i64,
    start_id: u16,
    last_id: u16,
}

impl From<NfyQueryRsp> for RegisterPayload {
    fn from(value: NfyQueryRsp) -> Self {
        RegisterPayload::from_iter(
            value
                .status
                .to_le_bytes()
                .into_iter()
                .chain(value.start_id.to_le_bytes())
                .chain(value.last_id.to_le_bytes()),
        )
    }
}

#[derive(Default)]
struct NfyGenericRsp {
    status: i64,
}

impl From<NfyGenericRsp> for RegisterPayload {
    fn from(value: NfyGenericRsp) -> Self {
        RegisterPayload::from_iter(value.status.to_le_bytes())
    }
}

#[derive(Default)]
pub struct Notify {}

impl Notify {
    pub fn new() -> Self {
        Self::default()
    }

    fn nfy_query(&self) -> NfyQueryRsp {
        NfyQueryRsp {
            status: 0x0,
            start_id: 0x0,
            last_id: 0x1,
        }
    }

    fn nfy_setup(&self, msg: &MsgSendDirectReq2) -> NfyGenericRsp {
        debug!("cmd: {}", msg.register_at(0));
        debug!("uuid_lo: 0x{:x}", msg.register_at(1));
        debug!("uuid_hi: 0x{:x}", msg.register_at(2));
        debug!("Count: {}", msg.register_at(3));
        debug!("Mapping1: 0x{:x}", msg.register_at(4));
        debug!("Mapping2: 0x{:x}", msg.register_at(5));
        NfyGenericRsp {
            status: ErrorCode::Ok.into(),
        }
    }

    fn nfy_destroy(&self, _msg: &MsgSendDirectReq2) -> NfyGenericRsp {
        NfyGenericRsp { status: 0x0 }
    }
}

const UUID: Uuid = uuid!("B510B3A3-59F6-4054-BA7A-FF2EB1EAC765");

impl Service for Notify {
    fn service_name(&self) -> &'static str {
        "Notify"
    }

    fn service_uuid(&self) -> Uuid {
        UUID
    }

    async fn ffa_msg_send_direct_req2(&mut self, msg: MsgSendDirectReq2) -> Result<MsgSendDirectResp2> {
        let cmd = msg.u8_at(0);
        debug!("Received notify command 0x{:x}", cmd);

        let payload = match cmd {
            NFY_QUERY => RegisterPayload::from(self.nfy_query()),
            NFY_SETUP => RegisterPayload::from(self.nfy_setup(&msg)),
            NFY_DESTROY => RegisterPayload::from(self.nfy_destroy(&msg)),
            _ => {
                error!("Unknown Notify Command: {}", cmd);
                return Err(odp_ffa::Error::Other("Unknown Notify Command"));
            }
        };

        Ok(MsgSendDirectResp2::from_req_with_payload(&msg, payload))
    }
}
