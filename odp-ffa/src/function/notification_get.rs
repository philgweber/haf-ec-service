use crate::{exec_simple, util::combine_low_high_u32, Error, ExecResult, Function, FunctionId, SmcParams};

#[derive(Clone, Debug, PartialEq)]
pub struct NotificationGet {
    pub receiver_cpu_id: u16,
    pub receiver_endpoint_id: u16,
    pub flags: u32,
}

pub struct Response {
    pub sp_notifications_bitmap: u64,
    pub vm_notifications_bitmap: u64,
    pub fw_notifications_bitmap: u64,
}

impl NotificationGet {
    pub fn new(receiver_cpu_id: u16, receiver_endpoint_id: u16, flags: u32) -> Self {
        Self {
            receiver_cpu_id,
            receiver_endpoint_id,
            flags,
        }
    }
}

impl Function for NotificationGet {
    const ID: FunctionId = FunctionId::NotificationGet;
    type ReturnType = Response;

    fn exec(self) -> ExecResult<Self::ReturnType> {
        exec_simple(self, |result| {
            Ok(Response {
                sp_notifications_bitmap: combine_low_high_u32(result.params.x2 as u32, result.params.x3 as u32),
                vm_notifications_bitmap: combine_low_high_u32(result.params.x4 as u32, result.params.x5 as u32),
                fw_notifications_bitmap: combine_low_high_u32(result.params.x6 as u32, result.params.x7 as u32),
            })
        })
    }
}

impl TryInto<SmcParams> for NotificationGet {
    type Error = Error;

    fn try_into(self) -> Result<SmcParams, Self::Error> {
        Ok(SmcParams {
            x1: ((self.receiver_cpu_id as u64) << 16) | (self.receiver_endpoint_id as u64),
            x2: self.flags as u64,
            ..Default::default()
        })
    }
}

impl TryFrom<SmcParams> for NotificationGet {
    type Error = Error;

    fn try_from(value: SmcParams) -> Result<Self, Self::Error> {
        Ok(NotificationGet {
            receiver_cpu_id: (value.x1 >> 16) as u16,
            receiver_endpoint_id: (value.x1 & 0xFFFF) as u16,
            flags: value.x2 as u32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::zero_values(0, 0, 0)]
    #[case::max_u16_ids(u16::MAX, u16::MAX, 0)]
    #[case::max_flags(123, 456, u32::MAX)]
    #[case::typical_values(0x12, 0x34, 0x56789ABC)]
    fn test_notification_get_round_trip(#[case] cpu_id: u16, #[case] endpoint_id: u16, #[case] flags: u32) {
        let original_ng = NotificationGet {
            receiver_cpu_id: cpu_id,
            receiver_endpoint_id: endpoint_id,
            flags,
        };

        let params: SmcParams = original_ng.clone().try_into().unwrap();
        let new_ng: NotificationGet = params.try_into().unwrap();

        assert_eq!(original_ng, new_ng);
    }
}
