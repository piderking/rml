mod loader;
pub mod tensor;

#[cfg(test)]
mod tests {
    use crate::{tensor::shape::tensor::Tensor, tensorm};

    #[test]
    pub fn initalization() {

        assert_eq!(Tensor::from(vec![1, 1]), Tensor::new(vec![1,1].as_slice()))
    }
    #[test]
    pub fn macrotest() {
        assert_eq!(tensorm!(1,1), Tensor::from(vec![1,1]))
    }

    #[test]
    pub fn tensor_size() {
        assert_eq!(tensorm!(1,1).len(), 2)
    }
}
