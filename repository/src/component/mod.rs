pub mod page;


pub mod operation_trait {
    pub trait FeeCalculator {
        fn fee_calculate(&mut self);
    }
}