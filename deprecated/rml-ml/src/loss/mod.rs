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


/*
function [W,b,M,V] = Adam(W, b, dW, db, alpha, M, V, iT)
    beta1   = 0.9;
    beta2   = 0.999;
    epsilon = 1e-8;

    params = [W;b];
    grads  = [dW;db];

    M  = beta1*M + (1-beta1)*grads;
    V  = beta2*V + (1-beta2)*grads.^2;

    M2 = M / (1-beta1^iT);
    V2 = V / (1-beta2^iT);

    alpha   = alpha*sqrt(1-beta2^iT)/(1-beta1^iT);

    params  = params - alpha*M2 ./ (sqrt(V2)+epsilon);

    W = params(1:end-1,:);
    b = params(end,:);
end

*/