use std::collections::HashMap;

use crate::{backend::Backend, block_descriptor::BlockDescriptor, ir::block::Block};

struct CodeBlock {

}

pub struct IrInterpreter {
    // TODO: cache of block descriptor to vec of interpreter functions should go here
    code_cache: HashMap<BlockDescriptor, CodeBlock>,
}

impl IrInterpreter {
    pub fn new() -> Self {
        Self {
            code_cache: HashMap::new(),
        }
    }
}

impl Backend for IrInterpreter {
    fn compile(&mut self, block: Block) {
        todo!()
    }

    fn has_code_at(&self, descriptor: BlockDescriptor) -> bool {
        self.code_cache.contains_key(&descriptor)
    }
}