use crate::decoder::{instructions::ArmDataProcessing, visitor::Visitor};

pub struct Translator {

}

impl Visitor for Translator {
    fn arm_data_processing(&mut self, inst: ArmDataProcessing) {
        todo!()
    }
}