use crate::{block_descriptor::BlockDescriptor, decoder::{instructions::{DataProcessing, DataProcessingOpcode, ImmShiftedOperand, ShiftedOperand}, visitor::Visitor}, ir::{block::Block, emitter::Emitter, value::{Type, Value, U1, U32}}};

pub struct Translator {
    ir: Emitter,
}

impl Translator {
    pub fn new(descriptor: BlockDescriptor) -> Self {
        Self {
            ir: Emitter::new(descriptor),
        }
    }

    pub fn consume(self) -> Block {
        self.ir.consume()
    }

    pub fn emit_imm_shift(&mut self, operand: ImmShiftedOperand, set_carry: bool) -> (Value<U32>, Value<U32>) {
        let rotated = Value::from_imm(operand.rotated);
        if set_carry {
            (rotated, Value::from_imm(operand.rotated >> 31))
        } else {
            (rotated, self.ir.load_c())
        }
    }
}

impl Visitor for Translator {
    fn arm_data_processing(&mut self, inst: DataProcessing) {
        let set_carry = inst.set_flags && !inst.opcode.uses_carry();
        let set_flags = inst.set_flags && (inst.rd != 15 || !inst.opcode.uses_destination());
        let (rhs, carry) = match inst.operand {
            ShiftedOperand::Imm(operand) => self.emit_imm_shift(operand, set_carry),
            ShiftedOperand::Reg(_) => todo!("handle reg shifted operand"),
        };

        let result = match inst.opcode {
            DataProcessingOpcode::And => todo!(),
            DataProcessingOpcode::Eor => todo!(),
            DataProcessingOpcode::Sub => todo!(),
            DataProcessingOpcode::Rsb => todo!(),
            DataProcessingOpcode::Add => {
                // TODO: clean this up
                let lhs = self.ir.load_gpr(inst.rn);
                if set_flags {
                    let (result, carry, overflow) = self.ir.add_with_flags(lhs, rhs);
                    self.ir.store_nzcv(result, carry, overflow);
                    Some(result)
                } else {
                    Some(self.ir.add(lhs, rhs))
                }
            },
            DataProcessingOpcode::Adc => todo!(),
            DataProcessingOpcode::Sbc => todo!(),
            DataProcessingOpcode::Rsc => todo!(),
            DataProcessingOpcode::Tst => todo!(),
            DataProcessingOpcode::Teq => todo!(),
            DataProcessingOpcode::Cmp => todo!(),
            DataProcessingOpcode::Cmn => todo!(),
            DataProcessingOpcode::Orr => todo!(),
            DataProcessingOpcode::Mov => {
                let result = self.ir.copy(rhs);
                if set_flags {
                    self.ir.store_nzc(result, carry);
                }

                Some(result)
            },
            DataProcessingOpcode::Bic => todo!(),
            DataProcessingOpcode::Mvn => todo!(),
        };

        if let Some(result) = result {
            self.ir.store_gpr(inst.rd, result);
        }

        self.ir.advance_pc();
    }
}