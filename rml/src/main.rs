use rml_data::tensor::tensor::Tensor;

pub fn main() {
    let mut  t = Tensor::new(vec![1, 2, 3, 4]);
    println!("{:?}", t);
    t.map(|f| *f+1);
    println!("{:?}", t);

    
}
