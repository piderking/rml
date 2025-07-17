use rml_data::tensor::shape::tensor::Tensor;

use crate::context::{LayerState, Model};


pub struct Linear<'a> {
    position: usize,
    layers: Vec<LayerState<'a>>,
}

impl<'a> Linear<'a> {
    pub fn new(mut layers: Vec<LayerState<'a>>) -> Self {
        layers.insert(0, LayerState::Input(0));
        layers.push(LayerState::Output());
        Linear {
            position: 0,
            layers: layers,
        }
    }
}
impl<'a> Model<'a> for Linear<'a> {
    type Output = Tensor<'a, f32>;

    fn model(&mut self, input: Vec<Self::Output>) -> Self::Output {
        
        let mut tensr: Self::Output = input.first().or_else(|| panic!("Must pass valid input for model!")).unwrap().clone();
        for layers in self.layers.iter() {
                match layers {
                    LayerState::Layer(layer) => {
                        tensr = layer.forward(tensr)
                    },
                    _ => {} // Linear do nothing for input
                }
        };

        tensr
        
    }
}

