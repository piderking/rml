use crate::context::Context;

use super::create::{Deep, Layer, Softmax};
use rml_data::tensor::ops::create::Softmax as Sft;
use rml_data::tensor::{shape::tensor::Tensor, traits::dtype::dtype};

impl<'a, T: dtype, C: Context> Layer<C> for Softmax<'a, T> {
    type Output = Tensor<'a, T>;

    fn layer(&self, ctx: &C) -> Self::Output {
        <Tensor<'a, T> as Sft>::softmax(&self.tensor)
    }
}
