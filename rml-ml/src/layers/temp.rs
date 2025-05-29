use rml_data::tensor::{
    shape::tensor::Tensor,
    traits::{dtype::dtype, tensor::TensorBound},
};

use crate::contextual::Contextual;

use super::create::{Empty, Layer, Temp};

impl<'a, T: dtype> Layer<'a, T> for Temp<'a, T> {
    type Output = Tensor<'a, T>;

    fn fill(&mut self, t: rml_data::tensor::shape::tensor::Tensor<'a, T>) -> () {
        self.tensor = t;
    }

    fn layer(&self, ctx: Contextual) -> Self::Output {
        self.tensor
            .clone()
            .apply(|f| T::from_f32(f.as_f32() + ctx.digit))
    }
}

impl<'a, T: dtype> Empty for Temp<'a, T> {
    fn empty() -> Self {
        Self {
            tensor: Tensor::empty(),
        }
    }
}
