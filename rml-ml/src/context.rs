//pub trait Context {}

use rml_data::tensor::{
    shape::tensor::Tensor,
    traits::{dtype::dtype, tensor::TensorBound},
};

use crate::layers::create::Layer;

pub trait Model<'a> {
    type Output: TensorBound;
    fn next(&mut self) -> Option<Self::Output>;
}

pub enum ContextState<'a> {
    Input(Tensor<'a, f32>),
    Layer(Box<dyn Layer<'a, f32, Output = Tensor<'a, f32>>>),
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

impl<'a> Model<'a> for ContextStruct<'a> {
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
