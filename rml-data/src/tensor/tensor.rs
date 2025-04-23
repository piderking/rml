use std::{
    cell::{Ref, RefCell},
    marker::PhantomData,
    ops,
    rc::Rc,
};

use super::tensorable::Tensorable;

pub trait TensorBound<'a> {
    type T: Tensorable<'a>;
    fn to_tensor(&'a self) -> &'a Tensor<'a, Self::T>;
}

#[derive(Clone)] // going to be managing the data --> 
pub struct Tensor<'a, T: Tensorable<'a>> {
    /// wrapper of the Vec<> class --> additional methods
    pub(crate) data: Rc<RefCell<Vec<T>>>,
    phn: PhantomData<&'a T>,
}

impl<'a, T: Tensorable<'a>> TensorBound<'a> for Tensor<'a, T> {
    type T = T;
    fn to_tensor(&'a self) -> &'a Tensor<'a, Self::T> {
        return &*self;
    }
}

impl<'a, T: Tensorable<'a>> Tensor<'a, T> {
    pub fn new(data: Vec<T>) -> Tensor<'a, T> {
        // check for the size to be identical
        Tensor {
            data: Rc::new(RefCell::new(data)),
            phn: PhantomData,
        }
    }
    pub fn get(&'a self, i: usize) -> Option<Ref<'a, T>> {
        Option::Some(Ref::map(self.data.borrow(), |x| &x[i]))
    }
}

impl<'a, T: Tensorable<'a>> From<&[T]> for Tensor<'a, T> {
    fn from(value: &[T]) -> Self {
        Tensor::new(Vec::from(value))
    }
}

impl<'a, T: Tensorable<'a>> From<&'a dyn TensorBound<'a, T = T>> for &'a Tensor<'a, T> {
    fn from(value: &'a dyn TensorBound<'a, T = T>) -> Self {
        value.to_tensor()
    }
}
