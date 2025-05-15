use std::ops::{Add, Div, Mul, Sub};

use super::create::{Relu, Sigmoid};

#[allow(non_camel_case_types)]
pub trait dtypeops: Clone
where
    // Add Boundary
    Self::Input: Add<Self::Input, Output = Self::Input>,
    Self::Input: Sub<Self::Input, Output = Self::Input>,
    Self::Input: Mul<Self::Input, Output = Self::Input>,
    Self::Input: Div<Self::Input, Output = Self::Input>,
    Self::Input: Sigmoid,
    Self::Input: Relu,

    // Equals
    Self::Input: PartialEq

{
    type Input;
}

macro_rules! req_ops {
    (impl ($tr:ty) for ($($type:ty),*)) => {
        $(impl $tr for $type where
            $type: Add<$type, Output = $type>,
            $type: Sub<$type, Output = $type>,
            $type: Mul<$type, Output = $type>,
            $type: Div<$type, Output = $type>,
            $type: Sigmoid,
            $type: Clone
        {
            type Input = $type;
        })*
    };

}

req_ops!(impl (dtypeops) for (i32, f32));
