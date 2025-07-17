use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use super::super::len::TensorSize;
use crate::tensor::types::tstring::TString;
use crate::{
    tensor::ops::{
        create::{Relu, Sigmoid},
        dtype::dtypeops,
    },
    tstring,
};

// Trait Bound for primitives and objects
// Bind the types that can be tensorable

#[allow(non_camel_case_types)]
pub trait dtype: dtypeops
where
    Self: Display,
{
    fn default() -> Self;
    fn to_value(&self) -> Self;
    fn is_string() -> bool;

    // Conversion
    fn as_f32(&self) -> f32;
    fn from_f32(t: f32) -> Self;
    fn to<T: dtype>(self) -> T;
    fn from_string(s: String) -> Self;
}

// Trait Defintion for Basic Numbers
macro_rules! dtype {
    ($ty:ty, $val:expr, {$($i:item)*}) => {
        impl dtype for $ty {
            fn default() -> Self {
                $val
            }
            fn to_value(&self) -> Self {
                *self
            }
            fn to <T: dtype> (self) -> T {
                T::from_f32(self as f32)
            }
            fn is_string() -> bool {false}
            fn from_string(s: String) -> Self {
                todo!()
            }

            $($i)*
        }
    };
}

//convert!(f32, self, into_i32:{ *self as i32 }, into_f32:{ *self });
//convert!(i32, self, into_i32:{ *self }, into_f32:{ *self as f32 });

dtype!(i32, 0, {
    fn from_f32(f: f32) -> Self {
        f as i32
    }
    fn as_f32(&self) -> f32 {
        *self as f32
    }
});
dtype!(f32, 0.0, {
    fn from_f32(f: f32) -> Self {
        f
    }
    fn as_f32(&self) -> f32 {
        *self
    }
});

impl dtype for TString {
    fn default() -> Self {
        TString::new("".to_string())
    }

    fn to_value(&self) -> Self {
        todo!()
    }

    fn as_f32(&self) -> f32 {
        todo!()
    }

    fn from_f32(t: f32) -> Self {
        todo!()
    }

    fn to<T: dtype>(self) -> T {
        T::from_string(self.into())
    }
    fn is_string() -> bool {
        true
    }
    fn from_string(s: String) -> Self {
        Self::new(s)
    }
}
/*
dtype!(TString, TString::new(), {
    fn from_f32(f: f32) -> Self {
            f
    }
        fn as_f32(&self) -> f32 {
            *self
        }
})



*/
