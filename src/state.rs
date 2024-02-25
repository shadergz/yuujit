#[derive(Clone, Copy)]
pub enum Mode {
    Usr,
    Fiq,
    Irq,
    Svc,
    Abt,
    Und,
    Sys,
}

impl From<Mode> for u32 {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Usr => 0x10,
            Mode::Fiq => 0x11,
            Mode::Irq => 0x12,
            Mode::Svc => 0x13,
            Mode::Abt => 0x17,
            Mode::Und => 0x1b,
            Mode::Sys => 0x1f,
        }
    }
}

pub struct StatusRegister {
    pub mode: Mode,
    pub t: bool,
    pub f: bool,
    pub i: bool,
    pub q: bool,
    pub v: bool,
    pub c: bool,
    pub z: bool,
    pub n: bool,
}

impl Default for StatusRegister {
    fn default() -> Self {
        Self {
            mode: Mode::Svc,
            t: false,
            f: true,
            i: true,
            q: false,
            v: false,
            c: false,
            z: false,
            n: false,
        }
    }
}

#[derive(Default)]
pub struct State {
    pub gpr: [u32; 32],
    pub cpsr: StatusRegister,
}