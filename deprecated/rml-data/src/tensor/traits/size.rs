use crate::tensor::{len::TensorSize, shape::tensor::Tensor};

use super::{dtype::dtype, tensor::TensorBound};

pub trait TensorSizable {
    #[allow(dead_code)]
    fn to_size(&self) -> TensorSize;
    fn is_deep(&self) -> bool {
        false
    }
    //fn valid_index(&self) -> bool;
}

// Implentations
#[warn(deprecated)]
impl<T> TensorSizable for Tensor<'_, T>
where
    T: dtype,
{
    fn to_size(&self) -> TensorSize {
        TensorSize::new(Box::new([self.len()]))
    }
}
