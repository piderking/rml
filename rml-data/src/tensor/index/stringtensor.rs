use std::ops::{Index, IndexMut, Range};

use crate::tensor::stringtensor::StringTensor;

impl Index<usize> for StringTensor {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        self.data_index(index)
    }
}

impl IndexMut<usize> for StringTensor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data_index_mut(index)
    }
}

impl Index<Range<usize>> for StringTensor {
    type Output = [String];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        self.data_range_index(index).unwrap()
    }
}

impl IndexMut<Range<usize>> for StringTensor {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        self.data_mut_range_index(index).unwrap()
    }
}
