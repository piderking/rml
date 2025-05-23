use std::{
    fmt::Display,
    ops::{Index, IndexMut, Range},
    slice::Iter,
};

use super::{index::TensorIndex, traits::tensor::TensorBound};

pub struct StringTensor {
    pub(crate) data: Vec<String>,
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
}

impl TensorIndex for StringTensor {}
impl TensorBound for StringTensor {
    type inner = String;

    fn apply<F: Fn(&String) -> Self::inner>(self, f: F) -> Self {
        Self::from(self.data.iter().map(f).collect())
    }

    fn mutate<F: Fn(&Self::inner) -> Self::inner>(&mut self, f: F) -> () {
        for s in self.data.iter_mut() {
            *s = f(s)
        }
    }

    fn push(&mut self, item: Self::inner) -> &Self {
        self.data.push(item);
        self
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn get(&self, i: usize) -> Option<&Self::inner> {
        self.data.get(i)
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut Self::inner> {
        self.data.get_mut(i)
    }

    fn combine(&mut self, other: Self) {
        self.data.extend_from_slice(
            Vec::with_capacity(i32::abs(self.len() as i32 - other.len() as i32) as usize)
                .as_slice(),
        );

        for (i, t) in self.data.iter_mut().zip(other.into_iter()) {
            *i = format!("{}{}", i.clone(), t.clone());
        }
    }
}

impl<'a> IntoIterator for &'a StringTensor {
    type IntoIter = Iter<'a, String>;
    type Item = &'a String;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl Display for StringTensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.data
                .iter()
                .map(|f| format!("{}", f))
                .collect::<Vec<_>>()
                .join(",  ")
        )
    }
}
