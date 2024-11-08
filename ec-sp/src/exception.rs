use aarch64_cpu::registers::{CurrentEL::EL, *};
use core::{arch::global_asm, fmt::Display};

global_asm!(include_str!("exception.s"),);

#[derive(PartialEq, Eq, Debug)]
pub enum ExceptionLevel {
    EL0,
    EL1,
    EL2,
    EL3,
    Unknown,
}

impl ExceptionLevel {
    /// Reads current exception level from the Hardware and
    /// instantiates the correct enum variant.
    pub fn current() -> Self {
        match CurrentEL.read_as_enum::<EL::Value>(CurrentEL::EL) {
            Some(EL::Value::EL0) => ExceptionLevel::EL0,
            Some(EL::Value::EL1) => ExceptionLevel::EL1,
            Some(EL::Value::EL2) => ExceptionLevel::EL2,
            Some(EL::Value::EL3) => ExceptionLevel::EL3,
            _ => ExceptionLevel::Unknown,
        }
    }
}

impl Display for ExceptionLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ExceptionLevel::EL0 => write!(f, "EL0"),
            ExceptionLevel::EL1 => write!(f, "EL1"),
            ExceptionLevel::EL2 => write!(f, "EL2"),
            ExceptionLevel::EL3 => write!(f, "EL3"),
            ExceptionLevel::Unknown => write!(f, "Unknown"),
        }
    }
}
