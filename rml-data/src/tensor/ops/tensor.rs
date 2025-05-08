use std::ops::{Add, Div, Mul, Sub};

use crate::tensor::{
    shape::tensor::{MutTensor, Tensor},
    traits::{dtype::dtype, tensor::TensorBound},
};

use super::create::Combine;

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

impl<T: dtype> Combine<Tensor<'_, T>> for Tensor<'_, T> {
    type Output = Self;

    fn combine(&mut self, other: Tensor<'_, T>) -> Self {
        let mut m = MutTensor::from(self.clone());
        m.into_iter().map(|d| *d = T::from_f32(0.0));
        m.into()
    }
}
