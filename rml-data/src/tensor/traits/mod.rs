pub mod dtype;
pub mod size;
pub mod tensor;

mod tests {
    use crate::tensor::{ops::create::Sigmoid, shape::tensor::Tensor};

    #[test]
    pub fn apply() {
        let t = Tensor::from(vec![1.0, 2.0]);
        assert_ne!(t, t.clone().apply(|f| f.sigmoid()))
    }
}
