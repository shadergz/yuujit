use crate::bits::Bit;

#[derive(Clone, Copy)]
pub enum ArmInstructionType {
    DataProcessing,
    Illegal,
}

pub struct ArmDataProcessing {
    pub set_flags: bool,
}

impl From<u32> for ArmDataProcessing {
    fn from(value: u32) -> Self {
        Self {
            set_flags: value.bit(20),
        }
    }
}