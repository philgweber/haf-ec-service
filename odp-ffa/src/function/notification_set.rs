use crate::{exec_simple, Error, Function, SmcParams};

#[derive(Clone, Debug, PartialEq)]
pub struct NotificationSet {
    sender_id: u16,
    receiver_id: u16,
    flags: u32,
    notification_bitmap: u64,
}

impl NotificationSet {
    pub fn new(sender_id: u16, receiver_id: u16, flags: u32, notification_bitmap: u64) -> Self {
        Self {
            sender_id,
            receiver_id,
            flags,
            notification_bitmap,
        }
    }
}

impl Function for NotificationSet {
    type ReturnType = ();

    const ID: crate::FunctionId = crate::FunctionId::NotificationSet;

    fn exec(self) -> crate::ExecResult<Self::ReturnType> {
        exec_simple(self, |_| Ok(()))
    }
}

impl TryInto<SmcParams> for NotificationSet {
    type Error = Error;

    fn try_into(self) -> Result<SmcParams, Self::Error> {
        let bitmap_low = self.notification_bitmap & 0xffffffff;
        let bitmap_high = self.notification_bitmap >> 32;
        Ok(SmcParams {
            x1: ((self.sender_id as u64) << 16) | (self.receiver_id as u64),
            x2: self.flags as u64,
            x3: bitmap_low,
            x4: bitmap_high,
            ..Default::default()
        })
    }
}

impl TryFrom<SmcParams> for NotificationSet {
    type Error = Error;

    fn try_from(value: SmcParams) -> Result<Self, Self::Error> {
        Ok(NotificationSet {
            sender_id: (value.x1 >> 16) as u16,
            receiver_id: (value.x1 & 0xFFFF) as u16,
            flags: value.x2 as u32,
            notification_bitmap: (value.x4 << 32) | (value.x3 & 0xFFFFFFFF),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::zero_values(0, 0, 0, 0)]
    #[case::max_ids_flags(u16::MAX, u16::MAX, u32::MAX, 0)]
    #[case::max_bitmap(1, 2, 3, u64::MAX)]
    #[case::typical_values(0x12, 0x34, 0x56789ABC, 0xDEF0123456789ABC)]
    fn test_notification_set_round_trip(
        #[case] sender_id: u16,
        #[case] receiver_id: u16,
        #[case] flags: u32,
        #[case] bitmap: u64,
    ) {
        let original_ns = NotificationSet {
            sender_id,
            receiver_id,
            flags,
            notification_bitmap: bitmap,
        };

        let params: SmcParams = original_ns.clone().try_into().unwrap();
        let new_ns: NotificationSet = params.try_into().unwrap();

        assert_eq!(original_ns, new_ns);
    }
}
