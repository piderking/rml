use std::{
    cell::RefCell,
    fmt::{Display, Error, Formatter},
    ops::Add,
    rc::Rc,
};

use super::{
    dtype::dtype,
    tensor::{Tensor, TensorBound},
};

pub trait TensorSizable {
    #[allow(dead_code)]
    fn to_size(&self) -> TensorSize;
}

impl<T> TensorSizable for Vec<T> {
    fn to_size(&self) -> TensorSize {
        TensorSize::new(Box::new([self.len()]))
    }
}
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
    pub fn from<T: dtype>(t: Tensor<T>) -> TensorSize where {
        /*
             deep: match t.data.borrow().get(0){
            Some(n) => n.is_tensor(),
            None => false,
        }, */
        TensorSize::new(match t.get(0) {
            Some(e) => match e.size() {
                Some(f) => f.add(Box::new([t.data.borrow().len()])),
                None => Box::new([t.data.borrow().len()]),
            },
            None => Box::new([0]),
        })
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

impl From<&TensorSize> for Box<[usize]> {
    fn from(val: &TensorSize) -> Self {
        val.clone().size
    }
}
impl Into<TensorSize> for Box<[usize]> {
    fn into(self) -> TensorSize {
        TensorSize::new(self)
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

// Living Tensor Size
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ActiveTensorSize<T: dtype> {
    data: Rc<RefCell<Vec<T>>>,
}

impl<T: dtype> ActiveTensorSize<T> {
    pub fn new(t: Rc<RefCell<Vec<T>>>) -> ActiveTensorSize<T> {
        ActiveTensorSize { data: t.clone() }
    }
    pub fn fetch(&self) -> TensorSize {
        TensorSize::from(Tensor::from(self.data.clone()))
    }
}
