use super::super::traits::dtype::dtype;
use std::ops::{Add, Div, Mul, RangeTo};


    
/*
pub trait Sigmoid <T: dtype, Output = T / dtype >

*/




/*
ops!(trait Power (Mul<Self>), (
    fn pow(&self, i){
        Self::from_f32(self.as_f32().powf(i.as_f32()))
    })
);

use std::f32::consts::E;



ops!(trait Relu (), (
    fn relu(&self){
        if self.as_f32() > 0.0 {
            self.clone()
        } else {
            Self::from_f32(0.0)
        }
    })
);

// TanH
// (e^x - e^-x) / (e^x + e^-x)
ops!(trait TanH (), (
    fn tanh(&self){
        let t = self.clone().as_f32();
        Self::from_f32((E.powf(t) - E.powf(-1.0 * t))/(E.powf(t) + E.powf(-1.0 * t)))
    })
); */

// Other Impl




// T$(:$($con:tt)+*)?
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
            type Output: Clone;
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
    fn sigmoid(&self) -> Self::Output;
});


impl <T:dtype> Sigmoid for T {
    type Output = T;

    fn sigmoid(&self) -> Self::Output {
        Self::Output::from_f32((1.0)/(1.0+E.powf(-1.0*self.as_f32())))
    }
    
    
}


