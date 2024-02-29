use std::collections::HashMap;

use crate::{backend::Backend, block_descriptor::BlockDescriptor, ir::{block::Block, opcode::{Copy, Opcode}, value::{Type, Value, U32}}, state::State};

pub struct IrInterpreter {
    code_cache: HashMap<BlockDescriptor, Block>,
    variables: Vec<u32>,
}

impl IrInterpreter {
    pub fn new() -> Self {
        Self {
            code_cache: HashMap::new(),
            variables: Vec::new(),
        }
    }
}

impl IrInterpreter {
    fn assign(&mut self, dst: Value<U32>, src: u32) {
        if let Value::Var(id, _) = dst {
            if self.variables.len() <= id as usize {
                self.variables.push(src);
            } else {
                self.variables[id as usize] = src;
            }
        } else {
            panic!("destination shouldn't be an immediate")
        }
    }

    fn resolve<T: Type>(&self, src: Value<T>) -> u32 {
        match src {
            Value::Imm(imm, _) => imm,
            Value::Var(id, _) => self.variables[id as usize],
        }
    }
}

impl Backend for IrInterpreter {
    fn compile(&mut self, descriptor: BlockDescriptor, block: Block) {
        // No compilation is required as we just need to cache the ir block
        self.code_cache.insert(descriptor, block);
    }

    fn run(&mut self, descriptor: BlockDescriptor, state: &mut State) {
        // TODO: remove need to clone
        let block = self.code_cache.get(&descriptor).expect("block not compiled").clone();
        for opcode in block.opcodes {
            match opcode {
                Opcode::Copy(opcode) => {
                    let src = self.resolve(opcode.src);
                    self.assign(opcode.dst, src);
                },
                Opcode::LoadFlags(opcode) => {
                    let src = u32::from(state.cpsr);
                    self.assign(opcode.dst, src);
                },
                Opcode::StoreFlags(opcode) => todo!(),
                Opcode::StoreGpr(opcode) => {
                    let src = self.resolve(opcode.src);

                    // TODO: deal with modes
                    state.gpr[opcode.dst as usize] = src;
                },
            }
        }
    }

    fn has_code_at(&self, descriptor: BlockDescriptor) -> bool {
        self.code_cache.contains_key(&descriptor)
    }
}