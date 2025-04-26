use rml_data::tensor::tensor::Tensor;

pub fn main() {
    let mut d: Tensor<i32> = Tensor::new(vec![1, 2, 3]);
    let a = Tensor::new(vec![1, 2]);
    println!("{:}", d.combine(a))
}
