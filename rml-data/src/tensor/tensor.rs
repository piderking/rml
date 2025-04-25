use super::{error::TensorError, iterator::TensorVecWrapper, tensorable::Tensorable};
use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::fmt::Display;
use std::ops::Add;
use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    rc::Rc,
};

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
    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }
    pub fn from(data: Rc<RefCell<Vec<T>>>) -> Tensor<T> {
        // check for the size to be identical
        Tensor { data }
    }
    pub fn get(&self, i: usize) -> Option<T> {
        match self.get_data_ref().get(i) {
            Option::Some(n) => Option::Some(*n),
            Option::None => Option::None,
        }
    }

    pub fn set(&mut self, i: usize, t: T) -> T {
        let mut n = self.data.borrow_mut();
        let l = n.swap_remove(i);
        n.insert(i, t);
        l
    }
    pub fn set_from_borrow(&mut self, mut bor: std::cell::RefMut<'_, Vec<T>>, i: usize, t: T) -> T {
        let l = bor.swap_remove(i);
        bor.insert(i, t);
        l
    }

    pub fn as_ref(&self) -> Tensor<T> {
        // still pointing to the same data
        Tensor::from(self.data.clone())
    }

    pub fn get_data_ref(&self) -> Ref<'_, Vec<T>> {
        self.data.borrow()
    }

    pub fn map<F: FnMut(&mut T) -> T>(&mut self, mut f: F) -> Self {
        let mut t = self.data.borrow_mut();
        for item in t.iter_mut() {
            *item = f(item);
        }
        self.as_ref()
    }

    pub fn map_zip<F: FnMut(&mut T, &T) -> T>(&mut self, item: Tensor<T>, mut f: F) -> Self {
        let mut t = self.data.borrow_mut();
        let o = item.data.borrow();

        for (i1, i2) in t.iter_mut().zip(o.iter()) {
            *i1 = f(i1, i2);
        }
        self.as_ref()
    }

    pub fn push(&mut self, t: T) -> Tensor<T> {
        self.data.borrow_mut().push(t);
        self.as_ref()
    }
    pub fn insert(&mut self, i: usize, t: T) -> Tensor<T> {
        self.data.borrow_mut().insert(i, t);
        self.as_ref()
    }

    pub fn combine(&mut self, item: Tensor<T>) -> Tensor<T>
    where
        T: Add<Output = T>,
    {
        {
            // to keep values alive
            let this = self.iter();
            let other = item.iter();

            let this_iter = this.into_iter();
            let other_iter = other.into_iter();

            // extend vector to applicable size
            if this_iter.len() < other_iter.len() {
                self.data
                    .borrow_mut()
                    .extend(vec![T::default(); other_iter.len() - this_iter.len()]);
            }
        }
        // call instance function
        self.map_zip(item, |i1, i2| *i1 + *i2);
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

impl<T: Tensorable + Debug> Display for Tensor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data.borrow())
    }
}

/*impl<T: Tensorable> From<&dyn TensorBound<T = T>> for &'_ Tensor<T> {
    fn from(value: &dyn TensorBound<T = T>) -> Self {
        value.to_tensor()
    }
} */
