//! This module implements the error object for jrpc-types crate.
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("invalid type: {0}")]
    InvalidType(String),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
}
