#![no_std]
#![no_main]
#![feature(format_args_nl)]

#[macro_use]
extern crate ffa;

use core::arch::global_asm;
use ffa::msg::FfaMsg;
use ffa::rxtx::FfaRxTxMsg;
use ffa::Ffa;
use ffa::FfaError;
use uuid::{uuid, Uuid};

mod exception;
mod fw_mgmt;
mod notify;
mod panic;
mod thermal;

pub type Result<T> = core::result::Result<T, ffa::FfaError>;

// Constants used by several modules
const TX_BUFFER_BASE: u64 = 0x100600A0000;
const RX_BUFFER_BASE: u64 = 0x100600B0000;
const RXTX_PAGE_COUNT: u32 = 1;
const SMEM_BUFFER_BASE: u64 = 0x10060000000;
const SMEM_RX_BUFFER: u64 = 0x10060000000;
const SMEM_TX_BUFFER: u64 = 0x10060001000;

const UUID_EC_SVC_NOTIFY: Uuid = uuid!("B510B3A3-59F6-4054-BA7A-FF2EB1EAC765");
const UUID_EC_SVC_MANAGEMENT: Uuid = uuid!("330c1273-fde5-4757-9819-5b6539037502");
const UUID_EC_SVC_POWER: Uuid = uuid!("7157addf-2fbe-4c63-ae95-efac16e3b01c");
const UUID_EC_SVC_BATTERY: Uuid = uuid!("25cb5207-ac36-427d-aaef-3aa78877d27e");
const UUID_EC_SVC_THERMAL: Uuid = uuid!("31f56da7-593c-4d72-a4b3-8fc7171ac073");
const UUID_EC_SVC_UCSI: Uuid = uuid!("65467f50-827f-4e4f-8770-dbf4c3f77f45");
const UUID_EC_SVC_TIME_ALARM: Uuid = uuid!("23ea63ed-b593-46ea-b027-8924df88e92f");
const UUID_EC_SVC_DEBUG: Uuid = uuid!("0bd66c7c-a288-48a6-afc8-e2200c03eb62");
const UUID_EC_SVC_OEM: Uuid = uuid!("9a8a1e88-a880-447c-830d-6d764e9172bb");

global_asm!(
    include_str!("start.s"),
    CONST_CORE_ID_MASK = const 0b11,
    CONST_CURRENTEL_EL1 = const 0x04,
);

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;

fn ffa_msg_handler(msg: &FfaMsg) -> Result<FfaMsg> {
    println!(
        r#"Successfully received ffa msg:
        function_id = {:08x}
               uuid = {}"#,
        msg.function_id, msg.uuid
    );

    match msg.uuid {
        UUID_EC_SVC_MANAGEMENT => {
            let fwmgmt = fw_mgmt::FwMgmt::new();
            fwmgmt.exec(msg)
        }
        UUID_EC_SVC_NOTIFY => {
            let ntfy = notify::Notify::new();
            ntfy.exec(msg)
        }
        UUID_EC_SVC_POWER => unimplemented!(),
        UUID_EC_SVC_BATTERY => unimplemented!(),
        UUID_EC_SVC_THERMAL => {
            let thm = thermal::ThmMgmt::new();
            thm.exec(msg)
        }
        UUID_EC_SVC_UCSI => unimplemented!(),
        UUID_EC_SVC_TIME_ALARM => unimplemented!(),
        UUID_EC_SVC_DEBUG => unimplemented!(),
        UUID_EC_SVC_OEM => unimplemented!(),
        _ => {
            println!("Unknown UUID {}", msg.uuid);
            Err(FfaError::InvalidParameters)
        }
    }
}

#[no_mangle]
pub extern "C" fn sp_main(_sp_params: u64) -> ! {
    let el = exception::ExceptionLevel::current();

    println!("Hello from {}", el);

    // Get current FFA version
    let ffa = Ffa::new();

    // Call the msg_wait method
    match ffa.version() {
        Ok(ver) => println!("FFA Version: {}.{}", ver.major(), ver.minor()),
        Err(_e) => {
            // This is fatal, terminate SP
            println!("FFA Version failed")
        }
    }

    // Map in shared RX/TX buffers
    println!(
        "Mapping shared RX/TX buffers:
               TX_BUFFER_BASE: 0x{:x}
               RX_BUFFER_BASE: 0x{:x}
               RXTX_PAGE_COUNT: 0x{:x}",
        TX_BUFFER_BASE, RX_BUFFER_BASE, RXTX_PAGE_COUNT
    );

    let mut rxtx = FfaRxTxMsg::new();
    let result = rxtx.map(TX_BUFFER_BASE, RX_BUFFER_BASE, RXTX_PAGE_COUNT);
    match result {
        FfaError::Ok => println!("Successfully mapped RXTX buffers"),
        _ => {
            // This is fatal, terminate SP
            println!("Error mapping RXTX buffers")
        }
    }

    println!("Entering FFA message loop");

    // Call the msg_wait method
    let mut next_msg = ffa.msg_wait();
    loop {
        match next_msg {
            Ok(ffamsg) => match ffa_msg_handler(&ffamsg) {
                Ok(msg) => next_msg = ffa.msg_resp(&msg),
                Err(_e) => panic!("Failed to handle FFA msg"),
            },
            Err(_e) => {
                println!("Error executing msg_wait: {:?}", _e);
                next_msg = ffa.msg_wait();
            }
        }
    }
}
