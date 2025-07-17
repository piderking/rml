use std::ops::{Add, Div, Mul, Sub};

pub trait DTYPE
where
    Self: Sized,
    Self: Clone,
    // Operations
    Self: Add<Self>,
    Self: Sub<Self>,
    Self: Mul<Self>,
    Self: Div<Self>,
{
    fn as_f32(&self) -> f32;
    fn from_f32(val: f32) -> Self;
}

impl DTYPE for f32 {
    fn as_f32(&self) -> f32 {
        *self
    }

    fn from_f32(val: f32) -> Self {
        val
    }
}

pub trait Tensor<T: DTYPE> {
    fn as_slice(&self) -> &[T];
}

impl<T: DTYPE> Tensor<T> for Vec<T> {
    fn as_slice(&self) -> &[T] {
        self.as_slice()
    }
}
