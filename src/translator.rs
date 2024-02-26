use crate::{decoder::{instructions::{DataProcessing, ShiftedOperand}, visitor::Visitor}, ir::emitter::Emitter};

pub struct Translator {
    emitter: Emitter,
}

impl Translator {
    pub fn new() -> Self {
        Self {
            emitter: Emitter::new(),
        }
    }
}

impl Visitor for Translator {
    fn arm_data_processing(&mut self, inst: DataProcessing) {
        let rhs = match inst.operand {
            ShiftedOperand::Imm(operand) => {
                
            },
            ShiftedOperand::Reg(_) => todo!("handle reg shifted operand"),
        };
    }
}