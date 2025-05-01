use rml_data::tensor::tensor::Tensor;
use rml_data::tensorm;

pub fn main() {
    //let mut d: Tensor<i32> = Tensor::new(vec![1, 2, 3]);

    //let p = tensorm![tensorm![1, 1], tensorm![1, 1], tensorm![1, 1]];

    let d = tensorm![tensorm![tensorm![1]]];
    println!("{:}", d.size())
}
