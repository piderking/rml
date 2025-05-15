pub mod deep;
pub mod tensor;
//mod tesnorrefcell;

mod tests {
    use crate::{
        tensor::{ops::create::Sigmoid, shape::tensor::Tensor},
        tensorm,
    };

    #[test]
    pub fn apply() {
        let t = Tensor::from(vec![1.0, 2.0]);
        assert_ne!(t, t.clone().apply(|f| f.sigmoid()))
    }
    #[test]
    pub fn mutates() {
        let mut t = Tensor::from(vec![1.0, 2.0]);
        t.mutate(|f| f.sigmoid());
        assert_ne!(t, tensorm!(1.0, 2.0))
    }
}
