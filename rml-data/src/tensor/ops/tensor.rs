use std::ops::{Add, Div, Mul, Sub};

use crate::tensor::{
    shape::tensor::Tensor,
    traits::{dtype::dtype, tensor::TensorBound},
};

use super::{create::Sigmoid, dtype::dtypeops};

impl<T: dtype> Div<T> for Tensor<'_, T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        todo!()
    }
}
impl<T: dtype> Mul<T> for Tensor<'_, T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        todo!()
    }
}
