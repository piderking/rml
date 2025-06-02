//pub trait Context {}

use rml_data::tensor::{
    shape::tensor::Tensor,
    traits::{dtype::dtype, tensor::TensorBound},
};

use crate::layers::create::Layer;

pub trait Model<'a> {
    type Output: TensorBound;
    fn model(&mut self, input: Vec<Self::Output>) -> Self::Output;
}

pub enum LayerState<'a> {
    Input(usize),
    Layer(Box<dyn Layer<'a, f32, Input = Option<f32>, Output = Tensor<'a, f32>>>),
    Output(),
}

