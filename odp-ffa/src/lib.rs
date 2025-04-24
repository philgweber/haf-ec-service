//! ARM Firmware Framework for ARMv8-A Profile

#![doc(html_root_url = "https://docs.rs/ffa/latest")]
#![cfg_attr(not(test), no_std)]

#[macro_use]
mod function;
mod indirect_msg;
#[macro_use]
mod smc;
mod util;

pub use function::*;
use smc::*;

/// Convert an SmcCall into a Function
/// Blanket implementation for all functions
pub trait TryFromSmcCall<F: Function> {
    fn try_from_smc_call(smc_call: SmcCall) -> Result<F, Error>;
}

pub type ExecResult<T> = Result<T, Error>;

#[allow(private_bounds)]
pub trait Function: TryInto<SmcParams, Error = Error> {
    type ReturnType;
    const ID: FunctionId;

    fn exec(self) -> ExecResult<Self::ReturnType>;
}

fn exec_simple<T, Func: Function<ReturnType = T>>(
    function: Func,
    on_success: impl FnOnce(SmcCall) -> ExecResult<T>,
) -> ExecResult<T> {
    let result: SmcCall = ffa_smc(function)?.try_into()?;
    handle_result_simple(result, on_success)
}

fn handle_result_simple<T>(result: SmcCall, on_success: impl FnOnce(SmcCall) -> ExecResult<T>) -> ExecResult<T> {
    match result.id {
        FunctionId::Success32 | FunctionId::Success64 => Ok(on_success(result)?),
        FunctionId::Error => Err(Error::ErrorCode(try_parse_error_code(result.params.x2)?)),
        _ => Err(Error::UnexpectedFunctionId(result.id)),
    }
}

impl<F: Function + TryFrom<SmcParams, Error = Error>> TryFromSmcCall<F> for F {
    fn try_from_smc_call(smc_call: SmcCall) -> Result<F, Error> {
        if smc_call.id == F::ID {
            smc_call.params.try_into()
        } else {
            Err(Error::UnexpectedFunctionId(smc_call.id))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidFunctionId(u64),
    UnexpectedFunctionId(FunctionId),
    InvalidErrorCode(i64),
    ErrorCode(ErrorCode),
    TooManySmcParams,
    Other(&'static str),
}

impl From<ErrorCode> for Error {
    fn from(value: ErrorCode) -> Self {
        Error::ErrorCode(value)
    }
}

fn try_parse_function_id(func_id: u64) -> Result<FunctionId, Error> {
    FunctionId::try_from(func_id).map_err(Error::InvalidFunctionId)
}

fn try_parse_error_code(err: u64) -> Result<ErrorCode, Error> {
    ErrorCode::try_from(err).map_err(Error::InvalidErrorCode)
}

macro_rules! define_safe_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident: $type:ty {
        $(
            $variant:ident = $value:expr,
        )*
    }) => {
        $(#[$meta])*
        #[repr($type)]
        $vis enum $name {
            $(
                $variant = $value,
            )*
        }

        impl From<$name> for $type {
            fn from(value: $name) -> Self {
                value as $type
            }
        }

        impl TryFrom<$type> for $name {
            type Error = $type;

            fn try_from(value: $type) -> core::result::Result<Self, $type> {
                Ok(match value {
                    $($value => $name::$variant,)*
                    _ => return Err(value),
                })
            }
        }
    };
}

define_safe_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ErrorCode: i64 {
        Ok = 0,
        NotSupported = -1,
        InvalidParameters = -2,
        NoMemory = -3,
        Busy = -4,
        Interrupted = -5,
        Denied = -6,
        Retry = -7,
        Aborted = -8,
        NoData = -9,
        NotReady = -10,
    }
}

impl TryFrom<u64> for ErrorCode {
    type Error = i64;

    fn try_from(value: u64) -> core::result::Result<Self, Self::Error> {
        (value as i64).try_into()
    }
}

define_safe_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FunctionId: u64 {
        Error = 0x84000060,
        Success32 = 0x84000061,
        Success64 = 0xC4000061,
        Interrupt = 0x84000062,
        Version = 0x84000063,
        Features = 0x84000064,
        RxRelease = 0x84000065,
        RxTxMap = 0xC4000066,
        RxTxUnmap = 0x84000067,
        PartitionInfoGet = 0x84000068,
        IdGet = 0x84000069,
        MsgWait = 0x8400006B,
        MsgYield = 0x8400006C,
        MsgRun = 0x8400006D,
        MsgSend = 0x8400006E,
        MsgSendDirectReq = 0xC400006F,
        MsgSendDirectResp = 0xC4000070,
        MsgSend2 = 0x84000086,
        MsgPoll = 0x8400006A,
        MemDonate = 0xC4000071,
        MemLend = 0xC4000072,
        MemShare = 0xC4000073,
        MemRetrieveReq = 0x84000074,
        MemRetrieveResp = 0x84000075,
        MemRelinquish = 0x84000076,
        MemReclaim = 0x84000077,
        MemFragRx = 0x8400007A,
        MemFragTx = 0x8400007B,
        NotificationBind = 0x8400007F,
        NotificationSet = 0x84000081,
        NotificationGet = 0x84000082,
        MemPermGet = 0x84000088,
        MemPermSet = 0x84000089,
        ConsoleLog = 0xC400008A,
        MsgSendDirectReq2 = 0xC400008D,
        MsgSendDirectResp2 = 0xC400008E,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_id_conversion() {
        let id = FunctionId::ConsoleLog;
        let id_value: u64 = id.into();
        assert_eq!(id_value, 0xC400008A);
        let id_back = FunctionId::try_from(id_value).unwrap();
        assert_eq!(id, id_back);
    }
}
