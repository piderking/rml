use std::{
    fmt::{Display, Error, Formatter},
    marker::PhantomData,
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
impl<T> Tensor<'_, T>
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
    pub fn combine(&mut self, other: Tensor<'_, T>) -> () {
        self.data.extend_from_slice(
            Vec::with_capacity(i32::abs(self.len() as i32 - other.len() as i32) as usize)
                .as_slice(),
        );

        for (i, t) in self.data.iter_mut().zip(other.into_iter()) {
            *i = i.clone() + t.clone();
        }
    }
    pub fn get(&self, i: usize) -> Option<T> {
        self.data.get(i).cloned()
    }
    pub fn mutate<F: Fn(&T) -> T>(&mut self, f: F) -> () {
        print!("{:}", get![self, 0].unwrap());
        for t in self.data.iter_mut() {
            *t = f(t)
        }
    }
    pub fn sum(&self) -> T {
        let mut t = T::from_f32(0.0);
        for i in self.data.iter() {
            t = t + i.clone()
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
