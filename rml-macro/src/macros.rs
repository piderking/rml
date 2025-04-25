mod shallow {
    macro_rules! tensor {
        ($( $i:expr ),*) => {
            Tensor::new(vec![$i])
        };
    }
}
