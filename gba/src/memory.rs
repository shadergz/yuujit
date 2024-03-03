use byteorder::{ByteOrder, LittleEndian};
use yuujit::guest_memory::GuestMemory;

pub struct Memory {
    rom: Vec<u8>,
}

impl Memory {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
        }
    }
}

impl GuestMemory for Memory {
    fn read_byte(&self, addr: u32) -> u8 {
        todo!()
    }

    fn read_half(&self, addr: u32) -> u16 {
        todo!()
    }

    fn read_word(&self, addr: u32) -> u32 {
        match addr {
            0x08000000..=0x0bffffff => LittleEndian::read_u32(&self.rom[(addr - 0x08000000) as usize..]),
            _ => todo!("addr: {:#010x}", addr),
        }
    }
}