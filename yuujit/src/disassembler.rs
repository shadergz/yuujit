use crate::decoder::{instructions::{DataProcessing, DataProcessingOpcode, ShiftedOperand}, visitor::Visitor};

pub struct Disassembler {
    
}

impl Disassembler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Visitor for Disassembler {
    fn arm_data_processing(&mut self, inst: DataProcessing) {
        let set_flags = match inst.set_flags {
            true => "s",
            false => "",
        };

        let rhs = match inst.operand {
            ShiftedOperand::Imm(operand) => format!("{:#x}", operand.rotated),
            ShiftedOperand::Reg(operand) => todo!(),
        };

        let opcode = match inst.opcode {
            DataProcessingOpcode::And => todo!(),
            DataProcessingOpcode::Eor => todo!(),
            DataProcessingOpcode::Sub => todo!(),
            DataProcessingOpcode::Rsb => todo!(),
            DataProcessingOpcode::Add => println!("add{}{} r{}, r{}, {}", inst.condition, set_flags, inst.rd, inst.rn, rhs),
            DataProcessingOpcode::Adc => todo!(),
            DataProcessingOpcode::Sbc => todo!(),
            DataProcessingOpcode::Rsc => todo!(),
            DataProcessingOpcode::Tst => todo!(),
            DataProcessingOpcode::Teq => todo!(),
            DataProcessingOpcode::Cmp => todo!(),
            DataProcessingOpcode::Cmn => todo!(),
            DataProcessingOpcode::Orr => todo!(),
            DataProcessingOpcode::Mov => println!("mov{}{} r{}, {}", inst.condition, set_flags, inst.rd, rhs),
            DataProcessingOpcode::Bic => todo!(),
            DataProcessingOpcode::Mvn => todo!(),
        };
    }
}