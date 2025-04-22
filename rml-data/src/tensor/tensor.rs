use std::cell::RefCell;
use std::ops;

use super::tensorable::Tensorable;

#[derive(Clone)] // going to be managing the data --> 
pub struct TensorWrapper<T: Tensorable> {
    data: RefCell<Vec<T>>,
}

impl<T: Tensorable> TensorWrapper<T> {
    pub fn new(data: Vec<T>) -> TensorWrapper<T> {
        // check for the size to be identical
        TensorWrapper {
            data: RefCell::new(data),
        }
    }
}

impl<T: Tensorable> From<Vec<T>> for TensorWrapper<T> {
    fn from(value: Vec<T>) -> Self {
        TensorWrapper::new(value)
    }
}
impl<T: Clone + Tensorable> From<&[T]> for TensorWrapper<T> {
    fn from(value: &[T]) -> Self {
        TensorWrapper::new(Vec::from(value))
    }
}

impl<T: Tensorable> ops::Deref for TensorWrapper<T> {
    type Target = RefCell<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
