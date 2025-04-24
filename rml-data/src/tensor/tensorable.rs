use super::tensor::{Tensor, TensorBound};

// Trait Bound for primitives and objects
// Bind the types that can be tensorable
pub trait Tensorable: Copy {
    type Result: TensorBound;

    fn to_value(&self) -> Self;
    fn to_tensor(&self) -> Self::Result;
}

impl Tensorable for i32 {
    type Result = Tensor<i32>;

    fn to_value(&self) -> Self {
        *self
    }

    fn to_tensor(&self) -> Self::Result {
        Tensor::new(vec![*self])
    }
}

impl Tensorable for f32 {
    type Result = Tensor<f32>;

    fn to_value(&self) -> Self {
        *self
    }
    fn to_tensor(&self) -> Self::Result {
        Tensor::new(vec![*self])
    }
}
