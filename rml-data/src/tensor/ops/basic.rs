use std::ops::{Add, Div, Mul, Sub};

use super::super::{tensor::Tensor, tensortype::TensorType};

impl<T> Add<T> for Tensor<T>
where
    T: TensorType,
    T: Add<Output = T>,
{
    type Output = Tensor<T>;

    fn add(mut self, rhs: T) -> Self::Output {
        self.map(|t| t.clone() + rhs.clone())
    }
}

impl<T> Sub<T> for Tensor<T>
where
    T: TensorType,
    T: Sub<Output = T>,
{
    type Output = Tensor<T>;

    fn sub(mut self, rhs: T) -> Self::Output {
        self.map(|t| t.clone() - rhs.clone())
    }
}

impl<T> Mul<T> for Tensor<T>
where
    T: TensorType,
    T: Mul<Output = T>,
{
    type Output = Tensor<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self.map(|t|t.clone() * rhs.clone())
    }
}

impl<T> Div<T> for Tensor<T>
where
    T: TensorType,
    T: Div<Output = T>,
{
    type Output = Tensor<T>;

    fn div(mut self, rhs: T) -> Self::Output {
        self.map(|t| t.clone() / rhs.clone())
    }
}
