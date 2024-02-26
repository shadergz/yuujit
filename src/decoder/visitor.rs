use super::instructions::ArmDataProcessing;

pub trait Visitor {
    fn arm_data_processing(&mut self, inst: ArmDataProcessing);
}