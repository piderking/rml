use std::{any::Any, marker::PhantomData};

use super::{tensorable::Tensorable, wrapper::Tensor};

pub struct TensorItterator<T: Tensorable + Clone> {
    curr: usize,
    tensor: Tensor<T>,
}

impl<T: Tensorable + Copy> Iterator for TensorItterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr += 1;
        match self.tensor.get(self.curr) {
            Option::Some(n) => Option::Some(*n),
            _ => Option::None,
        }
    }
}

/*
impl<T: Any + Clone> Tensorable for TensorItterator<T> {
    type Data = T;
    fn to_tensor(&self) -> &Tensor<Self::Data> {
        &self.tensor
    }
    fn size(&self) -> Vec<usize> {
        todo!()
    }
}

*/
