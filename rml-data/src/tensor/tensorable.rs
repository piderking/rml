use std::any::Any;
use std::io::Error;

use super::tensor::TensorWrapper;

pub trait Tensorable {}

impl Tensorable for i32 {}
impl Tensorable for f32 {}
impl Tensorable for i16 {}
impl<T: Tensorable> Tensorable for TensorWrapper<T> {}
