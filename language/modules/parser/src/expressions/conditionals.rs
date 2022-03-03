//! Conditional expressions
//!
//!

use crate::primitives::Attributes;

/// If-else conditional expression parse tree structure
///
/// This conditional expression is a branching path in program control. The syntax of such an `If`
/// expression is a conditional operand followed by a consequence block, any (optional) number of `else if`
/// conditions and blocks and an (optional) trailing `else` block, as show in the syntax below.
///
/// ## Syntax
/// ```text
/// IfConditionalExpression ::=
///     `if` [Expression] [BlockExpression]
///         (`else` ([BlockExpression] | IfConditionalExpression | IfLetConditionalExpression))?
/// ```
pub struct IfConditionalExpression {
    /// Attributes (or meta-data) tied to the field
    pub attributes: Attributes,
    pub index: u8,
}


/// If-let conditional expression parse tree structure
pub struct IfLetConditionalExpression {
    /// Attributes (or meta-data) tied to the field
    pub attributes: Attributes,
    pub index: u8,
}