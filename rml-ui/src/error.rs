use std::io;

use thiserror::Error;

#[derive(Error, Debug, Default)]
pub enum TError {
    #[error("data store disconnected")]
    Io(#[from] io::Error),
    
    #[default]
    #[error("unknown data store error")]
    Unknown,
}


