use std::ops::{Add, Div, Mul, Sub};

use crate::tensor::{error::TensorOpperationError, shape::shallow::Shallow, traits::dtype::dtype};

macro_rules! tensor_op {
    ($trait:ty, $i:path) => {
        // combine together the two tensors
        impl<T> $trait<Shallow<T>> for Shallow<T>
        where
            T: dtype,
            Shallow<T>: Add<T, Output = Shallow<T>>,
            T: $trait<T, Output = T>,
        {
            type Output = Shallow<T>;

            fn add(mut self, rhs: Shallow<T>) -> Self::Output {
                self.$i(rhs)
            }
        }
    };
}

tensor_op!(Add, combine);
/*
{
                self.combine(rhs)
            } */
