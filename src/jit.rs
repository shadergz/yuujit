use crate::{backend::Backend, block_descriptor::BlockDescriptor, config::Config, decoder::Decoder, disassembler::Disassembler, guest_memory::GuestMemory, ir::block::Block, ir_interpreter::IrInterpreter, state::{Mode, State, StatusRegister}, translator::Translator};

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
            let mut descriptor = BlockDescriptor::from(&self.state);
            if !self.backend.has_code_at(descriptor) {
                self.translate(&mut descriptor);
            }

            self.backend.run(descriptor, &mut self.state);
        }
    }

    fn translate(&mut self, descriptor: &mut BlockDescriptor) {
        let mut translator = Translator::new(*descriptor);
        let mut disassembler = Disassembler::new();
        for _ in 0..self.config.block_size_limit {
            let addr = descriptor.addr();
            if descriptor.is_arm() {
                let data = self.memory.read_word(addr);
                self.decoder.call_arm(data, &mut disassembler);
                self.decoder.call_arm(data, &mut translator);
            } else {
                todo!("handle thumb")
            }

            // TODO: reduce number of advance calls (we already call advance in the individual opcodes when advance the pc)
            descriptor.advance();
        }
        
        let block = translator.consume();
        println!("{}", block);
        self.backend.compile(*descriptor, block);
    }

    pub fn get_cpsr(&self) -> StatusRegister {
        self.state.cpsr
    }

    pub fn set_cpsr(&mut self, value: StatusRegister) {
        self.state.cpsr = value;
    }

    pub fn set_gpr(&mut self, gpr: usize, value: u32) {
        self.state.gpr[gpr] = value;
    }

    pub fn set_banked_gpr(&mut self, gpr: usize, mode: Mode, value: u32) {
        let start = match mode {
            Mode::Fiq => 8,
            _ => 13,
        };

        if mode.is_banked() && gpr >= start && gpr <= 14 {
            self.state.gpr_banked[u32::from(mode) as usize][gpr - 8] = value;
        } else {
            self.state.gpr[gpr] = value;
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}