use super::value::{Value, U1, U32};

pub struct Emitter {
    
}

impl Emitter {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn load_cpsr(&mut self) -> Value<U32> {
        todo!()
    }

    pub fn load_c(&mut self) -> Value<U1> {
        let cpsr = self.load_cpsr();
        self.get_bit(cpsr, 29)
    }

    pub fn get_bit(&mut self, src: Value<U32>, bit: usize) -> Value<U1> {
        todo!()
    }

    pub fn copy(&mut self, src: Value<U32>) -> Value<U32> {
        todo!()
    }

    pub fn store_nzc(&mut self, src: Value<U32>, carry: Value<U1>) {
        todo!()
    }

    pub fn store_gpr(&mut self, dst: u8, src: Value<U32>) {
        todo!()
    }
}