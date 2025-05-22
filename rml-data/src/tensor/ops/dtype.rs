use std::ops::{Add, Div, Mul, Sub};

use crate::tensor::types::tstring::TString;

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
    (impl dtypeops for ($($type:ty),*)) => {
        $(impl dtypeops for $type where
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

req_ops!(impl dtypeops for (i32, f32));
impl dtypeops for TString
where
    TString: Add<TString, Output = TString>,
    TString: Sub<TString, Output = TString>,
    TString: Mul<TString, Output = TString>,
    TString: Div<TString, Output = TString>,
    TString: Sigmoid,
    TString: Clone,
    TString: PartialEq,
{
}

macro_rules! ts_err {
    (output$(($ty:tt, $fun:ident)),+) => {
        $(
            impl $ty<TString> for TString {
                type Output = TString;

                fn $fun(self, _rhs: TString) -> Self::Output {
                    panic!("Can't Add TStrings Together")
                }
            }
        )+


    };
    (self $(($ty:tt, $fun:ident)),+) => {
        $(
            impl $ty for TString {

                fn $fun(&self) -> Self {
                    panic!("Can't Add TStrings Together")
                }
            }
        )+


    };
}

ts_err!(output(Add, add), (Sub, sub), (Mul, mul), (Div, div));
//ts_err!(self (Sigmoid, sigmoid), (Relu, relu));

impl PartialEq<TString> for TString {
    fn eq(&self, other: &TString) -> bool {
        let t: String = self.clone().into();
        let f: String = other.clone().into();
        t.eq(&f)
    }
}
