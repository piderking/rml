use std::ops::Add;

use super::{tensor::Tensor, tensorable::Tensorable};

impl<T: Tensorable> Add<T> for Tensor<T> {
    type Output = Tensor<T>;

    fn add(self, rhs: T) -> Self::Output {
        self.
    }
}
