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
    fn push(&mut self, item: Self::inner) -> &Self;
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> Option<&Self::inner>;
    fn get_mut(&mut self, i: usize) -> Option<&mut Self::inner>;
    fn combine(&mut self, other: Self);
}

impl<T: dtype> TensorIndex for Tensor<'_, T> {}

impl<T> TensorBound for Tensor<'_, T>
where
    T: dtype + dtypeops,
{
    type inner = T;

    fn apply<F: Fn(&T) -> T>(self, f: F) -> Self {
        Tensor::from(self.data.iter().map(f).collect())
    }

    fn mutate<F: Fn(&T) -> T>(&mut self, f: F) -> () {
        for t in self.data.iter_mut() {
            *t = f(t)
        }
    }

    fn push(&mut self, item: Self::inner) -> &Self {
        self.data.push(item);
        self
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn get(&self, i: usize) -> Option<&T> {
        self.data.get(i)
    }
    fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        self.data.get_mut(i)
    }
    fn combine(&mut self, other: Self) -> () {
        self.data.extend_from_slice(
            Vec::with_capacity(i32::abs(self.len() as i32 - other.len() as i32) as usize)
                .as_slice(),
        );

        for (i, t) in self.data.iter_mut().zip(other.into_iter()) {
            *i = i.clone() + t.clone();
        }
    }
}
