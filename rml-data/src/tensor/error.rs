use std::{cell::BorrowMutError, fmt::Error};

use thiserror::Error;

use super::len::TensorSize;


#[derive(Error, Debug)]
pub enum TensorError {
    #[error("no value at expected index (index {index:?}")]
    InvalidIndex { index: usize },
    #[error("index to large (index {index:?} max index: {max_index:?}")]
    OverflowMaxIndex { index: usize, max_index: usize },
    #[error("data store disconnected")]
    BorrowMutError(#[from] BorrowMutError),
    #[error("error formatting")]
    DisplayWrite(#[from] Error),
    #[error("unknown data store error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum TensorSizeError {
    #[error("Sizes Unequals {t1} != {t2}")]
    InvalidComparesSize { t1: TensorSize, t2: TensorSize },
    #[error("Sizes Unequals {t1}")]
    InvalidSize { t1: TensorSize },
    #[error("error formatting")]
    DisplayWrite(#[from] Error),
    #[error("unknown data store error")]
    Unknown,
}
#[derive(Error, Debug)]
pub enum TensorOpperationError {
    #[error("Invalid Dot, {t1:?} and {t2:?}")]
    InvalidDotProductSize { t1: TensorSize, t2: TensorSize },
    #[error("unknown data store error")]
    Unknown,
}
