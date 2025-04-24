use super::{tensor::Tensor, tensorable::Tensorable};

impl<T: Tensorable> From<Vec<T>> for Tensor<T> {
    fn from(value: Vec<T>) -> Self {
        Tensor::new(value)
    }
}

impl<T: Tensorable> From<Tensor<T>> for Vec<T> {
    fn from(value: Tensor<T>) -> Self {
        value.into()
    }
}
