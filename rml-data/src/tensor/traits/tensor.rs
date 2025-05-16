use std::ops::Index;

use crate::tensor::{
    index::TensorIndex,
    ops::dtype::dtypeops,
    shape::{deep::Deep, tensor::Tensor},
};

use super::dtype::dtype;

pub trait TensorBound
where
    Self: TensorIndex,
{
    #[allow(non_camel_case_types)]
    type inner; // Resulting Valu
    fn apply<F: Fn(&Self::inner) -> Self::inner>(self, f: F) -> Self;
    fn mutate<F: Fn(&Self::inner) -> Self::inner>(&mut self, f: F) -> ();
}

impl<T: dtype> TensorIndex for Tensor<'_, T> {}

impl<T> TensorBound for Tensor<'_, T>
where
    T: dtype + dtypeops,
{
    type inner = T;

    fn apply<F: Fn(&T) -> T>(self, f: F) -> Self {
        self.apply(f)
    }

    fn mutate<F: Fn(&T) -> T>(&mut self, f: F) -> () {
        self.mutate(f);
    }
}
