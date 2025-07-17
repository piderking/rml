use rml_data::tensor::shape::tensor::Tensor;

pub enum Unit <'a>{
    Bias(Tensor<'a, f32>),
    Weight(Tensor<'a, f32>)
}