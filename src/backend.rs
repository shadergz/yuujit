use crate::{block_descriptor::BlockDescriptor, ir::block::Block};

pub trait Backend {
    fn compile(&mut self, block: Block);
    fn has_code_at(&self, descriptor: BlockDescriptor) -> bool;
}