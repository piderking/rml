use std::ops::{Index, IndexMut, Range};

use crate::tensor::{shape::tensor::Tensor, stringtensor::StringTensor, traits::dtype::dtype};

impl Index<usize> for StringTensor {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl IndexMut<usize> for StringTensor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        todo!()
    }
}

impl Index<Range<usize>> for StringTensor {
    type Output = [String];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        todo!()
    }
}

impl IndexMut<Range<usize>> for StringTensor {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        todo!()
    }
}
