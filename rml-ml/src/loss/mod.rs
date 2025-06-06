use rml_data::tensor::{shape::tensor::Tensor, traits::tensor::TensorBound};

pub enum LossFunction {
    MeanSquaredError(),
    MeanAbsoluteError(),
    CrossEntropy(),
}

impl <'a> LossFunction {
    pub fn loss (&self, prediction: Tensor<'a, f32>, actual: Tensor<'a, f32>) -> f32{
        match self {
            LossFunction::MeanSquaredError() => {
                (1.0 / prediction.len() as f32) * Tensor::from(prediction.into_iter().zip(actual.into_iter()).map(|(p, a)| a - p).collect()).sum()
            },
            _ => panic!("Loss Function Undefined")
        }
    }
}