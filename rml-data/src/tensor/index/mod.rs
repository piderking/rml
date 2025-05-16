mod stringtensor;
mod tensor;

use std::{
    ops::{
        Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
    },
    slice::SliceIndex,
};

pub trait TensorIndex
where
    Self: Index<usize>,
    Self: IndexMut<usize>,
    Self: Index<Range<usize>>,
    Self: IndexMut<Range<usize>>,
{
}
