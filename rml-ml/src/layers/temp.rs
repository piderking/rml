use rml_data::tensor::{
    shape::tensor::Tensor,
    traits::{dtype::dtype, tensor::TensorBound},
};

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

}

