use super::{shape::tensor::Tensor, shape::tensor::MutTensor, traits::dtype::dtype};

/// Vec -> Tensor
impl <T: dtype> From<Vec<T>> for Tensor<'_, T> {
    fn from(value: Vec<T>) -> Self {
        Tensor::from(value)
    }
}

///  Tensor -> Tensor Mut
impl <'a, T: dtype> Into<MutTensor<'a, T>> for Tensor<'a, T> {
    fn into(self) -> MutTensor<'a, T> {
        MutTensor::from(self)
    }
}
