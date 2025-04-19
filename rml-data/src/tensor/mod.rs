use std::rc::Rc;

use tensorable::Tensorable;
pub mod iterator;
pub mod tensorable;

#[derive(Clone)]
pub struct Tensor<T: Clone> {
    data: Rc<Box<[T]>>,
}

impl<T: Clone> Tensor<T> {
    fn new(data: Box<[T]>) -> Tensor<T> {
        Tensor {
            data: Rc::new(data),
        }
    }
    fn get(&self, index: i32) -> Option<&T> {
        match (index >= 0, index.abs() < self.data.len() as i32) {
            (true, true) => self.data.get(index as usize),
            (false, true) => self.data.get(self.data.len() + index as usize), // if negative swap to the backside
            (_, false) => Option::None,
        }
    }
}

impl<T: Clone> From<Box<[T]>> for Tensor<T> {
    fn from(value: Box<[T]>) -> Self {
        Tensor::new(value)
    }
}
