use super::tensor::{Tensor, TensorBound};

// Trait Bound for primitives and objects
// Bind the types that can be tensorable
pub trait Tensorable<'a>: Copy {
    type Result: TensorBound<'a>;

    fn to_value(&self) -> Self;
    fn to_tensor(&self) -> Self::Result;
}

impl<'a> Tensorable<'a> for i32 {
    type Result = Tensor<'a, i32>;

    fn to_value(&self) -> Self {
        *self
    }

    fn to_tensor(&self) -> Self::Result {
        Tensor::new(vec![*self])
    }
}

impl<'a> Tensorable<'a> for f32 {
    type Result = Tensor<'a, f32>;

    fn to_value(&self) -> Self {
        *self
    }
    fn to_tensor(&self) -> Self::Result {
        Tensor::new(vec![*self])
    }
}
