//! Error codes and types

use thiserror::Error;

use crate::Playground;

/// Result type
pub type PlaygroundResult<ReturnType> = std::result::Result<ReturnType, PlaygroundError>;

/// Error handling function
pub type ErrorHandler<Context, ErrorType> = fn(playground: &Playground<Context>, error: ErrorType) -> PlaygroundResult<()>;

/// Default error handling function
pub fn default_error_handler<Context>(
    _playground: &Playground<Context>,
    error: PlaygroundError) -> PlaygroundResult<()> {
    eprintln!("Error: ${:?}", error);
    Ok(())
}

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

