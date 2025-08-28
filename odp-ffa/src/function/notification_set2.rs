use crate::{exec_simple, Error, Function, SmcParams};

#[derive(Clone, Debug, PartialEq)]
pub struct NotificationSet2 {
    sender_id: u16,
    receiver_id: u16,
    flags: u64,
    notification_bitmap: [u64;6],
}

impl NotificationSet2 {
    pub fn new(sender_id: u16, receiver_id: u16, flags: u64, notification_bitmap: [u64;6]) -> Self {
        Self {
            sender_id,
            receiver_id,
            flags,
            notification_bitmap,
        }
    }
}

impl Function for NotificationSet2 {
    type ReturnType = ();

    const ID: crate::FunctionId = crate::FunctionId::NotificationSet2;

    fn exec(self) -> crate::ExecResult<Self::ReturnType> {
        exec_simple(self, |_| Ok(()))
    }
}

impl TryInto<SmcParams> for NotificationSet2 {
    type Error = Error;

    fn try_into(self) -> Result<SmcParams, Self::Error> {
        Ok(SmcParams {
            x1: ((self.sender_id as u64) << 16) | (self.receiver_id as u64),
            x2: self.flags as u64,
            x3: self.notification_bitmap[0],
            x4: self.notification_bitmap[1],
            x5: self.notification_bitmap[2],
            x6: self.notification_bitmap[3],
            x7: self.notification_bitmap[4],
            x8: self.notification_bitmap[5],
            ..Default::default()
        })
    }
}

impl TryFrom<SmcParams> for NotificationSet2 {
    type Error = Error;

    fn try_from(value: SmcParams) -> Result<Self, Self::Error> {
        Ok(NotificationSet2 {
            sender_id: (value.x1 >> 16) as u16,
            receiver_id: (value.x1 & 0xFFFF) as u16,
            flags: value.x2,
            notification_bitmap: [value.x3,value.x4,value.x5,value.x6,value.x7,value.x8],
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
    #[case::typical_values(0x12, 0x34, 0x56789ABC, [0,1,2,3,4,5])]
    fn test_notification_set_round_trip(
        #[case] sender_id: u16,
        #[case] receiver_id: u16,
        #[case] flags: u64,
        #[case] bitmap: [u64;6],
    ) {
        let original_ns = NotificationSet2 {
            sender_id,
            receiver_id,
            flags,
            notification_bitmap: bitmap,
        };

        let params: SmcParams = original_ns.clone().try_into().unwrap();
        let new_ns: NotificationSet2 = params.try_into().unwrap();

        assert_eq!(original_ns, new_ns);
    }
}
