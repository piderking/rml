use std::{
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    mem,
};

use super::{
    tensor::{Tensor, TensorBound},
    tensorable::Tensorable,
};

pub struct TensorItterator<'a, T: Tensorable<'a>> {
    tensor: RefMut<'a, Vec<T>>,
    phn: PhantomData<&'a T>,
    curr: usize,
}

impl<'a, T: Tensorable<'a>> IntoIterator for &'a Tensor<'a, T> {
    type Item = &'a mut T;

    type IntoIter = TensorItterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        TensorItterator {
            tensor: self.data.borrow_mut(),
            phn: PhantomData,
            curr: 0,
        }
    }
}
// impl Itterator
impl<'a, T: Tensorable<'a>> Iterator for TensorItterator<'a, T> {
    type Item = &mut T;

    fn next(&mut self) -> Option<Self::Item> {
        &self.tensor.get_mut(self.curr)
    }
}
// Convert from any trait object into TensorWrapper()
/*
impl<T: Tensorable> From<Box<dyn TensorBound<T = T>>> for Tensor<T> {
    fn from(value: Box<dyn TensorBound<T = T>>) -> Self {
        value.to_tensor()
    }
} */
