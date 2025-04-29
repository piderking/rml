use std::ops::{Add, Div, Mul, Sub};

use crate::tensor::error::TensorOpperationError;

use super::super::{tensor::Tensor, tensortype::TensorType};

// combine together the two tensors
impl<T> Add<Tensor<T>> for Tensor<T>
where
    T: TensorType,
    T: Add<Output = T>,
{
    type Output = Result<Tensor<T>, TensorOpperationError>;

    fn add(mut self, rhs: Tensor<T>) -> Self::Output {
        Ok(self.combine(rhs))
    }
}

// combine together the two tensors
impl<T> Mul<Tensor<T>> for Tensor<T>
where
    T: TensorType,
    T: Mul<Output = T>,
{
    type Output = Result<Tensor<T>, TensorOpperationError>;

    fn mul(mut self, rhs: Tensor<T>) -> Self::Output {
        self.dot(rhs)
    }
}
