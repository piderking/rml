use rml_data::tensor::shape::tensor::Tensor;



pub trait Empty <'a>{
fn empty(ten: usize) -> Box<Self>;
}