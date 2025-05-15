use crate::tensor::traits::tensor::TensorBound;

use super::super::traits::dtype::dtype;
use super::dtype::dtypeops;
use std::f32::consts::E;
use std::ops::{Add, Div, Mul, RangeTo};

#[macro_export]
macro_rules! op {
    /*
    (fn $fn:ident (&$self_ident:ident $(, $arg:ident: $arg_ty:ty)*) -> $ret:ty $(where $($where:ty:$wherer:tt)+)?) => {
        fn $fn(&$self_ident, $( $arg: $arg_ty ),*) -> $ret $(where $($where:$wherer),+)?;
    };

    (fn $fn:ident (&$self_ident:ident $(, $arg:ident: $arg_ty:ty)*) -> $ret:ty $(where $($where:ty:$wherer:tt)+)? $body: block) => {
        fn $fn(&$self_ident, $( $arg: $arg_ty ),*) -> $ret $(where $($where:$wherer),+)? $body
    };
     */
($name: ident<T$(:$($con:tt),*)?> {$($i:item)*}) => {
        pub trait $name <T: Clone $($(+ $con)*)?>
        // Trait Bounds For SElf
        {
            // No Trait Bonds on rest
            type Output: Clone;
            $($i)*
        }
    };
     ($name:ident {$($i:item)*}) => {
        pub trait $name
        {
            // No Trait Bonds on rest
            $($i)*
        }
    };
    ($name: ident<T$(:$($con:tt),*)?> {$($i:item)*}) => {
        pub trait $name <T: Clone $($(+ $con)*)?>
        {
            // No Trait Bonds on rest
            type Output: Clone;
            $($i)*
        }
    };
    ($name: ident<T:$($con:tt),*><Output = $($assoc:tt),*>) => {
        pub trait $name <T: Clone $(+ $con)*>
        {
            // No Trait Bounds on rest
            type Output: Clone $(+ $assoc)*;
        }
    };

}

op!(Sigmoid {
    fn sigmoid(&self) -> Self;
});

impl<T: dtype> Sigmoid for T {
    fn sigmoid(&self) -> Self {
        Self::from_f32((1.0) / (1.0 + E.powf(-1.0 * self.as_f32())))
    }
}

op!(TanH {
    fn tanh(&self) -> Self;
});

impl<T: dtype> TanH for T {
    fn tanh(&self) -> Self {
        let t = self.clone().as_f32();
        Self::from_f32((E.powf(t) - E.powf(-1.0 * t)) / (E.powf(t) + E.powf(-1.0 * t)))
    }
}

op!(Relu {
    fn relu(&self) -> Self;
});

impl<T: dtype> Relu for T {
    fn relu(&self) -> Self {
        if self.as_f32() >= 0.0 && self.as_f32() <= 1.0 {
            self.clone()
        } else {
            Self::from_f32(0.0)
        }
    }
}

// Tensor Opperation Macros
/*
op!(Combine <T: TensorBound> { // Combine // <T> with
    fn combine(&mut self, other: T) -> Self;
});
// Tensor Opperation Macros
op!(CombineMul <T: TensorBound> { // Combine // <T> with
    fn combine(&mut self, other: T) -> Self;
});

*/
