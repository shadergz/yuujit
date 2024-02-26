pub mod visitor;
pub mod instructions;

use crate::decoder::visitor::Visitor;

use self::instructions::{DataProcessing, ArmInstructionType};

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

pub struct Decoder {
    arm_table: [ArmInstructionType; 4096],
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            arm_table: Self::build_arm_table(),
        }
    }

    const fn build_arm_table() -> [ArmInstructionType; 4096] {
        let mut arm_table = [ArmInstructionType::Illegal; 4096];
        let mut i = 0;
        while i < arm_table.len() {
            if Self::matches_pattern(i, "....00..........................") {
                arm_table[i] = ArmInstructionType::DataProcessing;
            } else {
                arm_table[i] = ArmInstructionType::Illegal;
            }

            i += 1;
        }

        arm_table
    }

    const fn matches_pattern(table_index: usize, pattern: &'static str) -> bool {
        let mask = Self::arm_index(build_pattern_mask(pattern));
        let value = Self::arm_index(build_pattern_value(pattern));
        table_index & mask == value
    }

    pub fn call_arm<V: Visitor>(&self, inst: u32, visitor: &mut V) {
        println!("{:#08x}", inst);
        let index = Self::arm_index(inst);
        match self.arm_table[index] {
            ArmInstructionType::DataProcessing => visitor.arm_data_processing(DataProcessing::from(inst)),
            ArmInstructionType::Illegal => todo!(),
        }
    }

    const fn arm_index(inst: u32) -> usize {
        let index = ((inst >> 16) & 0xff0) | ((inst >> 4) & 0xf);
        index as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_pattern_mask_all_ones() {
        assert_eq!(build_pattern_mask("10110100101101011010100101101010"), 0xffffffff);
    }

    #[test]
    fn test_build_pattern_mask_all_zeroes() {
        assert_eq!(build_pattern_mask("................................"), 0x00000000);
    }

    #[test]
    fn test_build_pattern_mask_mixed() {
        assert_eq!(build_pattern_mask("10101010..1...0."), 0b1111111100100010);
        assert_eq!(build_pattern_mask("101.1011.101.1011.101..101101.10"), 0b11101111011101111011100111111011);
    }

    #[test]
    fn test_build_pattern_value_all_ones() {
        assert_eq!(build_pattern_value("11111111111111111111111111111111"), 0xffffffff);
    }

    #[test]
    fn test_build_pattern_value_all_zeroes() {
        assert_eq!(build_pattern_value("0...0.........0........0......0."), 0x00000000);
    }

    #[test]
    fn test_build_pattern_value_mixed() {
        assert_eq!(build_pattern_value("10101010..1...0."), 0b1010101000100000);
        assert_eq!(build_pattern_value("101.1011.101.1011.101..101101.10"), 0b10101011010101011010100101101010);
    }
}