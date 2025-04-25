use core::fmt;
use std::{
    cell::{Cell, Ref, RefCell}, fmt::Debug, marker::PhantomData, ops::{self, Index}, option::IterMut, rc::Rc, slice::SliceIndex
};
use std::fmt::Display;

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

#[derive(Debug)]
pub struct Tensor<T: Tensorable> {
    //* Wrapper Class for Vectors with Trait Bounds */
    #[allow(dead_code)]
    pub(crate) data: Rc<RefCell<Vec<T>>>,
}

impl<T: Tensorable> Tensor<T> {
    pub fn new(data: Vec<T>) -> Tensor<T> {
        // check for the size to be identical
        Tensor {
            data: Rc::new(RefCell::new(data)),
        }
    }
    pub fn from(data:Rc<RefCell<Vec<T>>>) -> Tensor<T> {
        // check for the size to be identical
        Tensor {
            data,
        }
    }
    pub fn get(&self, i: usize) -> Option<T> {
        match self.get_data_ref().get(i) {
            Option::Some(n) => Option::Some(*n),
            Option::None => Option::None,
        }
    }
    
    
    pub fn set(&mut self, i: usize, t: T) -> Result<T, TensorError> {
        match self.data.try_borrow_mut() {
            Ok(mut n) => {
                let l = n.swap_remove(i);
                n.insert(i, t,);
                Ok(l)
            }
            Err(n) => Err(TensorError::BorrowMutError(n)),
        }
    }

    pub fn as_ref(&self)-> Tensor<T>{ // still pointing to the same data
        Tensor::from(self.data.clone())
    }

    pub fn get_data_ref (&self) -> Ref<'_, Vec<T>> {
        self.data.borrow()
    }

    pub fn map<F: FnMut(&mut T)->T >(&mut self,mut f: F) -> Self {
        let mut t = self.data.borrow_mut();
        for item in t.iter_mut() {
            *item = f(item);
        }
        self.as_ref()
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

impl <T: Tensorable + Debug> Display for Tensor<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data.borrow())
    }
}

/*impl<T: Tensorable> From<&dyn TensorBound<T = T>> for &'_ Tensor<T> {
    fn from(value: &dyn TensorBound<T = T>) -> Self {
        value.to_tensor()
    }
} */
