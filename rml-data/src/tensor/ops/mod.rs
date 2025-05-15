use super::traits::tensor::TensorBound;
pub mod create;
pub mod dtype;
mod len;
mod tensor;

// temporary
mod tests {

    use crate::{
        tensor::{
            ops::create::{Relu, Sigmoid, Softmax},
            shape::tensor::Tensor,
            *,
        },
        tensorm,
    };

    #[test]
    pub fn sigmoid() {
        let t = Tensor::from(vec![1.0, 2.0]);
        assert_eq!(tensorm!(0.7310586, 0.880797), t.sigmoid())
    }
    #[test]
    pub fn relu() {
        let t = tensorm!(-1.0, 2.0);
        assert_eq!(tensorm!(0.0, 2.0), t.relu())
    }
    #[test]
    pub fn softmax() {
        let t = tensorm!(1.0, 2.0).softmax();
        assert_eq!(tensorm!(0.33333334, 0.6666667), t)
    }
}
