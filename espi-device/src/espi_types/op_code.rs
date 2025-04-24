#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortOpLength {
    U8,
    U16,
    U32,
}
impl ShortOpLength {
    pub fn encode(self) -> u8 {
        match self {
            ShortOpLength::U8 => 0x01,
            ShortOpLength::U16 => 0x02,
            ShortOpLength::U32 => 0x03,
        }
    }
}

pub trait ShortOpData: Clone + Copy + PartialEq + Eq + Into<u32> + core::fmt::Debug {
    const OP_LENGTH: ShortOpLength;
    const BYTES: usize;
    fn try_from_u32(value: u32) -> Result<Self, &'static str>;
}

impl ShortOpData for u8 {
    const OP_LENGTH: ShortOpLength = ShortOpLength::U8;
    const BYTES: usize = 1;

    fn try_from_u32(value: u32) -> Result<Self, &'static str> {
        if value > u8::MAX as u32 {
            Err("ShortOpData::try_from_u32: value is too large to fit in a u8")
        } else {
            Ok(value as u8)
        }
    }
}

impl ShortOpData for u16 {
    const OP_LENGTH: ShortOpLength = ShortOpLength::U16;
    const BYTES: usize = 2;

    fn try_from_u32(value: u32) -> Result<Self, &'static str> {
        if value > u16::MAX as u32 {
            Err("ShortOpData::try_from_u32: value is too large to fit in a u16")
        } else {
            Ok(value as u16)
        }
    }
}
impl ShortOpData for u32 {
    const OP_LENGTH: ShortOpLength = ShortOpLength::U32;
    const BYTES: usize = 4;

    fn try_from_u32(value: u32) -> Result<Self, &'static str> {
        Ok(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EspiCommandOpCode {
    /// Put a posted or completion header and optional data
    PutPc,
    /// Get a posted or completion header and optional data
    GetPc,
    /// Put a non-posted header and optional data
    PutNp,
    /// Get a non-posted header and optional data
    GetNp,
    /// Put a Tunneled virtual wire packet
    PutVwire,
    /// Get a Tunneled virtual wire packet
    GetVwire,
    /// Put an OOB (Tunneled SMBus) message
    PutOob,
    /// Get an OOB (Tunneled SMBus) message
    GetOob,
    /// Put a Flash Access completion
    PutFlashC,
    /// Get a non-posted Flash Access request
    GetFlashNp,
    /// Put a non-posted Flash Access request
    PutFlashNp,
    /// Get a Flash Access completion
    GetFlashC,
    /// Command initiated by the master to read the status register of the slave
    GetStatus,
    /// Command to discover the capabilities of the slave as part of the initialization
    GetConfiguration,
    /// Command to set the capabilities of the slave as part of the initialization
    SetConfiguration,
    /// In-band RESET command
    InbandReset,
    /// Put a short (1, 2 or 4 bytes) non-posted I/O Read packet
    PutIoRdShort(ShortOpLength),
    /// Put a short (1, 2 or 4 bytes) non-posted I/O Write packet
    PutIoWrShort(ShortOpLength),
    /// Put a short (1, 2 or 4 bytes) non-posted Memory Read 32 packet
    PutMemRd32Short(ShortOpLength),
    /// Put a short (1, 2 or 4 bytes) posted Memory Write 32 packet
    PutMemWr32Short(ShortOpLength),
}

impl EspiCommandOpCode {
    pub fn encode(self) -> u8 {
        match self {
            EspiCommandOpCode::PutPc => 0x00,
            EspiCommandOpCode::GetPc => 0x01,
            EspiCommandOpCode::PutNp => 0x02,
            EspiCommandOpCode::GetNp => 0x03,
            EspiCommandOpCode::PutVwire => 0x04,
            EspiCommandOpCode::GetVwire => 0x05,
            EspiCommandOpCode::PutOob => 0x06,
            EspiCommandOpCode::GetOob => 0x07,
            EspiCommandOpCode::PutFlashC => 0x08,
            EspiCommandOpCode::GetFlashNp => 0x09,
            EspiCommandOpCode::PutFlashNp => 0x0A,
            EspiCommandOpCode::GetFlashC => 0x0B,
            EspiCommandOpCode::GetStatus => 0x25,
            EspiCommandOpCode::GetConfiguration => 0x21,
            EspiCommandOpCode::SetConfiguration => 0x22,
            EspiCommandOpCode::InbandReset => 0xFF,
            EspiCommandOpCode::PutIoRdShort(len) => 0x40 | len.encode(),
            EspiCommandOpCode::PutIoWrShort(len) => 0x44 | len.encode(),
            EspiCommandOpCode::PutMemRd32Short(len) => 0x48 | len.encode(),
            EspiCommandOpCode::PutMemWr32Short(len) => 0x4C | len.encode(),
        }
    }
}
