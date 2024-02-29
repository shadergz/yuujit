use crate::block_descriptor::BlockDescriptor;

use super::{block::Block, opcode::{Add, AddWithFlags, Copy, LoadFlags, LoadGpr, Opcode, StoreFlags, StoreGpr}, value::{Type, Value, U1, U32}};

pub struct Emitter {
    opcodes: Vec<Opcode>,
    current_var: u32,
    descriptor: BlockDescriptor,
}

const FLAG_N: u32 = 1 << 31;
const FLAG_Z: u32 = 1 << 30;
const FLAG_C: u32 = 1 << 29;
const FLAG_V: u32 = 1 << 28;

impl Emitter {
    pub fn new(descriptor: BlockDescriptor) -> Self {
        Self {
            opcodes: Vec::new(),
            current_var: 0,
            descriptor,
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

    pub fn store_nzc(&mut self, src: Value<U32>, carry: Value<U32>) {
        self.opcodes.push(Opcode::StoreFlags(StoreFlags {
            src: Some(src),
            carry: Some(carry),
            overflow: None,
        }));
    }

    pub fn store_nzcv(&mut self, src: Value<U32>, carry: Value<U32>, overflow: Value<U32>) {
        self.opcodes.push(Opcode::StoreFlags(StoreFlags {
            src: Some(src),
            carry: Some(carry),
            overflow: Some(overflow),
        }));
    }

    pub fn load_c(&mut self) -> Value<U32> {
        let dst = self.create_var();
        self.opcodes.push(Opcode::LoadFlags(LoadFlags {
            dst,
            mask: Value::from_imm(FLAG_C),
        }));

        dst
    }

    pub fn load_gpr(&mut self, src: u8) -> Value<U32> {
        let dst = self.create_var();
        self.opcodes.push(Opcode::LoadGpr(LoadGpr {
            dst,
            src,
        }));

        dst
    }

    pub fn store_gpr(&mut self, dst: u8, src: Value<U32>) {
        self.opcodes.push(Opcode::StoreGpr(StoreGpr {
            dst,
            src,
        }));
    }

    pub fn add_with_flags(&mut self, lhs: Value<U32>, rhs: Value<U32>) -> (Value<U32>, Value<U32>, Value<U32>) {
        let dst = self.create_var();
        let carry = self.create_var();
        let overflow = self.create_var();
        self.opcodes.push(Opcode::AddWithFlags(AddWithFlags {
            dst,
            carry,
            overflow,
            lhs,
            rhs,
        }));

        (dst, carry, overflow)
    }

    pub fn add(&mut self, lhs: Value<U32>, rhs: Value<U32>) -> Value<U32> {
        let dst = self.create_var();
        self.opcodes.push(Opcode::Add(Add {
            dst,
            lhs,
            rhs,
        }));

        dst
    }

    pub fn advance_pc(&mut self) {
        self.descriptor.advance();
        self.store_gpr(15, Value::from_imm(self.descriptor.addr()));
    }
}