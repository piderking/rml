use rml_data::tensor::shape::tensor::Tensor;
use rml_data::tensor::traits::dtype::dtype;
use rml_data::tensor::traits::tensor::TensorBound;

use crate::context::ContextStruct;
use crate::contextual::Contextual;

pub trait Layer<'a, T: dtype> {
    type Output: TensorBound;

    fn fill(&mut self, t: Tensor<'a, T>) -> ();
    fn layer(&self, ctx: Contextual) -> Self::Output;
}
pub trait Empty {
    fn empty() -> Self;
}
macro_rules! create_op {
    ($name: ident) => {
        pub struct $name<'a, T: dtype> {
            pub tensor: Tensor<'a, T>,
        }
    };
}

create_op!(Deep);
create_op!(Softmax);
create_op!(Temp);
