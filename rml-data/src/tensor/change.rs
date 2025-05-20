use super::{shape::tensor::Tensor, traits::dtype::dtype};

pub struct DtypeChange <T> (pub T);
// Convert from two types of tensor
impl <'a, T1:dtype, T2: dtype> From<DtypeChange<Tensor<'a, T2>>> for  Tensor<'a, T1> {
    fn from(value: DtypeChange<Tensor<'a, T2>>) -> Self {
        Self::from(value.0.into_iter().map(|f|T1::from_f32(f.clone().as_f32())).collect())

    }
} 