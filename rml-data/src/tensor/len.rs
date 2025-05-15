use std::{
    cell::RefCell,
    fmt::{Display, Error, Formatter},
    ops::Add,
    rc::Rc,
};

use super::{shape::{deep::Deep, }, traits::{dtype::dtype, tensor::TensorBound}};





/*impl<T: dtype> TensorSizable for &dyn TensorBound<T = T> {
    fn to_size(&self) -> TensorSize {
        self.size
    }
} */

#[derive(Debug, Clone)]
pub struct TensorSize {
    pub(crate) size: Box<[usize]>,
    #[allow(dead_code)]
    pub(crate) deep: bool,
}

impl TensorSize {
    pub fn new(t: Box<[usize]>) -> TensorSize {
        TensorSize {
            deep: t.len() > 1,
            size: t,
        }
    }
    
}




impl Add<Box<[usize]>> for TensorSize {
    type Output = Box<[usize]>;

    fn add(self, ts: Box<[usize]>) -> Self::Output {
        // ts [10] + TensorSize
        ts.iter()
            .chain(self.size.iter())
            .map(|d| d.clone())
            .collect()
    }
}

// Conversion
impl From<Box<[usize]>> for TensorSize  {
    fn from(value: Box<[usize]>) -> Self {
        TensorSize::new(value)
    }
}
impl From<&TensorSize> for Box<[usize]> {
    fn from(val: &TensorSize) -> Self {
        val.clone().size
    }
}


// Display
impl Display for TensorSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let elements = self
            .size
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}]", elements)
    }
}


