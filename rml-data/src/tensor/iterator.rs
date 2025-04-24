use std::{cell::Ref, slice::Iter};

use super::{tensor::Tensor, tensorable::Tensorable};

// -------------------------
pub struct TensorVecWrapper<'a, T: Tensorable + 'a> {
    pub r: Ref<'a, Vec<T>>,
}
impl<'a, 'b: 'a, T: Tensorable + 'a> IntoIterator for &'b TensorVecWrapper<'a, T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Iter<'a, T> {
        self.r.iter()
    }
}

// and we'll implement FromIterator
impl<T: Tensorable> FromIterator<T> for Tensor<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let t: Vec<T> = iter.into_iter().collect();
        t.into()
    }
}
