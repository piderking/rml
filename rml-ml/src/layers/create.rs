use rml_data::tensor::shape::tensor::Tensor;
use rml_data::tensor::traits::dtype::dtype;
use rml_data::tensor::traits::tensor::TensorBound;

use crate::context::Context;

pub trait Layer<C: Context> {
    type Output: TensorBound;

    fn layer(&self, ctx: &C) -> Self::Output;
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
