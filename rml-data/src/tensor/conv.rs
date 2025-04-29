use super::{tensor::Tensor, tensortype::TensorType};

impl<T: TensorType> From<Vec<T>> for Tensor<T> {
    fn from(value: Vec<T>) -> Self {
        Tensor::new(value)
    }
}

impl <T: TensorType> From<&[T]> for Tensor<T>{
    fn from(value: &[T]) -> Self {
        value.into() // converts it into a vec --> converts it into a tensor
    }
}