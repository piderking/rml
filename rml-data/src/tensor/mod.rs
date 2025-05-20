pub mod change;
mod conv;
pub mod error;
mod index;
mod len;
pub mod macros;
pub mod ops;
pub mod shape;
pub mod stringtensor;
pub mod traits;
//pub mod wrapper;
#[cfg(test)]
mod tests {
    use crate::{
        tensor::{shape::tensor::Tensor, traits::tensor::TensorBound},
        tensorm,
    };

    #[test]
    pub fn initalization() {
        assert_eq!(Tensor::from(vec![1, 1]), Tensor::new(vec![1, 1].as_slice()))
    }
    #[test]
    pub fn macrotest() {
        assert_eq!(tensorm!(1, 1), Tensor::from(vec![1, 1]))
    }

    #[test]
    pub fn tensor_size() {
        assert_eq!(tensorm!(1, 1).len(), 2)
    }
}
