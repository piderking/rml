use std::{cell::RefMut, marker::PhantomData};

use super::{
    tensor::{Tensor, TensorBound},
    tensorable::Tensorable,
};

pub struct TensorIterator<'a, T: Tensorable>(&'a mut [T]);

impl<'a, T: Tensorable> Iterator for TensorIterator<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let shards = std::mem::replace(&mut self.0, &mut []);
        match shards {
            [next, rest @ ..] => {
                self.0 = rest;
                Some(next)
            }
            _ => None,
        }
    }
}
