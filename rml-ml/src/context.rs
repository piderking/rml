//pub trait Context {}

use rml_data::tensor::{
    shape::tensor::Tensor,
    traits::{dtype::dtype, tensor::TensorBound},
};

use crate::layers::create::Layer;

pub trait Model<'a> {
    type Output: TensorBound;
    fn forward(&mut self) -> &mut Self;
}

pub enum LayerState<'a> {
    Input(),
    Layer(Box<dyn Layer<'a, f32, Output = Tensor<'a, f32>>>),
    Output(),
}

pub struct Linear<'a> {
    position: usize,
    layers: Vec<LayerState<'a>>,
}

impl<'a> Linear<'a> {
    pub fn new(mut layers: Vec<LayerState<'a>>) -> Self {
        layers.insert(0, LayerState::Input());
        layers.push(LayerState::Output());
        Linear {
            position: 0,
            layers: layers,
        }
    }
}
impl<'a> Model<'a> for Linear<'a> {
    type Output = Tensor<'a, f32>;

    fn forward(&mut self) -> &mut Self {
        match (self.position, self.layers.get_mut(self.position)) {
            (0, LayerState::Input())
        };

        self
    }
}
