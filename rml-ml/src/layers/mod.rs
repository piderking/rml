use create::Layer;
/*

Establish Connection Matrix

Linear

Deep

trait Deep


*/
use rml_data::tensor::{shape::tensor::Tensor, traits::dtype::dtype};
use crate::context::LayerState;


impl <'a,> Into<LayerState<'a>> for Box<dyn Layer<'a, f32,  Output = Tensor<'a, f32>>> {
    fn into(self) -> LayerState<'a> {
        LayerState::Layer(self)
    }
}

pub mod create;
pub mod softmax;
pub mod temp;
pub mod deep;