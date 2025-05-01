use std::{
    cell::{Ref, RefMut},
    slice::{Iter, IterMut},
};

use super::{dtype::dtype, tensor::Tensor};

// -------------------------
pub struct TensorIterWrapper<'a, T: dtype + 'a> {
    pub r: Ref<'a, Vec<T>>,
}
impl<'a, 'b: 'a, T: dtype + 'a> IntoIterator for &'b TensorIterWrapper<'a, T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Iter<'a, T> {
        self.r.iter()
    }
}

// and we'll implement FromIterator
impl<T: dtype> FromIterator<T> for Tensor<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let t: Vec<T> = iter.into_iter().collect();
        t.into()
    }
}
