

use std::{  fmt::{Display, Formatter, Error}, marker::PhantomData, slice::{Iter, IterMut}};

use crate::tensor::traits::dtype::dtype;




// Standard 1 Dimensional
#[derive(Debug, Clone)]
pub struct Tensor <'a, T: dtype> {
    data: Vec<T>,
    phn: PhantomData<&'a T>
}
impl <T> Tensor<'_, T> where T: dtype  {
    pub fn new(data: &[T]) -> Self {
        Tensor { data: data.into(), phn: PhantomData }
    }
    pub fn from(data: Vec<T>) -> Self {
        Tensor { data, phn: PhantomData }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    
}


// Implement Itterator
impl<'a, 'b: 'a, T: dtype + 'a> IntoIterator for &'b Tensor<'a, T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Iter<'a, T> {
        self.data.iter()
    }
}

// Mutable Wrapper for the Tensor
pub struct MutTensor <'a, T: dtype> (
    pub(crate) Tensor<'a, T>
);

impl <'a, T: dtype> MutTensor<'a, T> {
    pub fn from (ten: Tensor<'a, T>) -> MutTensor<'a , T>{
        MutTensor(ten)
    }
}
// Implement Itterator
impl<'a, 'b: 'a, T: dtype + 'a> IntoIterator for &'b mut MutTensor<'a, T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> IterMut<'a, T> {
        self.0.data.iter_mut()
    }
}

// Display All of Body
impl<T: dtype + Display> Display for Tensor<'_, T> 

{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let elements = self.data
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}]", elements)
    }
}
