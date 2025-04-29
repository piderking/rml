use rml_data::tensor::tensor::Tensor;

pub fn main() {
    let mut d: Tensor<i32> = Tensor::new(vec![1, 2, 3]);

    let a = Tensor::new(vec![
        Tensor::new(vec![Tensor::new(vec![
            Tensor::new(vec![1]),
            Tensor::new(vec![2]),
        ])]),
        Tensor::new(vec![Tensor::new(vec![
            Tensor::new(vec![3]),
            Tensor::new(vec![4]),
        ])]),
    ]);
    println!("{:}", a);
    println!("{:?}", a.size())
}
