use std::ops::{Index, IndexMut, Range};

use crate::tensor::{shape::tensor::Tensor, traits::dtype::dtype};

impl<T: dtype> Index<usize> for Tensor<'_, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}
impl<T: dtype> IndexMut<usize> for Tensor<'_, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl<T: dtype> Index<Range<usize>> for Tensor<'_, T> {
    type Output = [T];
    fn index(&self, r: Range<usize>) -> &Self::Output {
        self.data.index(r)
    }
}

impl<T: dtype> IndexMut<Range<usize>> for Tensor<'_, T> {
    fn index_mut(&mut self, r: Range<usize>) -> &mut Self::Output {
        self.data.index_mut(r)
    }
}
