use std::ops::{Index, IndexMut, Range};

use super::{index::TensorIndex, traits::tensor::TensorBound};

pub struct StringTensor {
    data: Vec<String>,
}

impl StringTensor {
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn find(&self, s: String) -> Option<usize> {
        for (i, st) in self.data.iter().enumerate() {
            if st.clone() == s {
                return Option::Some(i);
            }
        }
        Option::None
    }
    pub fn new(data: &[&str]) -> Self {
        StringTensor::from(data.iter().map(|f| String::from(*f)).collect())
    }
    pub fn from(data: Vec<String>) -> Self {
        StringTensor { data }
    }
    pub(crate) fn data_index(&self, i: usize) -> &String {
        self.data.index(i)
    }
    pub(crate) fn data_index_mut(&mut self, i: usize) -> &mut String {
        self.data.index_mut(i)
    }
    pub fn get(&self, i: usize) -> Option<&String> {
        self.data.get(i)
    }
    pub fn get_mut(&mut self, i: usize) -> Option<&mut String> {
        self.data.get_mut(i)
    }
    pub unsafe fn get_unchecked(&self, i: usize) -> &String {
        unsafe { self.data.get_unchecked(i) }
    }
    pub unsafe fn get_mut_unchecked(&mut self, i: usize) -> &mut String {
        unsafe { self.data.get_unchecked_mut(i) }
    }

    // range index
    pub(crate) fn data_range_index(&self, r: Range<usize>) -> Option<&[String]> {
        match self.data.as_slice().get(r) {
            Some(n) => Some(n),
            None => None,
        }
    }

    // range mut index
    pub(crate) fn data_mut_range_index(&mut self, r: Range<usize>) -> Option<&mut [String]> {
        match self.data.as_mut_slice().get_mut(r) {
            Some(n) => Some(n),
            None => None,
        }
    }
}

impl TensorIndex for StringTensor {}
impl TensorBound for StringTensor {
    type inner = String;

    fn apply<F: Fn(&String) -> Self::inner>(self, f: F) -> Self {
        Self::from(self.data.iter().map(f).collect())
    }

    fn mutate<F: Fn(&Self::inner) -> Self::inner>(&mut self, f: F) -> () {
        for String in self.data.iter_mut() {
            *String = f(String)
        }
    }
}
