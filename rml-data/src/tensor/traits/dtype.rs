use std::{marker::PhantomData, ops::Add};


use crate::tensor::ops::dtype::dtypeops;

use super::super::len::TensorSize;

// Trait Bound for primitives and objects
// Bind the types that can be tensorable

trait Primitive {
    fn into_i32(&self) -> i32;
}

macro_rules! convert {
    ($ty:ty, $self_ident:ident, $body:block) => {
        impl Primitive for $ty {
            fn into_i32(&$self_ident) -> i32 {
                $body
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

convert!(f32, self, {
    *self as i32
});
convert!(i32, self, {
    *self
});

dtype!(i32, 0);
dtype!(f32, 0.0);
