use std::marker::PhantomData;

use super::{
    tensor::{Tensor, TensorBound},
    tensorable::Tensorable,
};

pub struct TensorMutItterator<'a, T: Tensorable<'a>> {
    curr: usize,
    tensor: Vec<T>, // Convert Tensor into Vec[]
    phn: PhantomData<&'a T>,
}
