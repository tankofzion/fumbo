use crate::InnerTokenStream;

/// Code generation interface definition.
///
/// When implementing this code generation interface an item ensures that it can generates code from
/// its intermediate representation (or parse tree).
pub trait Generate {

    /// Generates code for entity implementing this trait.
    fn generate(&self) -> InnerTokenStream;
}