use core::ptr;
use ffa::indirect::FfaIndirectMsg;
use ffa::memory::FfaMemory;
use ffa::msg::FfaMsg;
use ffa::notify::FfaNotify;
use ffa::FfaError;
use ffa::FfaFunctionId;

pub type Result<T> = core::result::Result<T, ffa::FfaError>;

// Protocol CMD definitions for FwMgmt
const EC_CAP_INDIRECT_MSG: u8 = 0x0;
const EC_CAP_GET_FW_STATE: u8 = 0x1;
const EC_CAP_GET_SVC_LIST: u8 = 0x2;
const EC_CAP_GET_BID: u8 = 0x3;
const EC_CAP_TEST_NFY: u8 = 0x4;
const EC_CAP_MAP_SHARE: u8 = 0x5;

#[derive(Default)]
struct FwStateRsp {
    _status: i64,
    _fw_version: u16,
    _secure_state: u8,
    _boot_status: u8,
}

#[derive(Default)]
struct ServiceListRsp {
    _status: i64,
    _debug_mask: u16,
    _battery_mask: u8,
    _fan_mask: u8,
    _thermal_mask: u8,
    _hid_mask: u8,
    _key_mask: u16,
}

#[derive(Default)]
struct GetBidRsp {
    _status: i64,
    _bid: u64,
}

#[derive(Default)]
struct GenericRsp {
    _status: i64,
}

#[derive(Default)]
pub struct FwMgmt {}

impl FwMgmt {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_fw_state(&self) -> FwStateRsp {
        FwStateRsp {
            _status: 0x0,
            _fw_version: 0x0100,
            _secure_state: 0x0,
            _boot_status: 0x1,
        }
    }

    fn get_svc_list(&self) -> ServiceListRsp {
        ServiceListRsp {
            _status: 0x0,
            _debug_mask: 0x1,
            _battery_mask: 0x1,
            _fan_mask: 0x1,
            _thermal_mask: 0x1,
            _hid_mask: 0x0,
            _key_mask: 0x7,
        }
    }

    fn get_bid(&self) -> GetBidRsp {
        GetBidRsp {
            _status: 0x0,
            _bid: 0xdead0001,
        }
    }

    unsafe fn read_memory(&self, address: u64) -> u64 {
        ptr::read_volatile(address as *const u64)
    }

    unsafe fn write_memory(&self, address: u64, value: u64) {
        ptr::write_volatile(address as *mut u64, value)
    }

    fn map_share(&self, address: u64, length: u64) -> GenericRsp {
        let mut _sts = FfaError::Ok.into();

        // Make sure the address passed in points to our TX_BUFFER_BASE
        if address == super::TX_BUFFER_BASE {
            let mut ffamem = FfaMemory::new();
            ffamem.set_rxtx_buffers(super::RX_BUFFER_BASE, super::TX_BUFFER_BASE);
            let result = ffamem.retrieve_req(address, length);

            match result {
                Ok(_params) => {
                    _sts = FfaError::Ok.into();
                }
                Err(e) => {
                    _sts = e.into();
                }
            }
            let msg = FfaIndirectMsg::new();
            unsafe {
                msg.init_indirect_msg(super::SMEM_TX_BUFFER, 0x1000);
                let value = self.read_memory(super::SMEM_BUFFER_BASE);
                println!("Value from SHARE_MEM_BUFFER: 0x{:08x}", value);
            };
        } else {
            println!("Memory share request passed invalid location");
            _sts = FfaError::InvalidParameters.into();
        }

        GenericRsp { _status: _sts }
    }

    #[cfg(debug_assertions)]
    fn test_notify(&self, msg: &FfaMsg) -> GenericRsp {
        // Trigger Notification Event for testing
        unsafe {
            self.write_memory(super::SMEM_BUFFER_BASE, 0x12345678);
            println!(
                "SMEM_BUFFER_BASE at 0x{:08x} value: 0x{:08x}",
                super::SMEM_BUFFER_BASE,
                self.read_memory(super::SMEM_BUFFER_BASE)
            );
        }

        let nfy = FfaNotify {
            function_id: FfaFunctionId::FfaNotificationSet.into(),
            source_id: msg.destination_id,
            destination_id: msg.source_id,
            args64: [
                0x2, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
        };

        let _result = nfy.exec();

        // Return status success
        GenericRsp { _status: 0x0 }
    }

    fn process_indirect(&self, seq_num: u16) -> GenericRsp {
        println!("Processing indirect message: 0x{:x}", seq_num);
        let msg = FfaIndirectMsg::new();
        let mut in_buf: [u8; 256] = [0; 256];
        let mut status;

        unsafe {
            status = msg.read_indirect_msg(super::SMEM_RX_BUFFER, seq_num, &mut in_buf);
        };

        if status == FfaError::Ok {
            println!("Indirect Message: {:?}", in_buf);
        }

        // Populate TX buffer with response and matching seq num
        let buf: [u8; 16] = [
            0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
        ];
        unsafe {
            status = msg.write_indirect_msg(super::SMEM_TX_BUFFER, seq_num, &buf);
        };

        GenericRsp {
            _status: status.into(),
        }
    }

    fn ffa_msg_send_direct_req2(&self, msg: &FfaMsg) -> Result<FfaMsg> {
        let cmd = msg.extract_u8_at_index(0);
        println!("Received FwMgmt command 0x{:x}", cmd);

        // Create new generic rsp packet swap destination and source
        let mut rsp = FfaMsg {
            function_id: FfaFunctionId::FfaMsgSendDirectResp2.into(),
            source_id: msg.destination_id,
            destination_id: msg.source_id,
            uuid: msg.uuid,
            ..Default::default()
        };
        match cmd {
            EC_CAP_INDIRECT_MSG => {
                rsp.struct_to_args64(&self.process_indirect((msg.args64[0] >> 8) as u16));
                Ok(rsp)
            }
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
            #[cfg(debug_assertions)]
            EC_CAP_TEST_NFY => {
                rsp.struct_to_args64(&self.test_notify(msg));
                Ok(rsp)
            }
            EC_CAP_MAP_SHARE => {
                // First parameter is pointer to memory descriptor
                rsp.struct_to_args64(&self.map_share(msg.args64[1], msg.args64[2]));
                Ok(rsp)
            }
            _ => {
                println!("Unknown FwMgmt Command: {}", cmd);
                Err(FfaError::InvalidParameters)
            }
        }
    }

    // Handles messages sent to the EC Firmware management service
    pub(crate) fn exec(self, msg: &FfaMsg) -> Result<FfaMsg> {
        let id = FfaFunctionId::from(msg.function_id);

        match id {
            FfaFunctionId::FfaMsgSendDirectReq2 => self.ffa_msg_send_direct_req2(msg),
            _ => {
                println!("Unhandled FfaFunctionId in FwMgmt: {:?}", id);
                Err(FfaError::InvalidParameters)
            }
        }
    }
}
