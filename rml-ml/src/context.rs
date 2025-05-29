//pub trait Context {}

use rml_data::tensor::{
    shape::tensor::Tensor,
    traits::{dtype::dtype, tensor::TensorBound},
};

use crate::{
    contextual::Contextual,
    layers::{self, create::Layer},
};

pub trait ContextFlag<'a> {
    type Output: TensorBound;
    fn next(&mut self) -> Option<Self::Output>;
}
pub struct Input {}

pub enum ContextState<'a> {
    Input(Tensor<'a, f32>),
    Layer(Box<dyn Layer<'a, f32, Output = Tensor<'a, f32>>>),
    Output(),
}
impl<'a> ContextState<'a> {
    pub fn context(&self) -> Contextual {
        Contextual::new(1.0)
    }
}

pub struct ContextStruct<'a> {
    position: usize,
    layers: Vec<ContextState<'a>>,
}

impl<'a> ContextStruct<'a> {
    pub fn new() -> Self {
        ContextStruct {
            position: 0,
            layers: vec![],
        }
    }
    pub fn fill(&mut self, v: Vec<ContextState<'a>>) -> () {
        self.layers = v;
    }
    pub fn prev(&self) -> Tensor<'a, f32> {
        todo!()
    }
}

impl<'a> ContextFlag<'a> for ContextStruct<'a> {
    type Output = Tensor<'a, f32>;
    fn next(&mut self) -> Option<Self::Output> {
        match self.layers.pop() {
            Some(prev) => match self.layers.get_mut(0) {
                Some(layer) => match (prev, layer) {
                    (ContextState::Input(n), ContextState::Layer(a)) => {
                        a.fill(n);
                        a.layer(prev.context());
                        Option::None
                    }
                    (ContextState::Input(n), ContextState::Output()) => {
                        // TODO --> Return Here
                        Option::Some(n)
                    }
                    (ContextState::Input(n), _) => panic!("Only 1 Input!"),
                    (ContextState::Layer(n1), ContextState::Layer(n2)) => {
                        n2.fill(n1.layer(prev.context()));
                        Option::None
                    }
                    (ContextState::Layer(n), ContextState::Output()) => {
                        Option::Some(n.layer(prev.context()))
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

#[cfg(test)]
pub mod test {
    use rml_data::tensorm;

    use super::{ContextState, ContextStruct};
    use crate::layers::create::{Empty, Temp};
    use rml_data::tensor::shape::tensor::Tensor;

    #[test]
    pub fn context() -> () {
        let mut t = ContextStruct::new();

        t.fill(vec![
            ContextState::Input(Tensor::empty()),
            ContextState::Layer(Box::new(Temp::empty())),
        ]);
    }
}
