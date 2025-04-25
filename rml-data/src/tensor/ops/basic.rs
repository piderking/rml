use std::ops::{Add, Div, Mul, Sub};

use super::super::{tensor::Tensor, tensorable::Tensorable};

impl<T> Add<T> for Tensor<T>
where
    T: Tensorable,
    T: Add<Output = T>,
{
    type Output = Tensor<T>;

    fn add(mut self, rhs: T) -> Self::Output {
        self.map(|t| *t + rhs)
    }
}

impl<T> Sub<T> for Tensor<T>
where
    T: Tensorable,
    T: Sub<Output = T>,
{
    type Output = Tensor<T>;

    fn sub(mut self, rhs: T) -> Self::Output {
        self.map(|t| *t - rhs)
    }
}

impl<T> Mul<T> for Tensor<T>
where
    T: Tensorable,
    T: Mul<Output = T>,
{
    type Output = Tensor<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self.map(|t| *t * rhs)
    }
}

impl<T> Div<T> for Tensor<T>
where
    T: Tensorable,
    T: Div<Output = T>,
{
    type Output = Tensor<T>;

    fn div(mut self, rhs: T) -> Self::Output {
        self.map(|t| *t / rhs)
    }
}
