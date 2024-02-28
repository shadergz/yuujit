use std::fmt;

use super::opcode::Opcode;

pub struct Block {
    pub opcodes: Vec<Opcode>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "block:")?;
        for opcode in &self.opcodes {
            writeln!(f, "  {}", opcode)?;
        }

        Ok(())
    }
}