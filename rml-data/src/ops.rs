use std::{marker::PhantomData, ops::Add, path::Iter, vec::IntoIter};

use crate::tensor::{DTYPE, Tensor};

impl<'a, T: DTYPE> IntoIterator for Box<dyn Tensor<T> + 'a> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(self.as_slice()).into_iter()
    }
}

// Lifetime Requirements for the Iterator Magic
impl<'a, T: DTYPE + 'a> FromIterator<T> for Box<dyn Tensor<T> + 'a> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Box::new(Vec::from_iter(iter))
    }
}

impl<'a, T: DTYPE + 'a> Add<T> for Box<dyn Tensor<T> + 'a>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self::Output::from_iter(self.into_iter().map(|f| f + rhs.clone()))
    }
}
