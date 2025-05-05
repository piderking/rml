#[macro_export]
macro_rules! tensorm {
        //($x:expr, $($y:expr),+)
        [$($x:expr),*] => {
             
            Tensor::from(
                vec![$($x),*]
            )
        };

    }
