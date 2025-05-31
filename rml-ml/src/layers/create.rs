use rml_data::tensor::shape::tensor::Tensor;
use rml_data::tensor::traits::dtype::dtype;
use rml_data::tensor::traits::tensor::TensorBound;

pub trait Layer<'a, T: dtype> {
    type Output: TensorBound;

    fn forward(&self, tensor: Tensor<'a, T>) -> Self::Output;
}
