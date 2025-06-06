use rml_data::tensor::shape::tensor::Tensor;



pub trait Empty <'a>{
fn empty(ten: &Tensor<'a, f32>) -> Box<Self>;
}