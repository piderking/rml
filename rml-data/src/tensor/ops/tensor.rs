use std::ops::{Add, Div, Mul, Sub};

use crate::tensor::{
    shape::tensor::Tensor,
    traits::{dtype::dtype, tensor::TensorBound},
};

use super::create::{Relu, Sigmoid, Softmax};
impl<T: dtype> Add<T> for Tensor<'_, T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        self.apply(|f| f.clone() + rhs.clone())
    }
}
impl<T: dtype> Sub<T> for Tensor<'_, T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        self.apply(|f| f.clone() - rhs.clone())
    }
}

impl<T: dtype> Div<T> for Tensor<'_, T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self.apply(|f| f.clone() / rhs.clone())
    }
}
impl<T: dtype> Mul<T> for Tensor<'_, T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        self.apply(|f| f.clone() * rhs.clone())
    }
}

// Softmax Function
impl<T: dtype> Softmax for Tensor<'_, T> {
    fn softmax(&self) -> Self {
        let sum: T = self.sum();

        // clone the data --> return copy of it
        self.clone().apply(|f| f.relu() / sum.clone())
    }
}

// Sigmoid
impl<T: dtype> Sigmoid for Tensor<'_, T> {
    fn sigmoid(&self) -> Self {
        // clone the data --> return copy of it
        self.clone().apply(|f| f.sigmoid())
    }
}

// Relu
impl<T: dtype> Relu for Tensor<'_, T> {
    fn relu(&self) -> Self {
        // clone the data --> return copy of it
        self.clone().apply(|f| f.relu())
    }
}
