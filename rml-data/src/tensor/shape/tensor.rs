use std::{
    fmt::{Display, Error, Formatter},
    marker::PhantomData,
    ops::{Index, IndexMut, Range},
    slice::{Iter, IterMut},
};

use crate::tensor::traits::dtype::dtype;
macro_rules! get {
    [$e:expr, $it:expr] => {
        $e.get($it)
    };
}

// Standard 1 Dimensional
#[derive(Debug, Clone)]
pub struct Tensor<'a, T: dtype> {
    data: Vec<T>,
    phn: PhantomData<&'a T>,
}
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
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn combine(&mut self, other: Self) -> () {
        self.data.extend_from_slice(
            Vec::with_capacity(i32::abs(self.len() as i32 - other.len() as i32) as usize)
                .as_slice(),
        );

        for (i, t) in self.data.iter_mut().zip(other.into_iter()) {
            *i = i.clone() + t.clone();
        }
    }
    pub(crate) fn data_index(&self, i: usize) -> &T {
        self.data.index(i)
    }
    pub(crate) fn data_index_mut(&mut self, i: usize) -> &mut T {
        self.data.index_mut(i)
    }
    pub fn get(&self, i: usize) -> Option<&T> {
        self.data.get(i)
    }
    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        self.data.get_mut(i)
    }
    pub unsafe fn get_unchecked(&self, i: usize) -> &T {
        unsafe { self.data.get_unchecked(i) }
    }
    pub unsafe fn get_mut_unchecked(&mut self, i: usize) -> &mut T {
        unsafe { self.data.get_unchecked_mut(i) }
    }

    // range index
    pub(crate) fn data_range_index(&self, r: Range<usize>) -> Option<&[T]> {
        match self.data.as_slice().get(r) {
            Some(n) => Some(n),
            None => None,
        }
    }

    // range mut index
    pub(crate) fn data_mut_range_index(&mut self, r: Range<usize>) -> Option<&mut [T]> {
        match self.data.as_mut_slice().get_mut(r) {
            Some(n) => Some(n),
            None => None,
        }
    }

    pub fn mutate<F: Fn(&T) -> T>(&mut self, f: F) -> () {
        for t in self.data.iter_mut() {
            *t = f(t)
        }
    }
    pub fn sum(&self) -> T {
        let mut t = T::from_f32(0.0);
        for i in self.data.iter() {
            t = t + i.relu()
        }
        t
    }
    pub fn apply<F: Fn(&T) -> T>(self, f: F) -> Self {
        Tensor::from(self.data.iter().map(f).collect())
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
