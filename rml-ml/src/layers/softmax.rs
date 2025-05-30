use crate::context::{ContextStruct, Model};

use super::create::{Deep, Empty, Layer, Softmax};
use rml_data::tensor::ops::create::Softmax as Sft;
use rml_data::tensor::{shape::tensor::Tensor, traits::dtype::dtype};

impl<'a, T: dtype> Layer<'a, T> for Softmax<'a, T> {
    type Output = Tensor<'a, T>;

    fn forward(&self) -> Self::Output {
        <Tensor<'a, T> as Sft>::softmax(&self.tensor)
    }

    fn fill(&mut self, t: Tensor<'a, T>) -> () {
        self.tensor = t
    }
}

impl<'a, T: dtype> Empty for Softmax<'a, T> {
    fn empty() -> Self {
        Self {
            tensor: Tensor::empty(),
        }
    }
}
