use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
    path::Iter,
    vec::IntoIter,
};

use crate::tensor::{DTYPE, Tensor};

impl<'a, T: DTYPE> IntoIterator for Box<dyn Tensor<T> + 'a> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(self.as_slice()).into_iter()
    }
}

// Lifetime Requirements for the Iterator Magic
impl<'a, T: DTYPE + 'a> FromIterator<T> for Box<dyn Tensor<T> + 'a> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Box::new(Vec::from_iter(iter))
    }
}

// Basic Opperations
impl<'a, T: DTYPE + 'a> Add<T> for Box<dyn Tensor<T> + 'a>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self::Output::from_iter(self.into_iter().map(|f| f + rhs.clone()))
    }
}

impl<'a, T: DTYPE + 'a> Sub<T> for Box<dyn Tensor<T> + 'a>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self::Output::from_iter(self.into_iter().map(|f| f - rhs.clone()))
    }
}

impl<'a, T: DTYPE + 'a> Mul<T> for Box<dyn Tensor<T> + 'a>
where
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output::from_iter(self.into_iter().map(|f| f * rhs.clone()))
    }
}

impl<'a, T: DTYPE + 'a> Div<T> for Box<dyn Tensor<T> + 'a>
where
    T: Div<T, Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output::from_iter(self.into_iter().map(|f| f / rhs.clone()))
    }
}
