//! Error and Result definitions

use std::fmt::Display;
use thiserror::Error;

/// The error type of this crate
#[derive(Error, Debug)]
pub enum VfsError {
    /// A generic IO error
    #[error("IO error")]
    IoError(#[from] std::io::Error),

    /// The file or directory at the given path could not be found
    #[error("The file or directory `{path}` could not be found")]
    FileNotFound { path: String },

    /// The given path is invalid, e.g. because contains '.' or '..'
    #[error("The path `{path}` is invalid")]
    InvalidPath { path: String },

    /// Generic error variant
    #[error("other FileSystem error: {message}")]
    Other { message: String },

    /// Generic error context, used for adding context to an error (like a path)
    #[error("{context}, cause: {cause}")]
    WithContext {
        context: String,
        #[source]
        cause: Box<VfsError>,
    },
}

/// The result type of this crate
pub type VfsResult<T> = std::result::Result<T, VfsError>;

pub(crate) trait VfsResultExt<T> {
    fn with_context<C, F>(self, f: F) -> VfsResult<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T> VfsResultExt<T> for VfsResult<T> {
    fn with_context<C, F>(self, context: F) -> VfsResult<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|error| VfsError::WithContext {
            context: context().to_string(),
            cause: Box::new(error),
        })
    }
}