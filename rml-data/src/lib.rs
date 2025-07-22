pub mod func;
pub mod ops;
pub mod tensor;

#[cfg(test)]
pub mod tests {
    use crate::tensor::Tensor;

    #[test]
    pub fn test() {
        let v = vec![1.0, 1.0];

        println!("{:?}", <Vec<f32> as Tensor<f32>>::as_slice(&v));
        println!("{:?}", v.as_slice())
    }
}
