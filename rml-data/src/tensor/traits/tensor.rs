use crate::tensor::{
    ops::dtype::dtypeops,
    shape::{deep::Deep, tensor::Tensor},
};

use super::dtype::dtype;

pub trait TensorBound
where
    Self::inner: dtypeops,
{
    #[allow(non_camel_case_types)]
    type inner: dtype; // Resulting Valu

}

impl<T> TensorBound for Tensor<'_, T>
where
    T: dtype + dtypeops,
{
    type inner = T;
    
}

impl<T, U> TensorBound for Deep<T>
where
    U: dtype,
    T: TensorBound<inner = U>,
{
    type inner = U;
  
}
