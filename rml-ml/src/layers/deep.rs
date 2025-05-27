use rml_data::tensor::{shape::tensor::Tensor, traits::dtype::dtype};

use crate::context::Context;

use super::create::{Deep, Layer};

impl<'a, T: dtype, C: Context> Layer<C> for Deep<'a, T> {
    type Output = Tensor<'a, T>;

    fn layer(&self, ctx: &C) -> Self::Output {
        todo!()
    }
}
