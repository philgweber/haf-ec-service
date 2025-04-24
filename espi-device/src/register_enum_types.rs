bit_register! {
    /// eSPI flash sharing mode
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum FlashSharingMode: u8 {
        MasterAttached = 0b0,
        ShaveAttached = 0b1,
    }
}

bit_register! {
    /// eSPI I/O modes selection for the master device
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum IoMode: u32 {
        Single = 0b00,
        Dual = 0b01,
        Quad = 0b10,
    }
}

bit_register! {
    /// eSPI operating frequency
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Frequency: u32 {
        Freq20Mhz = 0b000,
        Freq25Mhz = 0b001,
        Freq33Mhz = 0b010,
        Freq50Mhz = 0b011,
        Freq66Mhz = 0b100,
    }
}

bit_register! {
    /// eSPI channel types
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum ChannelType: u32 {
    Peripheral = 0,
        VirtualWire = 1,
        OobMessage = 2,
        FlashAccess = 3,
    }
}

bit_register! {
    /// Maximum payload size
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MaxPayloadSize: u32 {
        Size64Bytes = 0b001,
        Size128Bytes = 0b010,
        Size256Bytes = 0b011,
    }
}

bit_register! {
    /// Maximum read request size
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MaxReadRequestSize: u32 {
        Size64Bytes = 0b001,
        Size128Bytes = 0b010,
        Size256Bytes = 0b011,
        Size512Bytes = 0b100,
        Size1024Bytes = 0b101,
        Size2048Bytes = 0b110,
        Size4096Bytes = 0b111,
    }
}

bit_register! {
    /// Flash erase block size
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum FlashEraseBlockSize: u32 {
        Size4KBytes = 0b001,
        Size64KBytes = 0b010,
        Size4KAnd64KBytes = 0b011,
        Size128KBytes = 0b100,
        Size256KBytes = 0b101,
    }
}

bit_register! {
    /// eSPI alert mode
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AlertMode: u32 {
        IoPin = 0,
        AlertPin = 1,
    }
}

bit_register! {
    /// Indicates which I/O modes are supported by the slave device
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum IoModeSupport: u32 {
        Single = 0b00,
        SingleDual = 0b01,
        SingleQuad = 0b10,
        SingleDualQuad = 0b11,
    }
}

bit_register! {
    /// eSPI max payload size support
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MaxPayloadSizeSupport: u32 {
        Size64Bytes = 0b001,
        Size128Bytes = 0b010,
        Size256Bytes = 0b011,
    }
}

impl MaxPayloadSizeSupport {
    pub fn supports(&self, max_payload_size: MaxPayloadSize) -> bool {
        match self {
            MaxPayloadSizeSupport::Size64Bytes => max_payload_size == MaxPayloadSize::Size64Bytes,
            MaxPayloadSizeSupport::Size128Bytes => {
                max_payload_size == MaxPayloadSize::Size64Bytes || max_payload_size == MaxPayloadSize::Size128Bytes
            }
            MaxPayloadSizeSupport::Size256Bytes => {
                max_payload_size == MaxPayloadSize::Size64Bytes
                    || max_payload_size == MaxPayloadSize::Size128Bytes
                    || max_payload_size == MaxPayloadSize::Size256Bytes
            }
        }
    }
}
