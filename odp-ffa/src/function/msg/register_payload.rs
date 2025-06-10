use core::ops::Range;

/// A payload of data for a direct message, transmitted in registers
#[derive(Debug, Clone, PartialEq)]
pub struct RegisterPayload([u8; 14 * 8]);

impl FromIterator<u8> for RegisterPayload {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut payload = [0u8; 14 * 8];
        for (i, byte) in iter.into_iter().enumerate() {
            payload[i] = byte;
        }
        Self(payload)
    }
}

pub trait Payload: Sized {
    fn u8_at(&self, byte_offset: usize) -> u8;
    fn u16_at(&self, byte_offset: usize) -> u16;
    fn u32_at(&self, byte_offset: usize) -> u32;
    fn u64_at(&self, byte_offset: usize) -> u64;
    fn register_at(&self, index: usize) -> u64;
    fn registers_iter(&self) -> impl Iterator<Item = u64>;
    fn slice(&self, range: core::ops::Range<usize>) -> &[u8];
}

impl Payload for RegisterPayload {
    fn u8_at(&self, byte_offset: usize) -> u8 {
        self.0[byte_offset]
    }

    fn u16_at(&self, byte_offset: usize) -> u16 {
        u16::from_le_bytes(self.0[byte_offset..byte_offset + 2].try_into().unwrap())
    }

    fn u32_at(&self, byte_offset: usize) -> u32 {
        u32::from_le_bytes(self.0[byte_offset..byte_offset + 4].try_into().unwrap())
    }

    fn u64_at(&self, byte_offset: usize) -> u64 {
        u64::from_le_bytes(self.0[byte_offset..byte_offset + 8].try_into().unwrap())
    }

    fn register_at(&self, index: usize) -> u64 {
        self.u64_at(index * 8)
    }

    fn registers_iter(&self) -> impl Iterator<Item = u64> {
        self.0
            .chunks_exact(8)
            .map(|slice| u64::from_le_bytes(slice.try_into().unwrap()))
    }

    fn slice(&self, range: Range<usize>) -> &[u8] {
        &self.0[range]
    }
}

pub trait HasRegisterPayload {
    fn payload(&self) -> &RegisterPayload;
}

impl<T: HasRegisterPayload> Payload for T {
    fn u8_at(&self, byte_offset: usize) -> u8 {
        self.payload().u8_at(byte_offset)
    }

    fn u16_at(&self, byte_offset: usize) -> u16 {
        self.payload().u16_at(byte_offset)
    }

    fn u32_at(&self, byte_offset: usize) -> u32 {
        self.payload().u32_at(byte_offset)
    }

    fn u64_at(&self, byte_offset: usize) -> u64 {
        self.payload().u64_at(byte_offset)
    }

    fn register_at(&self, index: usize) -> u64 {
        self.payload().register_at(index)
    }

    fn registers_iter(&self) -> impl Iterator<Item = u64> {
        self.payload().registers_iter()
    }

    fn slice(&self, range: Range<usize>) -> &[u8] {
        self.payload().slice(range)
    }
}

impl<Idx> core::ops::Index<Idx> for RegisterPayload
where
    Idx: core::slice::SliceIndex<[u8]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_iter_and_u8_at() {
        let data = (0..14 * 8).map(|i| i as u8);
        let payload = RegisterPayload::from_iter(data);
        for i in 0..(14 * 8) {
            assert_eq!(payload.u8_at(i), i as u8, "Byte at offset {} should be {}", i, i);
        }
    }

    #[test]
    fn test_u16_at() {
        let mut data = [0u8; 14 * 8];
        data[0] = 0x12;
        data[1] = 0x34;
        data[10] = 0x56;
        data[11] = 0x78;
        let payload = RegisterPayload(data);
        assert_eq!(payload.u16_at(0), 0x3412);
        assert_eq!(payload.u16_at(10), 0x7856);
    }

    #[test]
    fn test_u32_at() {
        let mut data = [0u8; 14 * 8];
        data[0] = 0x12;
        data[1] = 0x34;
        data[2] = 0x56;
        data[3] = 0x78;
        data[20] = 0x9A;
        data[21] = 0xBC;
        data[22] = 0xDE;
        data[23] = 0xF0;
        let payload = RegisterPayload(data);
        assert_eq!(payload.u32_at(0), 0x78563412);
        assert_eq!(payload.u32_at(20), 0xF0DEBC9A);
    }

    #[test]
    fn test_u64_at() {
        let mut data = [0u8; 14 * 8];
        data[0..8].copy_from_slice(&[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
        data[8..16].copy_from_slice(&[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11]);
        let payload = RegisterPayload(data);
        assert_eq!(payload.u64_at(0), 0x8877665544332211);
        assert_eq!(payload.u64_at(8), 0x1100FFEEDDCCBBAA);
    }

    #[test]
    fn test_register_at() {
        let mut data = [0u8; 14 * 8];
        // Set register 0
        data[0..8].copy_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
        // Set register 5
        let offset = 5 * 8;
        data[offset..offset + 8].copy_from_slice(&[0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8]);
        let payload = RegisterPayload(data);
        assert_eq!(payload.register_at(0), 0x0807060504030201);
        assert_eq!(payload.register_at(5), 0xA8A7A6A5A4A3A2A1);
    }

    #[test]
    fn test_index_trait() {
        let mut data = [0u8; 14 * 8];
        data[0] = 10;
        data[1] = 20;
        data[2] = 30;
        let payload = RegisterPayload(data);
        assert_eq!(payload[0], 10);
        assert_eq!(payload[1], 20);
        assert_eq!(payload[0..2], [10, 20]);
        assert_eq!(&payload[0..3], &[10, 20, 30u8]);
    }
}
