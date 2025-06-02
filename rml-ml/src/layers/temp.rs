use rml_data::tensor::{
    shape::tensor::Tensor,
    traits::{dtype::dtype, tensor::TensorBound},
};

use crate::empty::Empty;

use super::create::Layer;

pub struct Temp {
    bias: f32,
}

impl Temp {
    pub fn new(b: f32) -> Box<Self> {
        Box::new(Temp {
            bias: b
        })
    }
}

impl<'a, T: dtype + 'a> Layer<'a, T> for Temp {
    type Output = Tensor<'a, T>;
    
    fn forward(&self, tensor: Tensor<'a, T>) -> Self::Output {
        tensor.apply(|f| f.clone() + T::from_f32(self.bias))
    }
    
    fn fill(&mut self, arg: super::create::LayerArgument<'a, T>) -> () {
        todo!()
    }
    
    

}

impl Empty for Temp {
    fn empty() -> Self {
        Temp {
            bias: 0.0
        }
    }
}