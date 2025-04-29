#[macro_export]
macro_rules! tensorm {
        //($x:expr, $($y:expr),+)
        [$($x:expr),*] => {
            Tensor::new(
                vec![$($x),*]
            )
        };
    }
