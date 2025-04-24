use rml_data::tensor::tensor::Tensor;

pub fn main() {
    let t = Tensor::new(vec![1, 2, 3, 4]);
    let i: Tensor<i32> = t.iter().into_iter().map(|d| d + 1).collect();
}
