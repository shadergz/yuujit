use crate::state::State;

// Any block can be identified by the pc, mode bits and thumb bit.
// To encode this efficiently we use a u64.
pub struct BlockDescriptor(u64);

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