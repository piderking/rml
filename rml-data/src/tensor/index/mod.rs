mod stringtensor;
mod tensor;

use std::ops::{Index, IndexMut, Range};

pub trait TensorIndex
where
    Self: Index<usize>,
    Self: IndexMut<usize>,
    Self: Index<Range<usize>>,
    Self: IndexMut<Range<usize>>,
{
}
