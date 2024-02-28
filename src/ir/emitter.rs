use super::{block::Block, opcode::{Copy, LoadFlags, Opcode, StoreFlags, StoreGpr}, value::{Type, Value, U1, U32}};

pub struct Emitter {
    opcodes: Vec<Opcode>,
    current_var: u32,
}

impl Emitter {
    pub fn new() -> Self {
        Self {
            opcodes: Vec::new(),
            current_var: 0,
        }
    }

    fn create_var<T: Type>(&mut self) -> Value<T> {
        let var = Value::from_var(self.current_var);
        self.current_var += 1;
        var
    }

    pub fn consume(self) -> Block {
        Block {
            opcodes: self.opcodes,
        }
    }

    pub fn copy(&mut self, src: Value<U32>) -> Value<U32> {
        let dst = self.create_var();
        self.opcodes.push(Opcode::Copy(Copy {
            dst,
            src,
        }));

        dst
    }

    pub fn store_nz(&mut self, src: Value<U32>) {
        self.opcodes.push(Opcode::StoreFlags(StoreFlags {
            src,
            mask: Value::from_imm(0xc0000000),
        }));
    }

    pub fn load_c(&mut self) -> Value<U32> {
        let dst = self.create_var();
        self.opcodes.push(Opcode::LoadFlags(LoadFlags {
            dst,
            mask: Value::from_imm(0x20000000),
        }));

        dst
    }

    pub fn store_gpr(&mut self, dst: u8, src: Value<U32>) {
        self.opcodes.push(Opcode::StoreGpr(StoreGpr {
            dst,
            src,
        }));
    }
}