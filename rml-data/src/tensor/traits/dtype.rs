use std::{marker::PhantomData, ops::Add};

use crate::tensor::ops::dtype::dtypeops;

use super::super::len::TensorSize;

// Trait Bound for primitives and objects
// Bind the types that can be tensorable

trait Primitive {
    fn into_i32(&self) -> i32;
    fn into_f32(&self) -> f32;
}

macro_rules! convert {
    ($ty:ty, $self_ident:ident, into_i32:$body:block, into_f32: $body2:block) => {
        impl Primitive for $ty {
            fn into_i32(&$self_ident) -> i32 {
                $body
            }
            fn into_f32(&$self_ident) -> f32 {
                $body2
            }
        }
    };
}

#[allow(non_camel_case_types)]
pub trait dtype: dtypeops
where
    Self: Primitive,
    Self: Add<Self, Output = Self>,
{
    fn default() -> Self;
    fn to_value(&self) -> Self;
    fn as_i32(&self) -> i32 {
        self.into_i32()
    }
    fn as_f32(&self) -> f32 {
        self.into_f32()
    }
}

// Trait Defintion for Basic Numbers
macro_rules! dtype {
    ($ty:ty, $val:expr) => {
        impl dtype for $ty {
            fn default() -> Self {
                $val
            }
            fn to_value(&self) -> Self {
                *self
            }
        }
    };
}

convert!(f32, self, into_i32:{ *self as i32 }, into_f32:{ *self });
convert!(i32, self, into_i32:{ *self }, into_f32:{ *self as f32 });

dtype!(i32, 0);
dtype!(f32, 0.0);
