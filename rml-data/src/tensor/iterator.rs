use std::marker::PhantomData;

use super::{
    tensor::{Tensor, TensorBound},
    tensorable::Tensorable,
};

pub struct MutTensorItterator<'a, T: Tensorable<'a>> {
    curr: usize,
    tensor: Vec<T>,
    phn: PhantomData<&'a T>,
}

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
// impl Itterator

// Convert from any trait object into TensorWrapper()
/*
impl<T: Tensorable> From<Box<dyn TensorBound<T = T>>> for Tensor<T> {
    fn from(value: Box<dyn TensorBound<T = T>>) -> Self {
        value.to_tensor()
    }
} */
