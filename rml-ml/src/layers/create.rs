use rml_data::tensor::shape::tensor::Tensor;
use rml_data::tensor::traits::dtype::{dtype};
use rml_data::tensor::traits::tensor::TensorBound;


pub struct WB <'a, T: dtype> {
    pub weight: Tensor<'a, T>,
    pub bias: Tensor<'a, T>
}


pub enum LayerArgument <'a, T: dtype> {
    Empty(),
    Value(T),
    Tensor(Tensor<'a, T>),
    WB(WB<'a, T>)
}

pub trait Layer<'a, T: dtype> {
    type Output: TensorBound;

    fn forward(&self, tensor: Tensor<'a, T>) -> Self::Output;
    fn fill(&mut self, arg: LayerArgument<'a, T>) -> ();

}

