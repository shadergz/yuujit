pub trait GuestMemory {
    fn read_byte(addr: u32) -> u8;
    fn read_half(addr: u32) -> u16;
    fn read_word(addr: u32) -> u32;
}