use super::tensor::{Tensor, TensorBound};

// Trait Bound for primitives and objects
// Bind the types that can be tensorable

pub trait Combine {}
pub trait TensorType: Clone {
    type Result;

    fn default() -> Self;
    fn to_value(&self) -> Self;
    fn to_tensor(&self) -> Self::Result;
    fn is_tensor(&self) -> bool {
        false
    } 
}
pub trait Tensorable {
    type Result;
}



impl TensorType for i32 {
    type Result = Tensor<i32>;

    fn default() -> Self {
        0
    }
    fn to_value(&self) -> Self {
        *self
    }

    fn to_tensor(&self) -> Self::Result {
        Tensor::new(vec![*self])
    }
}

impl TensorType for f32 {
    type Result = Tensor<f32>;

    fn default() -> Self {
        0.0
    }
    fn to_value(&self) -> Self {
        *self
    }
    fn to_tensor(&self) -> Self::Result {
        Tensor::new(vec![*self])
    }
}

impl <T: TensorType> TensorType for Tensor<T> {
    type Result = Tensor<T>;

    fn default() -> Self {
        Tensor::new(vec![])
    }
    fn to_value(&self) -> Self {
        self.as_ref()
    }
    fn to_tensor(&self) -> Self::Result {
        self.as_ref()
    }
}


impl<'a> TensorType for &'a str {
    type Result = Tensor<&'a str>;

    fn default() -> Self {
        &""
    }
    fn to_value(&self) -> Self {
        self
    }
    fn to_tensor(&self) -> Self::Result {
        Tensor::new(vec![self])
    }
}