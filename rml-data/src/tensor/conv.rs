use super::{
    shape::{deep::Deep, shallow::Shallow},
    traits::{dtype::dtype, tensor::TensorBound},
};

impl<T: TensorBound> From<Vec<T>> for Deep<T> {
    fn from(value: Vec<T>) -> Self {
        Deep::new(value)
    }
}

impl<T: dtype> From<Vec<T>> for Shallow<T> {
    fn from(value: Vec<T>) -> Self {
        Shallow::new(value)
    }
}

impl<T: dtype> From<&[T]> for Shallow<T> {
    fn from(value: &[T]) -> Self {
        value.into() // converts it into a vec --> converts it into a Shallow
    }
}
