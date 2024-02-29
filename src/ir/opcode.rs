use std::fmt;

use crate::bits::Bit;

use super::value::{Value, U1, U32, U8};

pub type GuestReg = u8;

#[derive(Clone, Copy)]
pub enum Opcode {
    Copy(Copy),
    LoadFlags(LoadFlags),
    StoreFlags(StoreFlags),
    StoreGpr(StoreGpr),
}

#[derive(Clone, Copy)]
pub struct Copy {
    pub dst: Value<U32>,
    pub src: Value<U32>,
}

#[derive(Clone, Copy)]
pub struct LoadFlags {
    pub dst: Value<U32>,

    // TODO: do we need this?
    pub mask: Value<U32>,
}

#[derive(Clone, Copy)]
pub struct StoreFlags {
    // Optionally used for calculating n and z flags
    pub src: Option<Value<U32>>,

    // Optionally used for c flag
    pub carry: Option<Value<U32>>,

    // Optionally used for v flag
    pub overflow: Option<Value<U32>>,
}

#[derive(Clone, Copy)]
pub struct StoreGpr {
    pub dst: GuestReg,
    pub src: Value<U32>,
}

#[derive(Clone, Copy)]
pub struct LoadCpsr {
    pub dst: Value<U32>,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Copy(opcode) => write!(f, "{} = copy {}", opcode.dst, opcode.src),
            Opcode::LoadFlags(opcode) => {
                if let Value::Imm(imm, _) = opcode.mask {
                    let n = if imm.bit(31) { "n" } else { "" };
                    let z = if imm.bit(30) { "z" } else { "" };
                    let c = if imm.bit(29) { "c" } else { "" };
                    let v = if imm.bit(28) { "v" } else { "" };
                    write!(f, "{} = load.{}{}{}{}", opcode.dst, n, z, c, v)?;
                }
                
                Ok(())
            },
            Opcode::StoreFlags(opcode) => todo!(),
            Opcode::StoreGpr(opcode) => write!(f, "store_gpr r{}, {}", opcode.dst, opcode.src),
        }
    }
}