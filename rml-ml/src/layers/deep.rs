use rml_data::tensor::{shape::tensor::Tensor, traits::dtype::dtype};

use crate::{
    context::{ContextFlag, ContextStruct},
    contextual::Contextual,
};

use super::create::{Deep, Layer};

impl<'a, T: dtype> Layer<'a, T> for Deep<'a, T> {
    type Output = Tensor<'a, T>;

    fn fill(&mut self, t: Tensor<'a, T>) -> () {
        self.tensor = t
    }

    fn layer(&self, ctx: Contextual) -> Self::Output {
        todo!()
    }
}
