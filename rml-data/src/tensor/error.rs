use std::cell::BorrowMutError;

use thiserror::Error;

use super::{len::TensorSize, tensor::{Tensor, TensorBound}, tensortype::TensorType};

#[derive(Error, Debug)]
pub enum TensorError {
    #[error("no value at expected index (index {index:?}")]
    InvalidIndex { index: usize },
    #[error("index to large (index {index:?} max index: {max_index:?}")]
    OverflowMaxIndex { index: usize, max_index: usize },
    #[error("data store disconnected")]
    BorrowMutError(#[from] BorrowMutError),
    #[error("unknown data store error")]
    Unknown,
}
#[derive(Error, Debug)]
pub enum TensorOpperationError {
    #[error("Invalid Dot, {t1:?} and {t2:?}")]
    InvalidDotProductSize { t1: TensorSize, t2: TensorSize  },
    #[error("unknown data store error")]
    Unknown,
}
