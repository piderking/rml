use std::cell::BorrowMutError;

use thiserror::Error;

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
