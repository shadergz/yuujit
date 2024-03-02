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

impl Mode {
    pub fn is_banked(&self) -> bool {
        !matches!(self, Self::Usr | Self::Sys)
    }
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

#[derive(Clone, Copy)]
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

impl From<StatusRegister> for u32 {
    fn from(value: StatusRegister) -> Self {
        let mut result = 0;
        result |= u32::from(value.mode);
        result |= (value.t as u32) << 5;
        result |= (value.f as u32) << 6;
        result |= (value.i as u32) << 7;
        result |= (value.q as u32) << 27;
        result |= (value.v as u32) << 28;
        result |= (value.c as u32) << 29;
        result |= (value.z as u32) << 30;
        result |= (value.n as u32) << 31;
        result
    }
}

#[derive(Default)]
pub struct State {
    pub gpr: [u32; 32],
    pub gpr_banked: [[u32; 7]; 6],
    pub cpsr: StatusRegister,
}