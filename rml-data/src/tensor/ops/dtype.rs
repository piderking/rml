use crate::tensor::shape::shallow::Shallow;
use std::ops::{Add, Div, Mul, Sub};
#[allow(non_camel_case_types)]
pub trait dtypeops: Clone
// if Shallow Input = Shallow<T>
//+ Sub<Self::Input, Output = Self> //
//+ Mul<Self::Input, Output = Self>
//+ Div<Self::Input, Output = Self>
where
    // Add Boundary
    Self::Input: Add<Self::Input, Output = Self::Input>,
{
    type Input;
}

macro_rules! dtypeops {
    ($type:ty) => {
        // define devops definitions
        impl dtypeops for $type {
            type Input = $type;
        }

        // Assume $type + $type is implemented for all
        // define them your self
        // Impl
        // <$type> + Shallow<$type>
        impl Add<$type> for Shallow<$type> {
            type Output = Self;

            fn add(mut self, rhs: $type) -> Self::Output {
                self.map(|t| t.clone() + rhs)
            }
        }
        impl Sub<$type> for Shallow<$type> {
            type Output = Self;

            fn sub(mut self, rhs: $type) -> Self::Output {
                self.map(|t| t.clone() - rhs)
            }
        }
        impl Mul<$type> for Shallow<$type> {
            type Output = Self;

            fn mul(mut self, rhs: $type) -> Self::Output {
                self.map(|t| t.clone() * rhs)
            }
        }
        impl Div<$type> for Shallow<$type> {
            type Output = Self;

            fn div(mut self, rhs: $type) -> Self::Output {
                self.map(|t| t.clone() / rhs)
            }
        }
    };
}

dtypeops!(i32);
dtypeops!(f32);
