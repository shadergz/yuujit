use crate::{config::Config, guest_memory::GuestMemory, state::State};

pub struct Jit<T: GuestMemory> {
    config: Config,
    memory: T,
    state: State,
}

impl<T: GuestMemory> Jit<T> {
    pub fn new(config: Config, memory: T) -> Self {
        Self {
            config,
            memory,
            state: State::default(),
        }
    }

    pub fn run(&mut self, cycles: usize) {
        todo!()
    }
}