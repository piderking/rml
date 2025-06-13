pub mod linear;


mod tests {
    use rml_data::frame::tensor;
    use rml_data::tensor::traits::tensor::TensorBound;
    use rml_data::tensorm;
    use rml_data::tensor::shape::tensor::Tensor;

    use crate::context::{LayerState, Model};
    use crate::empty::Empty;
    use crate::layers::shallow::Shallow;

    use super::linear::Linear;
    use super::super::layers::{softmax::Softmax, temp::Temp};
    
    #[test]
    fn test() -> () {
        
        let mut t = Linear::new(vec![LayerState::Layer(Temp::new(1.0)),  LayerState::Layer(Softmax::new())]);

        let fin = t.model(vec![tensorm![1.0, 0.1, 1.0]]);

        println!("{:}", fin)
    }
    #[test]
    fn empty_linear_test() -> () {
        
        let input = tensorm!(1.0, 1.0, 1.0);
        let mut t = Linear::new(vec![LayerState::Layer(Temp::empty(input.len())),  LayerState::Layer(Softmax::empty(0))]);

        let fin = t.model(vec![tensorm![1.0, 0.1, 1.0]]);

        println!("{:}", fin)
    }
    #[test]
    fn empty_shallow_test() -> () {
        let deep = Shallow::new(tensorm!(0.25, 0.9, 0.1), tensorm!(0.0, 0.0, 0.0));
        let input = tensorm!(0.0, 1.0, 0.0);
        let mut t = Linear::new(vec![LayerState::Layer(Temp::empty(3)),  LayerState::Layer(deep),  LayerState::Layer(Softmax::empty(0))]);

        let fin = t.model(vec![tensorm![0.0, 1.1, 1.0]]);

        println!("{:}", fin)
    }
}