use rml_data::tensor::tensor::Tensor;

pub fn main() {
    let t = Tensor::new(vec![1, 2, 3, 4]);
    t.data.borrow_mut();
}
