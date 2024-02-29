use crate::{backend::Backend, block_descriptor::BlockDescriptor, config::Config, decoder::Decoder, disassembler::Disassembler, guest_memory::GuestMemory, ir::block::Block, ir_interpreter::IrInterpreter, state::State, translator::Translator};

pub struct Jit<T: GuestMemory> {
    config: Config,
    memory: T,
    state: State,
    cycles_left: usize,
    decoder: Decoder,
    backend: IrInterpreter,
}

impl<T: GuestMemory> Jit<T> {
    pub fn new(config: Config, memory: T) -> Self {
        Self {
            config,
            memory,
            state: State::default(),
            cycles_left: 0,
            decoder: Decoder::new(),
            backend: IrInterpreter::new(),
        }
    }

    pub fn run(&mut self, cycles: usize) {
        for _ in 0..cycles {
            let descriptor = BlockDescriptor::from(&self.state);
            let mut translator = Translator::new(descriptor);
            let mut disassembler = Disassembler::new();
            
            assert_eq!(self.backend.has_code_at(descriptor), false);

            let addr = descriptor.addr();
            if descriptor.is_arm() {
                let data = self.memory.read_word(addr);
                self.decoder.call_arm(data, &mut disassembler);
                self.decoder.call_arm(data, &mut translator);
            } else {
                todo!("handle thumb")
            }

            let block = translator.consume();
            println!("{}", block);
            self.backend.compile(descriptor, block);
            self.backend.run(descriptor, &mut self.state);
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}