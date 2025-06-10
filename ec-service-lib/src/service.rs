use core::future::Future;

use log::error;
use odp_ffa::{FunctionId, MsgSendDirectReq2, MsgSendDirectResp2};
use uuid::Uuid;

pub type Result<T> = core::result::Result<T, odp_ffa::Error>;

pub trait Service {
    fn service_name(&self) -> &'static str;
    fn service_uuid(&self) -> Uuid;

    fn ffa_msg_send_direct_req2(&mut self, msg: MsgSendDirectReq2) -> impl Future<Output = Result<MsgSendDirectResp2>> {
        async move { self.handler_unimplemented(msg).await }
    }
}

pub(crate) trait ServiceImpl: Service {
    async fn handler_unimplemented(&self, msg: MsgSendDirectReq2) -> Result<MsgSendDirectResp2> {
        error!(
            "MsgSendDirectReq2 is unimplemented in {}: {:?}",
            self.service_name(),
            msg
        );
        Err(odp_ffa::Error::UnexpectedFunctionId(FunctionId::MsgSendDirectReq2))
    }
}

impl<T: Service + ?Sized> ServiceImpl for T {}
