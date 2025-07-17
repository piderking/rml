use std::marker::PhantomData;

use super::{tensor::TensorWrapper, tensorable::Tensorable};

pub enum TensorFold<T: Tensorable> {
    DEEP(Vec<Tensor<T>>, PhantomData<T>),
    SHALLOW(TensorWrapper<T>),
    EMPTY,
}

/*
Example: [0,0,0] -> Tensor<i32>::SHALLOW(TensorWrapper<i32> { Rc<Vec<[0,0,0]})
Example: [[0, 0], [0, 0]] -> Types ommited for clarity
Tensor::DEEP(vec![Tensor::SHALLOW(Rc<Vec<[0,0]>>), ensor::SHALLOW(Rc<Vec<[0,0]>>)])
*/
