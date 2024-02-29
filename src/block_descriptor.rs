use crate::state::State;

// Any block can be identified by the pc, mode bits and thumb bit.
// To encode this efficiently we use a u64.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockDescriptor(u64);

impl BlockDescriptor {
    pub fn addr(&self) -> u32 {
        (self.0 as u32 & 0x7fffffff) << 1
    }

    pub fn is_arm(&self) -> bool {
        (self.0 >> 36) & 0x1 == 0
    }

    pub fn advance(self) -> BlockDescriptor {
        let step = if self.is_arm() { 4 } else { 2 };
        let addr = self.addr() + step;
        let value = (self.0 & !0x7fffffff) | (addr >> 1) as u64;
        BlockDescriptor(value)
    }
}

impl From<&State> for BlockDescriptor {
    fn from(state: &State) -> Self {
        let mut value = 0;

        // Since every pc value is at least always aligned to 2 bytes, we can discard
        // the first bit.
        value |= (state.gpr[15] >> 1) as u64;

        // Store the mode bits and thumb bit.
        // value |= u3:state.cpsr.mode
        value |= (u32::from(state.cpsr.mode) as u64) << 31;
        value |= (state.cpsr.t as u64) << 36;

        BlockDescriptor(value)
    }
}