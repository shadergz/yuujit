use crate::{block_descriptor::BlockDescriptor, ir::block::Block, state::State};

pub trait Backend {
    fn compile(&mut self, descriptor: BlockDescriptor, block: Block);
    fn run(&mut self, descriptor: BlockDescriptor, state: &mut State);
    fn has_code_at(&self, descriptor: BlockDescriptor) -> bool;
}