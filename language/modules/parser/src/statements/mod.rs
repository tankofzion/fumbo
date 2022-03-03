//! Statements definition module
//!
//! A statement contains and evaluates one or more [`Expression`]s. It is contained in a block, which
//! is in turn, contained in an outer [Expression] or function declaration.
//! `Woodoo` has two kinds of statements, namely *declaration statements* and *evaluation statements*.
//! The former (declaration) introduces new *names* in the scope of the block it is contained in and
//! the latter (evaluation) evaluates an [`Expression`] while ignoring its result.
//!
//! ## Syntax definition
//!
//! ```text
//! Statement ::=
//!     | DeclarationStatement                  (name introduction statement)
//!     | EvaluationStatement                   (expression evaluation statement)
//! ```

mod declarations;           // items and names declaration statement
mod evaluations;            // expression evaluation statement

use declarations::DeclarationStatement;
//use evaluations::EvaluationStatement;

/// Statement syntax definition
pub enum Statement {

    /// Statements introducing new names or items
    Declaration(DeclarationStatement),

    // /// Statements evaluating an expression
    // Evaluation(EvaluationStatement),
}
