//pub trait Context {}

use rml_data::tensor::{
    shape::tensor::Tensor,
    traits::{dtype::dtype, tensor::TensorBound},
};

use crate::layers::{self, create::Layer};

pub trait ContextFlag<'a> {
    type Output: TensorBound;
    fn next(&mut self) -> Option<Self::Output>;
}
pub struct Input {}

pub enum ContextState<'a> {
    Input(Tensor<'a, f32>),
    Layer(
        (
            Box<dyn Layer<'a, f32, Output = Tensor<'a, f32>>>,
            &'a ContextStruct<'a>,
        ),
    ),
    Output(),
}

pub struct ContextStruct<'a> {
    position: usize,
    layers: Vec<ContextState<'a>>,
}

impl<'a> ContextStruct<'a> {
    pub fn new(v: Vec<ContextState<'a>>) -> Self {
        ContextStruct {
            position: 0,
            layers: v,
        }
    }
}

impl<'a> ContextFlag<'a> for ContextStruct<'a> {
    type Output = Tensor<'a, f32>;
    fn next(&mut self) -> Option<Self::Output> {
        match self.layers.pop() {
            Some(prev) => match self.layers.get_mut(0) {
                Some(layer) => match (prev, layer) {
                    (ContextState::Input(n), ContextState::Layer((a, ctx))) => {
                        a.fill(n);
                        a.layer(ctx);
                        Option::None
                    }
                    (ContextState::Input(n), ContextState::Output()) => {
                        // TODO --> Return Here
                        Option::Some(n)
                    }
                    (ContextState::Input(n), _) => panic!("Only 1 Input!"),
                    (ContextState::Layer((n1, t1)), ContextState::Layer((n2, _t2))) => {
                        n2.fill(n1.layer(t1));
                        Option::None
                    }
                    (ContextState::Layer((n, t)), ContextState::Output()) => {
                        Option::Some(n.layer(t))
                    }
                    (_, ContextState::Input(ns)) => panic!("Input must only be first!"),
                    (ContextState::Output(), _) => panic!("Out must be last"),
                },
                None => {
                    panic!("How did you get here? Like holy shit")
                }
            },
            None => panic!("Context Must Include an Output"),
        }
    }
}
