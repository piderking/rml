use std::marker::PhantomData;

use super::{
    tensor::{Tensor, TensorBound},
    tensorable::Tensorable,
};

pub struct TensorItterator<'a, T: Tensorable<'a>> {
    curr: usize,
    tensor: Vec<T>, // Convert Tensor into Vec[]
    phn: PhantomData<&'a T>,
}

impl<'a, T: Tensorable<'a>> From<Tensor<'a, T>> for Vec<T> {
    fn from(value: Tensor<'a, T>) -> Self {
        value.into()
    }
}

impl<'a, T: Tensorable<'a>> IntoIterator for &'a Tensor<'a, T> {
    type Item = T;

    type IntoIter = TensorItterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        TensorItterator {
            curr: 0,
            tensor: self.to_vec(),
            phn: PhantomData,
        }
    }
}
// impl Itterator
impl<'a, T: Tensorable<'a>> Iterator for TensorItterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr < self.tensor.len() {
            true => {
                self.curr += 1;
                Option::Some(self.tensor.get(self.curr - 1).unwrap().clone())
            }
            false => Option::None,
        }
    }
}
// Convert from any trait object into TensorWrapper()
/*
impl<T: Tensorable> From<Box<dyn TensorBound<T = T>>> for Tensor<T> {
    fn from(value: Box<dyn TensorBound<T = T>>) -> Self {
        value.to_tensor()
    }
} */
