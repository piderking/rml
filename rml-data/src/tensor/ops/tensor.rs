use core::panic;
use std::ops::{Add, Div, Mul, Sub};

use crate::tensor::traits::{dtype::dtype, tensor::TensorBound};

use super::dtype::dtypeops;

macro_rules! tensor_ops {
    () => {};
}
