pub trait Bit {
    fn bit(&self, bit: usize) -> bool;
}

impl Bit for u32 {
    fn bit(&self, bit: usize) -> bool {
        (*self >> bit) != 0
    }
}