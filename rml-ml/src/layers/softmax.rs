use rml_data::tensor::ops::create::Softmax as Sft;
use rml_data::tensor::{shape::tensor::Tensor, traits::dtype::dtype};

use super::create::Layer;

pub struct Softmax {}
impl<'a, T: dtype> Layer<'a, T> for Softmax {
    type Output = Tensor<'a, T>;

    fn forward(&self, tensor: Tensor<'a, T>) -> Self::Output {
        <Tensor<'a, T> as Sft>::softmax(&tensor)
    }
}
