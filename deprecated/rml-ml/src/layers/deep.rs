use std::clone;

use itertools::izip;
use rml_data::{tensor::{
    shape::tensor::Tensor,
    traits::{dtype::dtype, tensor::TensorBound},
}, tensorm};

use crate::{empty::Empty, layers::create::{LayerArgument, WB}};
use super::create::Layer;

pub struct Deep <'a> {
    weight: Vec<Tensor<'a, f32>>,
    bias: Vec<Tensor<'a, f32>>,
}

impl <'a> Deep <'a>{
    pub fn new(weights: Vec<Tensor<'a, f32>>, bias: Vec<Tensor<'a, f32>>) -> Box<Self> {
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

        // Inital x Weight X Bias in All Respect Orginizations
         Tensor::<T>::from(
            tensor.into_iter().zip(self.weight.clone().into_iter()).zip(self.bias.clone().into_iter()).map(|((i, w), b)| {
            let mut f = i.as_f32();

            w.into_iter().zip(b.into_iter()).for_each(|(w, b)| {
                f = f * w + b;
            });

            T::from_f32(f)

         }).collect::<Vec<_>>()
         )

        // apply weights and biases to original vector
    
    }
    
    fn fill(&mut self, arg:LayerArgument<'a, T>) -> () {
        todo!()
    }
    
    

}

impl <'a> Empty <'a> for Deep <'a> {
    fn empty(ten: usize) -> Box<Self> {
        Box::new(Deep {
            bias: (0..ten).into_iter().map(|_f|<Tensor<'a, f32>>::with_capacity(ten, 0.0)).collect(),
            weight: (0..ten).into_iter().map(|_f|<Tensor<'a, f32>>::with_capacity(ten, 0.0)).collect(),
        })
    }
}