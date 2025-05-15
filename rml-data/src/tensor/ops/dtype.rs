use std::ops::{Add, Div, Mul, Sub};

use super::create::{Relu, Sigmoid};

#[allow(non_camel_case_types)]
pub trait dtypeops: Clone
where
    // Add Boundary
    Self: Sub<Self, Output = Self>,
    Self: Add<Self, Output = Self>,
    Self: Mul<Self, Output = Self>,
    Self: Div<Self, Output = Self>,
    Self: Sigmoid,
    Self: Relu,
    // Equals
    Self: PartialEq,
{
}

macro_rules! req_ops {
    (impl ($tr:ty) for ($($type:ty),*)) => {
        $(impl $tr for $type where
            $type: Add<$type, Output = $type>,
            $type: Sub<$type, Output = $type>,
            $type: Mul<$type, Output = $type>,
            $type: Div<$type, Output = $type>,
            $type: Sigmoid,
            $type: Clone,
            $type: PartialEq,

        {
        })*
    };

}

req_ops!(impl (dtypeops) for (i32, f32));
