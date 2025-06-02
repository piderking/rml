pub mod linear;


mod tests {
    use rml_data::frame::tensor;
    use rml_data::tensorm;
    use rml_data::tensor::shape::tensor::Tensor;

    use crate::context::{LayerState, Model};

    use super::linear::Linear;
    use super::super::layers::{softmax::Softmax, temp::Temp};
    
    #[test]
    fn test() -> () {
        
        let mut t = Linear::new(vec![LayerState::Layer(Temp::new(1.0)),  LayerState::Layer(Softmax::new())]);

        let fin = t.model(vec![tensorm![1.0, 0.1, 1.0]]);

        println!("{:}", fin)
    }
}