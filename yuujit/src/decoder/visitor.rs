use super::instructions::DataProcessing;

pub trait Visitor {
    fn arm_data_processing(&mut self, inst: DataProcessing);
}