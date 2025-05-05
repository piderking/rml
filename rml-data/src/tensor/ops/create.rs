use super::super::traits::dtype::dtype;
use std::ops::{Add, Div, Mul, RangeTo};
macro_rules! ops {
    /*
       ($i: ident) => {
        pub trait $i  {
            type Output: Clone;
        }
    }; */
    (trait $i: ident ($($tt:tt),*),
    ($(
        fn $fn_name:ident(&$self_ident:ident$(, $arg:ident),* ) $body:block
    )*)


    ) => {
        pub trait $i

        where
        Self: Sized + Clone,
        Self: dtype,
        $(
            Self:$tt<Self, Output = Self>,
        )*
        {

            $(
                fn $fn_name(&$self_ident, $($arg: Self),*) -> Self { //<Self as $i>::Output
                    $body
                }
            )*

            fn from_f32(t: f32) -> Self;



        }

        impl $i for f32 {
            fn from_f32(t: f32) -> Self {
                t
            }
        }


    };



}

ops!(trait Power (Mul), (
    fn raise(&self, i){
        let mut t = self.clone();
        for _ in 0.. {
            t = t.mul(i.clone());
        }
        t
    })
);

use std::f32::consts::E;

ops!(trait Sigmoid (Mul, Div), (
    fn sigmoid(&self){
        Self::from_f32((1.0)/(1.0+E.powf(-1.0*self.as_f32())))
    })
);

ops!(trait Relu (), (
    fn relu(&self){
        if self.as_f32() > 0.0 {
            self.clone()
        } else {
            Self::from_f32(0.0)
        }
    })
);

//(e^x - e^-x) / (e^x + e^-x)
ops!(trait TanH (), (
    fn tanh(&self){
        let t = self.clone().as_f32();
        Self::from_f32((E.powf(t) - E.powf(-1.0 * t))/(E.powf(t) + E.powf(-1.0 * t)))
    })
);

// Other Impl
impl Sigmoid for i32 {
    fn from_f32(t: f32) -> Self {
        t as i32
    }
}
