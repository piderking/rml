use std::{
    cell::{Cell, Ref, RefCell},
    marker::PhantomData,
    ops,
    option::IterMut,
    rc::Rc,
};

use super::{error::TensorError, iterator::TensorVecWrapper, tensorable::Tensorable};

pub trait TensorBound {
    type T: Tensorable;
    fn to_tensor(&self) -> &Tensor<Self::T>;
}

impl<T: Tensorable> TensorBound for Tensor<T> {
    type T = T;

    fn to_tensor(&self) -> &Tensor<Self::T> {
        return self;
    }
}

pub struct Tensor<T: Tensorable> {
    //* Wrapper Class for Vectors with Trait Bounds */
    #[allow(dead_code)]
    pub(crate) data: RefCell<Vec<T>>,
}

impl<T: Tensorable> Tensor<T> {
    pub fn new(data: Vec<T>) -> Tensor<T> {
        // check for the size to be identical
        Tensor {
            data: RefCell::new(data),
        }
    }
    pub fn get(&self, i: usize) -> Option<T> {
        match self.data.borrow().get(i) {
            Option::Some(n) => Option::Some(*n),
            Option::None => Option::None,
        }
    }
    pub fn set(&mut self, i: usize, t: T) -> Result<T, TensorError> {
        match self.data.try_borrow_mut() {
            Ok(mut n) => {
                let l = n.swap_remove(i);
                Ok(l)
            }
            Err(n) => Err(TensorError::BorrowMutError(n)),
        }
    }

    pub fn iter(&self) -> TensorVecWrapper<'_, T> {
        TensorVecWrapper {
            r: self.data.borrow(),
        }
    }
}

impl<T: Tensorable> From<Vec<T>> for Tensor<T> {
    fn from(value: Vec<T>) -> Self {
        Tensor::new(value)
    }
}

/*impl<T: Tensorable> From<&dyn TensorBound<T = T>> for &'_ Tensor<T> {
    fn from(value: &dyn TensorBound<T = T>) -> Self {
        value.to_tensor()
    }
} */
