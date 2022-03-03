//! Expressions
//!
//! An expression produces a value. However it can also produce a side-effect.
//! An [Expression] returns a result, while a [Statement] expresses some action to be performed.
//! All control structures, such as, for instance [IfElse], can be used as expressions, and their
//! respective result assigned to a variable, as shown below:
//!
//! ```text
//! let result = if x > y then x else y
//! ```
//!
//! ## Abstract syntax
//!
//! ```text
//! Expression ::=
//!     IfExpression
//!   | WhileExpression
//!   | ForExpression
//! ```
//!

mod conditionals;                   // conditional expressions
mod loops;                          // loop expressions

use conditionals::*;
use loops::*;
use crate::InnerTokenStream;

/// Expression syntax rules
pub enum Expression {
    /// Conditional expression
    ///
    /// This expression is of the form `if expression {...} else { ... }`. Following the `else` branch
    /// is either a block or a nested `if` expression, like in `if expression {...} else if expression
    /// { ... }`.
    IfElse(IfConditionalExpression),

    /// If-let expression
//        IfLet(IfLetConditionalExpression),

    /// Guarded `let` expression
    ///
    /// This expression is of the form `let Some(x) = opt`.
//        Let(LetExpression),

    /// Infinite loop expression
    Loop(InfiniteLoopExpression),

    /// Iterator loop expression
    For(ForLoopExpression),

    /// Predicate loop expression
    While(WhileLoopExpression),

    /// Pattern-based predicate loop expression
//        WhileLet(WhileLetLoopExpression),

    /// Invalid expression
    Unknown(String),
}

impl Expression {

    // Parse an arbitrary expression from token stream
    pub fn parse(input: InnerTokenStream) -> InnerTokenStream {
        dbg!(input)
    }
}
