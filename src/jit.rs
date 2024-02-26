use crate::{block_descriptor::BlockDescriptor, config::Config, decoder::Decoder, guest_memory::GuestMemory, ir::block::Block, state::State, translator::Translator};

pub struct Jit<T: GuestMemory> {
    config: Config,
    memory: T,
    state: State,
    cycles_left: usize,
    decoder: Decoder,
}

impl<T: GuestMemory> Jit<T> {
    pub fn new(config: Config, memory: T) -> Self {
        Self {
            config,
            memory,
            state: State::default(),
            cycles_left: 0,
            decoder: Decoder::new(),
        }
    }

    pub fn run(&mut self, cycles: usize) {
        for _ in 0..cycles {
            let descriptor = BlockDescriptor::from(&self.state);

            let mut translator = Translator {

            };

            for i in 0..self.config.block_size_limit {
                let addr = descriptor.addr();
                if descriptor.is_arm() {
                    let data = self.memory.read_word(addr);
                    self.decoder.call_arm(data, &mut translator);
                    todo!("data {:#08x}", data)
                } else {
                    todo!("handle thumb")
                }
            }


            // let block = self.translate(descriptor);
            // lookup block based on current state to form descriptor
            // if block doesn't exist, then compile new block

            // when compiling new block:
            // call a translate function which returns us a basic block
            // in translate function:
            // find starting pc
            // continually read and decode instructions until a terminating instruction is found (e.g. branch)
            // for each decoded instruction emit some ir into the basic block
            // then return the basic block
            // once the block is returned we can run the optimiser over the basic block
            // this is where i want to spend more time figuring out cool ways to optimise
            
            // afterwards we can then use a struct that implements Backend to compile the ir
            // into machine code or interpret it

            // there's no need to cache the basic blocks, as they get compiled into something else later on
        }
        
        todo!()
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}