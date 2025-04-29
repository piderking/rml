use std::{
    cell::{Ref, RefMut},
    slice::{Iter, IterMut},
};

use super::{tensor::Tensor, tensortype::TensorType};

// -------------------------
pub struct TensorIterWrapper<'a, T: TensorType + 'a> {
    pub r: Ref<'a, Vec<T>>,
}
impl<'a, 'b: 'a, T: TensorType + 'a> IntoIterator for &'b TensorIterWrapper<'a, T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Iter<'a, T> {
        self.r.iter()
    }
}

// and we'll implement FromIterator
impl<T: TensorType> FromIterator<T> for Tensor<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let t: Vec<T> = iter.into_iter().collect();
        t.into()
    }
}
