use std::{
    fmt::{Display, Error, Formatter},
    marker::PhantomData,
    ops::{Index, IndexMut, Range},
    slice::{Iter, IterMut},
};

use crate::tensor::traits::{dtype::dtype, tensor::TensorBound};
macro_rules! get {
    [$e:expr, $it:expr] => {
        $e.get($it)
    };
}

// Standard 1 Dimensional
#[derive(Debug, Clone)]
pub struct Tensor<'a, T: dtype> {
    pub(crate) data: Vec<T>,
    phn: PhantomData<&'a T>,
}

// Transfer these all to Tensorbound (except from/new)
impl<'a, T> Tensor<'a, T>
where
    T: dtype,
{
    pub fn new(data: &[T]) -> Self {
        Tensor {
            data: data.into(),
            phn: PhantomData,
        }
    }
    pub fn from(data: Vec<T>) -> Self {
        Tensor {
            data,
            phn: PhantomData,
        }
    }
    pub fn empty() -> Self {
        Tensor {
            data: vec![],
            phn: PhantomData,
        }
        
    }

    pub fn with_capacity(size: usize, val: T) -> Self {
        Tensor {
            data: Vec::with_capacity(size).iter().map(|_: &T| val.clone()).collect(),
            phn: PhantomData,
        }
    }

    pub fn sum(&self) -> T {
        let mut t = T::from_f32(0.0);
        for i in self.data.iter() {
            t = t + i.relu()
        }
        t
    }
}

// eq
impl<'a, T: dtype + PartialEq> PartialEq for Tensor<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self
                .into_iter()
                .zip(other.into_iter())
                .filter(|(v1, v2)| v1 != v2)
                .count()
                == 0
    }
}

// Implement Itterator
impl<'a, 'b: 'a, T: dtype + 'a> IntoIterator for &'b Tensor<'a, T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Iter<'a, T> {
        self.data.iter()
    }
}

// Mutable Wrapper for the Tensor
#[warn(deprecated)]
pub struct MutTensor<'a, T: dtype>(pub(crate) Tensor<'a, T>);

impl<'a, T: dtype> MutTensor<'a, T> {
    pub fn from(ten: Tensor<'a, T>) -> MutTensor<'a, T> {
        MutTensor(ten)
    }
}

impl<'a, T: dtype> From<MutTensor<'a, T>> for Tensor<'a, T> {
    fn from(value: MutTensor<'a, T>) -> Self {
        value.0
    }
}

// Implement Itterator
impl<'a, 'b: 'a, T: dtype + 'a> IntoIterator for &'b mut MutTensor<'a, T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> IterMut<'a, T> {
        self.0.data.iter_mut()
    }
}

// Display All of Body
impl<T: dtype + Display> Display for Tensor<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let elements = self
            .data
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}]", elements)
    }
}
