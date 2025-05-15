use super::traits::tensor::TensorBound;
pub mod create;
pub mod dtype;
mod len;
mod tensor;

// temporary
mod tests {

    use crate::tensor::{ops::create::Sigmoid, shape::tensor::Tensor, *};

    #[test]
    pub fn sigmoid() {
        let t = Tensor::from(vec![1.0, 2.0]);
        assert_ne!(t, t.clone().apply(|f| f.sigmoid()))
    }
}
