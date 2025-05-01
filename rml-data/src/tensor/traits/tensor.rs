use crate::tensor::{
    ops::dtype::dtypeops,
    shape::{deep::Deep, shallow::Shallow},
};

use super::dtype::dtype;

pub trait TensorBound
where
    Self::inner: dtypeops,
{
    #[allow(non_camel_case_types)]
    type inner: dtype; // Resulting Value
}

impl<T> TensorBound for Shallow<T>
where
    T: dtype + dtypeops,
{
    type inner = T;
}

impl<T> TensorBound for Deep<T>
where
    T: TensorBound,
{
    type inner = T;
}
