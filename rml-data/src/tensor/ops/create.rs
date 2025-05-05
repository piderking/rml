use std::ops::{Add, Mul};
use super::super::traits::dtype::dtype;
macro_rules! ops {
    /*
       ($i: ident) => {
        pub trait $i  {
            type Output: Clone;
        }
    }; */
    (trait $i: ident, ($($tt:tt),*), 
    ($(
        fn $fn_name:ident(&$self_ident:ident, $($arg:ident),* ) $body:block
    )*)
   
    ) => {
        pub trait $i 
        
        where 
        <Self as $i>::Output: dtype,
        $(
            <Self as $i>::Output: $tt<<Self as $i>::Output>,
            for <'a> &'a Self: $tt<<Self as $i>::Output, Output=<Self as $i>::Output>
        ),*,
        {
            type Output: Clone;
            
            $(
                fn $fn_name(&$self_ident, $($arg: <Self as $i>::Output),*) -> <Self as $i>::Output { //<Self as $i>::Output
                    $body
                }
            )*
           
        }
    };
   
    
    
}

ops!(trait Power, (Mul), (
    fn raise(&self, i, t) {
        for _n in ..t {}
        self.mul(i)
    }
));