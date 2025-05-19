use std::f32::consts::E;

pub fn sigmoid(v: f32) -> f32 {
    (1.0) / (1.0 + E.powf(-1.0 * v))
}

pub fn relu(v: f32) -> f32 {
    if v <= 0.0 {
        return 0.0;
    }
    v
}
pub fn tanh(v: f32) -> f32 {
    (E.powf(v) - E.powf(-1.0 * v)) / (E.powf(v) + E.powf(-1.0 * v))
}
