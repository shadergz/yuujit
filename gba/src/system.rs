use std::fs;

use yuujit::{config::Config, jit::Jit, state::Mode};

use crate::memory::Memory;

pub struct System {
    cpu: Jit<Memory>,
}

impl System {
    pub fn new(path: String) -> Self {
        let config = Config::default();
        let rom = fs::read(path).expect("file not found");
        let memory = Memory::new(rom);
        Self {
            cpu: Jit::new(config, memory),
        }
    }

    pub fn run_frame(&mut self) {
        for _ in 0..280896 {
            self.cpu.run(1);
        }
    }

    pub fn skip_bios(&mut self) {
        let mut cpsr = self.cpu.get_cpsr();
        cpsr.mode = Mode::Sys;
        self.cpu.set_cpsr(cpsr);

        self.cpu.set_gpr(13, 0x03007f00);
        self.cpu.set_banked_gpr(13, Mode::Irq, 0x03007fa0);
        self.cpu.set_banked_gpr(13, Mode::Svc, 0x03007fe0);
        self.cpu.set_gpr(15, 0x08000000);
    }
}