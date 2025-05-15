use std::ops::{Add, Div, Mul, Sub};

use crate::tensor::{
    shape::tensor::{MutTensor, Tensor},
    traits::{dtype::dtype, tensor::TensorBound},
};

impl<T: dtype> Div<T> for Tensor<'_, T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self.apply(|f| f.clone() / rhs.clone())
    }
}
impl<T: dtype> Mul<T> for Tensor<'_, T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        todo!()
    }
}
