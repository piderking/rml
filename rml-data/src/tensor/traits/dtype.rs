use std::{marker::PhantomData, ops::Add};

use crate::tensor::{ops::dtype::dtypeops, shape::shallow::Shallow};

use super::super::len::TensorSize;

// Trait Bound for primitives and objects
// Bind the types that can be tensorable

pub trait Combine {}

//dtypeops!(f32);

#[allow(non_camel_case_types)]
pub trait dtype: dtypeops
where
    Self: Add<Self, Output = Self>,
{
    fn default() -> Self;
    fn to_value(&self) -> Self;
    fn size(&self) -> Option<TensorSize>;
    fn is_tensor() -> bool {
        false
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

            fn size(&self) -> Option<TensorSize> {
                Option::None
            }
        }
    };
}

impl<T> dtypeops for Shallow<T>
where
    T: dtype,
    T: Add<T>,
{
    type Input = T;
}

dtype!(i32, 0);
dtype!(f32, 0.0);
