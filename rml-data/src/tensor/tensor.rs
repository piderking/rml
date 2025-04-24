use std::{
    cell::{Ref, RefCell},
    marker::PhantomData,
    ops,
    option::IterMut,
    rc::Rc,
};

use super::{iterator::TensorVecWrapper, tensorable::Tensorable};

pub trait TensorBound {
    type T: Tensorable;
    fn to_tensor(&self) -> &Tensor<Self::T>;
}

#[derive(Clone, Default)] // going to be managing the data --> 
pub struct Tensor<T: Tensorable> {
    /// wrapper of the Vec<> class --> additional methods
    data: Rc<RefCell<Vec<T>>>,
}

impl<T: Tensorable> TensorBound for Tensor<T> {
    type T = T;
    fn to_tensor(&self) -> &Tensor<Self::T> {
        return &*self;
    }
}

impl<T: Tensorable> Tensor<T> {
    pub fn new(data: Vec<T>) -> Tensor<T> {
        // check for the size to be identical
        Tensor {
            data: Rc::new(RefCell::new(data)),
        }
    }
    pub fn get(&self, i: usize) -> Option<Ref<'_, T>> {
        Option::Some(Ref::map(self.data.borrow(), |x| &x[i]))
    }

    pub fn iter(&self) -> TensorVecWrapper<T> {
        TensorVecWrapper {
            r: self.data.borrow(),
        }
    }
}

impl<T: Tensorable> From<&[T]> for Tensor<T> {
    fn from(value: &[T]) -> Self {
        Tensor::new(Vec::from(value))
    }
}

/*impl<T: Tensorable> From<&dyn TensorBound<T = T>> for &'_ Tensor<T> {
    fn from(value: &dyn TensorBound<T = T>) -> Self {
        value.to_tensor()
    }
} */
