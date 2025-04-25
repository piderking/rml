use std::ops::{Add, Div, Mul, Sub};

use super::super::{tensor::Tensor, tensorable::Tensorable};

// combine together the two tensors
impl<T> Add<Tensor<T>> for Tensor<T>
where
    T: Tensorable,
    T: Add<Output = T>,
{
    type Output = Tensor<T>;

    fn add(mut self, rhs: Tensor<T>) -> Self::Output {
        self.combine(rhs)
    }
}
