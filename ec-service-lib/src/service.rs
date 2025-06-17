use core::future::Future;

use log::error;
use odp_ffa::{FunctionId, MsgSendDirectReq2, MsgSendDirectResp2};
use uuid::Uuid;

use crate::async_msg_loop;

pub type Result<T> = core::result::Result<T, odp_ffa::Error>;

pub trait Service {
    fn service_name(&self) -> &'static str;
    fn service_uuid(&self) -> Uuid;

    fn ffa_msg_send_direct_req2(&mut self, msg: MsgSendDirectReq2) -> impl Future<Output = Result<MsgSendDirectResp2>> {
        async move { self.handler_unimplemented(msg).await }
    }
}

pub trait ServiceNodeHandler {
    fn handle(&mut self, msg: MsgSendDirectReq2) -> impl Future<Output = Result<MsgSendDirectResp2>>;
}

pub struct ServiceNode<This: Service, Next: ServiceNodeHandler> {
    service: This,
    next: Next,
}

impl<This: Service, Next: ServiceNodeHandler> ServiceNode<This, Next> {
    pub async fn run_message_loop(&mut self) -> Result<()> {
        async_msg_loop(async |msg| self.handle(msg).await).await
    }
}

pub struct ServiceNodeNone;
impl ServiceNodeHandler for ServiceNodeNone {
    async fn handle(&mut self, msg: MsgSendDirectReq2) -> Result<MsgSendDirectResp2> {
        error!("Unknown UUID {}", msg.uuid());
        Err(odp_ffa::Error::Other("Unknown UUID"))
    }
}

impl<S: Service, N: ServiceNodeHandler> ServiceNode<S, N> {
    pub fn new(service: S, next: N) -> Self {
        Self { service, next }
    }
}

impl<This: Service, Next: ServiceNodeHandler> ServiceNodeHandler for ServiceNode<This, Next> {
    async fn handle(&mut self, msg: MsgSendDirectReq2) -> Result<MsgSendDirectResp2> {
        if msg.uuid() == self.service.service_uuid() {
            self.service.ffa_msg_send_direct_req2(msg).await
        } else {
            self.next.handle(msg).await
        }
    }
}

#[macro_export]
macro_rules! service_list {
    ($service:expr, $($next:expr),+$(,)?) => {
        $crate::ServiceNode::new($service, $crate::service_list!($($next),*))
    };

    ($service:expr) => {
        $crate::ServiceNode::new($service, $crate::ServiceNodeNone)
    };

    () => {
        $crate::ServiceNodeNone
    };
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
