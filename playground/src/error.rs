//! Error codes and types

use thiserror::Error;

/// Result type
pub type PlaygroundResult<T> = std::result::Result<T, PlaygroundError>;

/// Error codes enumeration
///
/// This enumeration defines error variants for every possible error that this playground library
/// might encounter.
#[derive(Error, Debug, PartialEq)]
pub enum PlaygroundError {
    #[error("Too many arguments are passed to a command")]
    TooManyArguments(String, usize),

    /// Command not found
    #[error("The requested command is not implemented")]
    UnknownCommand(String),
}

