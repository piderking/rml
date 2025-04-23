use std::{marker::PhantomData, ops, rc::Rc};

use super::tensorable::Tensorable;

pub trait TensorBound<'a> {
    type T: Tensorable<'a>;
    fn to_tensor(&'a self) -> Tensor<Self::T>;
}

#[derive(Clone)] // going to be managing the data --> 
pub struct Tensor<'a, T: Tensorable<'a>> {
    data: Rc<Vec<T>>,
    phn: PhantomData<&'a T>,
}

impl<'a, T: Tensorable<'a>> TensorBound<'a> for Tensor<'a, T> {
    type T = T;
    fn to_tensor(&'a self) -> Tensor<Self::T> {
        return self.clone();
    }
}

impl<'a, T: Tensorable<'a>> Tensor<'a, T> {
    pub fn new(data: Vec<T>) -> Tensor<'a, T> {
        // check for the size to be identical
        Tensor {
            data: Rc::new(data),
            phn: PhantomData,
        }
    }
    pub fn get(&'a self, i: usize) -> Option<&'a T> {
        self.data.get(i)
    }
}

impl<'a, T: Tensorable<'a>> From<Vec<T>> for Tensor<'a, T>
where
    &'a T: 'a,
{
    fn from(value: Vec<T>) -> Self {
        Tensor::new(value)
    }
}
impl<'a, T: Tensorable<'a>> From<&[T]> for Tensor<'a, T> {
    fn from(value: &[T]) -> Self {
        Tensor::new(Vec::from(value))
    }
}

impl<'a, T: Tensorable<'a>> ops::Deref for Tensor<'a, T> {
    type Target = Rc<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
