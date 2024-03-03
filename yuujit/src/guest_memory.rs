pub trait GuestMemory {
    fn read_byte(&self, addr: u32) -> u8;
    fn read_half(&self, addr: u32) -> u16;
    fn read_word(&self, addr: u32) -> u32;
}