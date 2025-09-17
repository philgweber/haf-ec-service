#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

mod service;
pub mod services;
pub mod sp_logger;

use log::{error, info};
use odp_ffa::{Function, FunctionId, MsgSendDirectReq2, MsgSendDirectResp2, MsgWait, TryFromSmcCall};
pub use service::{Result, Service, ServiceNode, ServiceNodeHandler, ServiceNodeNone};
use uuid::{uuid, Uuid};

// For reference, here are the UUIDs for services that ec-service-lib defines (not all of them are implemented)
// const UUID_EC_SVC_NOTIFY: Uuid = uuid!("B510B3A3-59F6-4054-BA7A-FF2EB1EAC765");
// const UUID_EC_SVC_MANAGEMENT: Uuid = uuid!("330c1273-fde5-4757-9819-5b6539037502");
// const UUID_EC_SVC_POWER: Uuid = uuid!("7157addf-2fbe-4c63-ae95-efac16e3b01c");
// const UUID_EC_SVC_BATTERY: Uuid = uuid!("25cb5207-ac36-427d-aaef-3aa78877d27e");
// const UUID_EC_SVC_THERMAL: Uuid = uuid!("31f56da7-593c-4d72-a4b3-8fc7171ac073");
// const UUID_EC_SVC_UCSI: Uuid = uuid!("65467f50-827f-4e4f-8770-dbf4c3f77f45");
// const UUID_EC_SVC_TIME_ALARM: Uuid = uuid!("23ea63ed-b593-46ea-b027-8924df88e92f");
// const UUID_EC_SVC_DEBUG: Uuid = uuid!("0bd66c7c-a288-48a6-afc8-e2200c03eb62");
// const UUID_EC_SVC_OEM: Uuid = uuid!("9a8a1e88-a880-447c-830d-6d764e9172bb");

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum HafEcError {
    Ok,
    InvalidParameters,
}

async fn async_msg_loop(
    mut handler: impl AsyncFnMut(MsgSendDirectReq2) -> core::result::Result<MsgSendDirectResp2, odp_ffa::Error>,
    mut before_handle_message: impl AsyncFnMut(&MsgSendDirectReq2) -> core::result::Result<(), odp_ffa::Error>,
) -> core::result::Result<(), odp_ffa::Error> {
    info!("async_msg_loop: start");
    let mut msg = MsgWait::new().exec()?;
    info!("async_msg_loop: msg: {:?}", msg);
    loop {
        let mut sec_int = false;
        if msg.id == FunctionId::Interrupt {
          // Create new request to notify service for interrupt
          info!("Received interrupt");
          const NOTIFY_UUID: Uuid = uuid!("e474d87e-5731-4044-a727-cb3e8cf3c8df");
          let (uuid_high, uuid_low) = NOTIFY_UUID.as_u64_pair();
          msg.id = FunctionId::MsgSendDirectReq2;
          msg.params.x1 = 0x8003 << 16; // Sender/receiver ID ignored
          msg.params.x2 = uuid_high.to_be(); // High UUID bits for Notify service
          msg.params.x3 = uuid_low.to_be();  // Low UUID bits for Notify service
          msg.params.x4 = 0; // Source ID
          msg.params.x5 = 0; // Sender UUID High
          msg.params.x6 = 0; // Sender UUID Low
          msg.params.x7 = 0; // Receiver UUID High
          msg.params.x8 = 0; // Reciever UUID Low
          msg.params.x9 = 6; // Message Id
          msg.params.x10 = 0; // Count make sure it is zero
          msg.params.x11 = 0; // Zero out first notification message
          sec_int = true;
        }
        msg = if let Ok(request) = MsgSendDirectReq2::try_from_smc_call(msg.clone()) {
            info!("async_msg_loop: request: {:?}", request);
            before_handle_message(&request).await?;
            match handler(request).await {
                Ok(response) => {
                    if sec_int {
                      info!("normal_world_resume");
                      MsgWait::new().exec()?
                    } else {
                      info!("async_msg_loop: response: {:?}", response);
                      response.exec()?
                    }
                }
                Err(e) => {
                    error!("Error handling FFA message: {:?}", e);
                    MsgWait::new().exec()?
                }
            }
        } else {
            error!("Unexpected FFA message: {:?}", msg);
            MsgWait::new().exec()?
        }
    }
}
