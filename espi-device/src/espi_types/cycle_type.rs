pub trait CycleType: Into<u8> + TryFrom<u8> {
    fn encode(self) -> u8 {
        Into::<u8>::into(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Routing {
    Local,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionType {
    Middle,
    First,
    Last,
    Only,
}
impl CompletionType {
    pub fn encode(self) -> u8 {
        match self {
            CompletionType::Middle => 0b00,
            CompletionType::First => 0b01,
            CompletionType::Last => 0b10,
            CompletionType::Only => 0b11,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeripheralChannelCycleType {
    // Peripheral Channel
    MemRead32,
    MemRead64,
    MemWrite32,
    MemWrite64,
    Message(Routing),
    MessageWithData(Routing),
    SuccessfulCompletionWithoutData,
    SuccessfulCompletionWithData(CompletionType),
    UnsuccessfulCompletionWithoutData(CompletionType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutOfBandChannelCycleType {
    // Out-of-Band Channel
    OutOfBand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashChannelCycleType {
    // Flash Channel
    FlashRead,
    FlashWrite,
    FlashErase,
    SuccessfulCompletionWithoutData,
    SuccessfulCompletionWithData(CompletionType),
    UnsuccessfulCompletionWithoutData(CompletionType),
}

impl From<PeripheralChannelCycleType> for u8 {
    fn from(value: PeripheralChannelCycleType) -> Self {
        match value {
            PeripheralChannelCycleType::MemRead32 => 0b00000000,
            PeripheralChannelCycleType::MemRead64 => 0b00000010,
            PeripheralChannelCycleType::MemWrite32 => 0b00000001,
            PeripheralChannelCycleType::MemWrite64 => 0b00000011,
            PeripheralChannelCycleType::Message(Routing::Local) => 0b00010000,
            PeripheralChannelCycleType::MessageWithData(Routing::Local) => 0b00010001,
            PeripheralChannelCycleType::SuccessfulCompletionWithoutData => 0b00000110,
            PeripheralChannelCycleType::SuccessfulCompletionWithData(ct) => 0b00001001 | (ct.encode() << 1),
            PeripheralChannelCycleType::UnsuccessfulCompletionWithoutData(ct) => 0b00001000 | (ct.encode() << 1),
        }
    }
}

impl From<OutOfBandChannelCycleType> for u8 {
    fn from(value: OutOfBandChannelCycleType) -> Self {
        match value {
            OutOfBandChannelCycleType::OutOfBand => 0b00100001,
        }
    }
}

impl From<FlashChannelCycleType> for u8 {
    fn from(value: FlashChannelCycleType) -> Self {
        match value {
            FlashChannelCycleType::FlashRead => 0b00001011,
            FlashChannelCycleType::FlashWrite => 0b00001100,
            FlashChannelCycleType::FlashErase => 0b00001101,
            FlashChannelCycleType::SuccessfulCompletionWithoutData => 0b00000110,
            FlashChannelCycleType::SuccessfulCompletionWithData(ct) => 0b00001001 | (ct.encode() << 1),
            FlashChannelCycleType::UnsuccessfulCompletionWithoutData(ct) => 0b00001000 | (ct.encode() << 1),
        }
    }
}

impl TryFrom<u8> for PeripheralChannelCycleType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00000000 => Ok(PeripheralChannelCycleType::MemRead32),
            0b00000010 => Ok(PeripheralChannelCycleType::MemRead64),
            0b00000001 => Ok(PeripheralChannelCycleType::MemWrite32),
            0b00000011 => Ok(PeripheralChannelCycleType::MemWrite64),
            0b00010000 => Ok(PeripheralChannelCycleType::Message(Routing::Local)),
            0b00010001 => Ok(PeripheralChannelCycleType::MessageWithData(Routing::Local)),
            0b00000110 => Ok(PeripheralChannelCycleType::SuccessfulCompletionWithoutData),
            _ if value & 0b00001001 == 0b00001001 => Ok(PeripheralChannelCycleType::SuccessfulCompletionWithData(
                CompletionType::try_from((value & 0b00000110) >> 1)?,
            )),
            _ if value & 0b00001000 == 0b00001000 => Ok(PeripheralChannelCycleType::UnsuccessfulCompletionWithoutData(
                CompletionType::try_from((value & 0b00000110) >> 1)?,
            )),
            _ => Err("Invalid peripheral channel cycle type"),
        }
    }
}

impl TryFrom<u8> for OutOfBandChannelCycleType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00100001 => Ok(OutOfBandChannelCycleType::OutOfBand),
            _ => Err("Invalid out-of-band channel cycle type"),
        }
    }
}

impl TryFrom<u8> for FlashChannelCycleType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00001011 => Ok(FlashChannelCycleType::FlashRead),
            0b00001100 => Ok(FlashChannelCycleType::FlashWrite),
            0b00001101 => Ok(FlashChannelCycleType::FlashErase),
            0b00000110 => Ok(FlashChannelCycleType::SuccessfulCompletionWithoutData),
            _ if value & 0b00001001 == 0b00001001 => Ok(FlashChannelCycleType::SuccessfulCompletionWithData(
                CompletionType::try_from((value & 0b00000110) >> 1)?,
            )),
            _ if value & 0b00001000 == 0b00001000 => Ok(FlashChannelCycleType::UnsuccessfulCompletionWithoutData(
                CompletionType::try_from((value & 0b00000110) >> 1)?,
            )),
            _ => Err("Invalid flash channel cycle type"),
        }
    }
}

impl TryFrom<u8> for CompletionType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(CompletionType::Middle),
            0b01 => Ok(CompletionType::First),
            0b10 => Ok(CompletionType::Last),
            0b11 => Ok(CompletionType::Only),
            _ => Err("Invalid completion type"),
        }
    }
}

impl CycleType for PeripheralChannelCycleType {}
impl CycleType for OutOfBandChannelCycleType {}
impl CycleType for FlashChannelCycleType {}
