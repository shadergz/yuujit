pub trait Bit {
    fn bit(&self, bit: usize) -> bool;
}

impl Bit for u32 {
    fn bit(&self, bit: usize) -> bool {
        (*self >> bit) & 0x1 != 0
    }
}

pub trait Bits {
    type T;

    fn bits(&self, start: usize, size: usize) -> Self::T;
}

impl Bits for u32 {
    type T = u32;

    fn bits(&self, start: usize, size: usize) -> Self::T {
        (self >> start) & !(0xffffffff << size)
    }
}