pub fn combine_low_high_u32(low: u32, high: u32) -> u64 {
    ((high as u64) << 32) | (low as u64)
}

pub fn combine_low_high_u16(low: u16, high: u16) -> u64 {
    ((high as u64) << 16) | (low as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_low_high_u32() {
        assert_eq!(combine_low_high_u32(0x12345678, 0x9ABCDEF0), 0x9ABCDEF012345678);
    }
}
