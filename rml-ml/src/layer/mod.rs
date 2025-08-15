use rml_data::tensor::{DTYPE, Tensor};

pub trait Layer<T: DTYPE, U: Tensor<T>> {
    fn process(self) -> U;
}
