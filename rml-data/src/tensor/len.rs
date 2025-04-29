use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use super::{tensor::{Tensor, TensorBound}, tensortype::TensorType};

pub trait TensorSizable{
    fn to_size(&self) -> TensorSize;
}

impl <T>  TensorSizable for Vec<T> {
    fn to_size(&self) -> TensorSize {
        TensorSize::new(Box::new([self.len()]))
    }
}
impl <T: TensorType>  TensorSizable for &dyn TensorBound <T=T> {
    fn to_size(&self) -> TensorSize {
        self.size()
    }
}


#[derive(Debug, Clone)]
pub struct TensorSize {
    size: Box<[usize]>,
    pub deep: bool
}

impl TensorSize {
    pub fn new (t: Box<[usize]>) -> TensorSize{
        TensorSize {
            deep: t.len() > 1,
            size: t,
        }
    }
    pub fn from <T: TensorType> (t: Tensor<T>) -> TensorSize{
        /*
                 deep: match t.data.borrow().get(0){
                Some(n) => n.is_tensor(),
                None => false,
            }, */
        let mut size: Vec<usize> = vec![t.data.borrow().len()];
        let mut d = t.get(0).unwrap_or(T::default());
        
        while d.is_tensor(){
            let f: Tensor<T> = t.as_ref().into();
            size.insert(0, f.data.borrow().len());

            let p = f.get(0);

            if p.is_none(){break;}

            // recurse through structure
            d = p.unwrap();
            
        };

        TensorSize { size: size.iter().as_slice().into(), deep: size.len() > 1 }

    }
}

// Conversion

impl From<TensorSize> for Box<[usize]>{
    fn from(val: TensorSize) -> Self {
        val.size
    }
}
impl Into<TensorSize> for Box<[usize]>{
    fn into(self) -> TensorSize {
        TensorSize::new( self )
    }
}

// Living Tensor Size
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ActiveTensorSize <T: TensorType> {
    data: Rc<RefCell<Vec<T>>>,
}

impl <T: TensorType> ActiveTensorSize <T> {
    pub fn new (t:  Rc<RefCell<Vec<T>>>) -> ActiveTensorSize<T>{
        ActiveTensorSize {
            data: t.clone(),
        }
    }
    pub fn fetch (&self) -> TensorSize {
        TensorSize::from(Tensor::from(self.data.clone()))
    }
    
}

