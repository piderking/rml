//pub trait Context {}

use rml_data::tensor::shape::tensor::Tensor;

use crate::layers::{self, create::Layer};

pub trait ContextFlag {
    fn next(&mut self) -> ();
}
pub struct Input {}

pub enum ContextState<'a> {
    Input(Tensor<'a, f32>),
    Layer(Box<dyn Layer<'a, f32, Output = Tensor<'a, f32>>>),
    Output(Tensor<'a, f32>),
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

impl<'a> ContextFlag for ContextStruct<'a> {
    fn next(&mut self) -> () {
        match self.layers.pop() {
            Some(prev) => match self.layers.get_mut(0) {
                Some(layer) => match (prev, layer) {
                    (ContextState::Input(n), ContextState::Layer(a)) => todo!(),
                    (ContextState::Input(n), ContextState::Output(a)) => todo!(),
                    (ContextState::Input(n), _) => panic!("Only 1 Input!"),
                    (ContextState::Layer(n), ContextState::Layer(a)) => todo!(),
                    (ContextState::Layer(n), ContextState::Output(a)) => todo!(),
                    (_, ContextState::Input(ns)) => panic!("Input must only be first!"),
                    (ContextState::Output(n), _) => panic!("Out must be last"),
                },
                None => (),
            },
            None => (),
        };
    }
}
