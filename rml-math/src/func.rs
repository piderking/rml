use rml_data::tensor::{DTYPE, Tensor};

use crate::function;

pub trait Sigmoid {
    fn sigmoid(self) -> Self;
}
pub trait Relu {
    fn relu(self) -> Self;
}
pub trait Tanh {
    fn tanh(self) -> Self;
}

impl Sigmoid for f32 {
    fn sigmoid(self) -> Self {
        function::sigmoid(self)
    }
}
impl Relu for f32 {
    fn relu(self) -> Self {
        function::relu(self)
    }
}
impl Tanh for f32 {
    fn tanh(self) -> Self {
        function::relu(self)
    }
}

// Tensor Transformation
impl<'a, T> Sigmoid for Box<dyn Tensor<T> + 'a>
where
    T: DTYPE + 'a,
    T: Sigmoid,
{
    fn sigmoid(self) -> Self {
        <Box<dyn Tensor<T>>>::from_iter(self.into_iter().map(|f| T::sigmoid(f)))
    }
}

impl<'a, T> Relu for Box<dyn Tensor<T> + 'a>
where
    T: DTYPE + 'a,
    T: Relu,
{
    fn relu(self) -> Self {
        <Box<dyn Tensor<T>>>::from_iter(self.into_iter().map(|f| T::relu(f)))
    }
}

impl<'a, T> Tanh for Box<dyn Tensor<T> + 'a>
where
    T: DTYPE + 'a,
    T: Tanh,
{
    fn tanh(self) -> Self {
        <Box<dyn Tensor<T>>>::from_iter(self.into_iter().map(|f| T::tanh(f)))
    }
}
