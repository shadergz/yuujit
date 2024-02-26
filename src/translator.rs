use crate::{decoder::{instructions::{DataProcessing, DataProcessingOpcode, ImmShiftedOperand, ShiftedOperand}, visitor::Visitor}, ir::{emitter::Emitter, value::{Type, Value, U1, U32}}};

pub struct Translator {
    ir: Emitter,
}

impl Translator {
    pub fn new() -> Self {
        Self {
            ir: Emitter::new(),
        }
    }

    pub fn emit_imm_shift(&mut self, operand: ImmShiftedOperand, set_carry: bool) -> (Value<U32>, Value<U1>) {
        let rotated = Value::from(operand.rotated);
        if set_carry {
            (rotated, Value::from(operand.rotated >> 31))
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
            DataProcessingOpcode::Mov => {
                let result = self.ir.copy(rhs);
                if set_flags {
                    self.ir.store_nzc(result, carry);
                }

                Some(result)
            },
            _ => todo!("handle {:?}", inst.opcode),
        };

        if let Some(result) = result {
            self.ir.store_gpr(inst.rd, result);
        }
    }
}