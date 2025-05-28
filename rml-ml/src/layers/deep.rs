use rml_data::tensor::{shape::tensor::Tensor, traits::dtype::dtype};

use crate::context::{ContextFlag, ContextStruct};

use super::create::{Deep, Layer};

impl<'a, T: dtype> Layer<'a, T> for Deep<'a, T> {
    type Output = Tensor<'a, T>;

    fn fill(&mut self, t: Tensor<'a, T>) -> () {
        todo!()
    }

    fn layer(&self, ctx: &ContextStruct<'a>) -> Self::Output {
        todo!()
    }
}
