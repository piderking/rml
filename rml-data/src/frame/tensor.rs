use crate::tensor::{shape::tensor::Tensor, stringtensor::StringTensor, traits::dtype::dtype};

pub enum TensorWrapper <'a, T: dtype> {
    StringTensor(Tensor<'a, T>),
    ValueTensor( StringTensor )
}

impl <'a, T: dtype> Into<Option<Tensor<'a, T>>> for TensorWrapper<'a, T> {
    fn into(self) -> Option<Tensor<'a, T>> {
        match self {
            TensorWrapper::StringTensor(tensor) => Option::Some(tensor),
            TensorWrapper::ValueTensor(_) => Option::None,
        }
    }
}
impl  <_T: dtype> Into<Option<StringTensor>> for TensorWrapper<'_, _T> {
    fn into(self) -> Option<StringTensor> {
        match self {
            TensorWrapper::StringTensor(_) => Option::None,
            TensorWrapper::ValueTensor(tensor) => Option::Some(tensor),
        }
    }
}