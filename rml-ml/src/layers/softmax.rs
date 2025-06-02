
use rml_data::tensor::ops::create::Softmax as Sft;
use rml_data::tensor::{shape::tensor::Tensor, traits::dtype::dtype};
use rml_data::utils::None;

use crate::empty::Empty;

use super::create::Layer;





#[derive(Debug)]
pub struct Softmax {}
impl Softmax {
    pub fn new() -> Self{
        Softmax {  }
    }
}

impl<'a, T: dtype + 'a> Layer<'a, T> for Softmax {
    type Output = Tensor<'a, T>;

    fn forward(&self, tensor: Tensor<'a, T>) -> Self::Output {
        <Tensor<'a, T> as Sft>::softmax(&tensor)
    }
    
    fn fill(&mut self, arg: super::create::LayerArgument<'a, T>) -> () {
        todo!()
    }
    

}

impl Empty for Softmax {
    fn empty() -> Self {
        Softmax {  }
    }
}