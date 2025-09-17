use crate::{exec_simple, Error, ExecResult, Function, SmcParams};
use crate::{FunctionId, SmcCall, MsgSendDirectReq2, Payload};
use zerocopy::{FromBytes, IntoBytes, KnownLayout, Immutable};

#[repr(C, packed)]
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Default, Clone, Copy, Debug)]
pub struct FfaMemTransDesc {
    pub sender_id: u16,
    pub memory_attributes: u16,
    pub flags: u32,
    pub handle: u64,
    pub tag: u64,
    pub memory_access_size: u32,
    pub memory_access_count: u32,
    pub memory_access_array_offset: u32,
    pub reserved: [u32; 3],
    pub memory_access_array: FfaMemAccessDesc,
}

#[repr(C, packed)]
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Default, Clone, Copy, Debug)]
pub struct FfaMemAccessDesc {
    pub memory_access_permissions: FfaMemPermDesc,
    pub composite_memory_region_offset: u32,
    pub reserved: u64,
}

#[repr(C, packed)]
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Default, Clone, Copy, Debug)]
pub struct FfaMemPermDesc {
    pub endpoint_id: u16, // Endpoint ID
    pub memory_access: u8, // Bitfield: Read (0x1), Write (0x2), Execute (0x4)
    pub flags: u8,      // Bitfield: Non-relinquishable (0x1), Time-bound, etc.
}

#[repr(C, packed)]
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Default, Clone, Copy, Debug)]
pub struct FfaCompositeMemRegionDesc {
    pub total_page_count: u32,     // Total number of 4KB pages across all constituents
    pub constituent_count: u32,    // Number of memory constituents
    pub reserved: u64,             // Reserved (must be zero)
    pub regions: FfaMemRegionConstituent,
}
#[repr(C, packed)]
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Default, Clone, Copy, Debug)]
pub struct FfaMemRegionConstituent {
    pub address: u64,              // Physical base address
    pub page_count: u32,           // Number of 4KB pages
    pub reserved: u32,             // Reserved (must be zero)
}


#[derive(Default, Clone, Debug)]
pub struct MemRetrieveReq { }

impl MemRetrieveReq {
    pub fn new() -> Self {
        Self{ }
    }
}

impl TryFrom<MsgSendDirectReq2> for FfaMemTransDesc {
    type Error = Error;

    fn try_from(msg: MsgSendDirectReq2) -> Result<Self, Self::Error> {
        let payload = msg.slice(24..88); 
        if payload.len() < core::mem::size_of::<FfaMemTransDesc>() {
            return Err(Error::Other("Invalid MemShare payload size"));
        }

        // SAFETY: zerocopy ensures alignment and layout correctness
        let mem_desc = FfaMemTransDesc::ref_from_bytes(payload).unwrap();
        Ok(*mem_desc)
    }
}

impl Function for MemRetrieveReq {
    type ReturnType = SmcCall;
    const ID: FunctionId = FunctionId::MemRetrieveReq;

    fn exec(self) -> ExecResult<Self::ReturnType> {
        exec_simple(self, Ok)
    }
}

impl TryInto<SmcParams> for MemRetrieveReq {
    type Error = Error;

    fn try_into(self) -> Result<SmcParams, Self::Error> {
        Ok(SmcParams {
            x1: core::mem::size_of::<FfaMemTransDesc>() as u64,
            x2: core::mem::size_of::<FfaMemTransDesc>() as u64,
            x3: 0,  // Hafnium only supports RX/TX buffers, so this is 0
            x4: 0,  // Must be zero when using RX/TX buffers
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_retrieve_req_round_trip() {

        let original_req = MemRetrieveReq {};

        let params: SmcParams = original_req.clone().try_into().unwrap();
        let new_req: MemRetrieveReq = params.try_into().unwrap();

        assert_eq!(original_req, new_req);
    }
}
