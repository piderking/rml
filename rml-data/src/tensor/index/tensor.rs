use std::ops::{Index, IndexMut, Range};

use crate::tensor::{shape::tensor::Tensor, traits::dtype::dtype};

impl<T: dtype> Index<usize> for Tensor<'_, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.data_index(index)
    }
}
impl<T: dtype> IndexMut<usize> for Tensor<'_, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data_index_mut(index)
    }
}

impl<T: dtype> Index<Range<usize>> for Tensor<'_, T> {
    type Output = [T];
    fn index(&self, r: Range<usize>) -> &Self::Output {
        self.data_range_index(r).unwrap()
    }
}

impl<T: dtype> IndexMut<Range<usize>> for Tensor<'_, T> {
    fn index_mut(&mut self, r: Range<usize>) -> &mut Self::Output {
        self.data_mut_range_index(r).unwrap()
    }
}
