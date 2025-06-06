use std::clone;

use itertools::izip;
use rml_data::{tensor::{
    shape::tensor::Tensor,
    traits::{dtype::dtype, tensor::TensorBound},
}, tensorm};

use crate::{empty::Empty, layers::create::{LayerArgument, WB}};
use super::create::Layer;

pub struct Deep <'a> {
    weight: Tensor<'a, f32>,
    bias: Tensor<'a, f32>,
}

impl <'a> Deep <'a>{
    pub fn new(weights: Tensor<'a, f32>, bias: Tensor<'a, f32>) -> Box<Self> {
        Box::new(Deep {
            bias: bias,
            weight: weights
        })
    }
}

impl<'a, T: dtype + 'a> Layer<'a, T> for Deep<'a> {
    type Output = Tensor<'a, T>;
    
    fn forward(&self, tensor: Tensor<'a, T>) -> Self::Output {
        /*Tensor::<T>::from(
            tensor.into_iter().zip(self.weight.into_iter()).zip(self.bias.into_iter()).map(|((i, w), b)| {
           T::from_f32((i.clone().as_f32()*w.clone()) + b.clone())

        }).collect::<Vec<_>>()
        ) */
       izip!(&tensor, &self.weight, &self.bias).map(|(i, w, b)| T::from_f32((i.clone().as_f32()*w.clone()) + b.clone())).collect::<Vec<T>>().into()
    }
    
    fn fill(&mut self, arg:LayerArgument<'a, T>) -> () {
        match arg {
            LayerArgument::WB(wb)=>{
                self.weight = wb.weight.to();
                self.bias = wb.bias.to();
            },
            _ => panic!("Unsupported!")
        }
    }
    
    

}

impl <'a> Empty <'a> for Deep <'a> {
    fn empty(ten: &Tensor<'a, f32>) -> Box<Self> {
        Box::new(Deep {
            bias: <Tensor<'a, f32>>::with_capacity(ten.len(), 0.0),
            weight: <Tensor<'a, f32>>::with_capacity(ten.len(), 1.0),
        })
    }
}