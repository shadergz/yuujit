struct Decoder {
    
}

impl Decoder {
    const fn build_pattern_mask(pattern: &'static str) -> u32 {
        let mut mask = 0;
        let bytes = pattern.as_bytes();
        let len = bytes.len();
        let mut i = 0;
        while i < len {
            let ch = bytes[i] as char;
            if ch == '0' || ch == '1' {
                mask |= 1 << (len - i - 1);
            }
            
            i += 1;
        }
    
        mask
    }
    
    const fn build_pattern_value(pattern: &'static str) -> u32 {
        let mut mask = 0;
        let bytes = pattern.as_bytes();
        let len = bytes.len();
        let mut i = 0;
        while i < len {
            let ch = bytes[i] as char;
            if ch == '1' {
                mask |= 1 << (len - i - 1);
            }
            
            i += 1;
        }
    
        mask
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_pattern_mask_all_ones() {
        assert_eq!(Decoder::build_pattern_mask("10110100101101011010100101101010"), 0xffffffff);
    }

    #[test]
    fn test_build_pattern_mask_all_zeroes() {
        assert_eq!(Decoder::build_pattern_mask("................................"), 0x00000000);
    }

    #[test]
    fn test_build_pattern_mask_mixed() {
        assert_eq!(Decoder::build_pattern_mask("10101010..1...0."), 0b1111111100100010);
        assert_eq!(Decoder::build_pattern_mask("101.1011.101.1011.101..101101.10"), 0b11101111011101111011100111111011);
    }

    #[test]
    fn test_build_pattern_value_all_ones() {
        assert_eq!(Decoder::build_pattern_value("11111111111111111111111111111111"), 0xffffffff);
    }

    #[test]
    fn test_build_pattern_value_all_zeroes() {
        assert_eq!(Decoder::build_pattern_value("0...0.........0........0......0."), 0x00000000);
    }

    #[test]
    fn test_build_pattern_value_mixed() {
        assert_eq!(Decoder::build_pattern_value("10101010..1...0."), 0b1010101000100000);
        assert_eq!(Decoder::build_pattern_value("101.1011.101.1011.101..101101.10"), 0b10101011010101011010100101101010);
    }
}