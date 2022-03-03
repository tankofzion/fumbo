//! Operator expressions
//!
//! ## Operator expressions
//! ```text
//! OperatorExpression :
//!      BorrowExpression
//!    | DereferenceExpression
//!    | ErrorPropagationExpression
//!    | NegationExpression
//!    | ArithmeticOrLogicalExpression
//!    | ComparisonExpression
//!    | LazyBooleanExpression
//!    | TypeCastExpression
//!    | AssignmentExpression
//!    | CompoundAssignmentExpression
//! ```
//!
//! ## Syntax definition
//! ```text
//! ComparisonExpression :
//!      Expression == Expression
//!    | Expression != Expression
//!    | Expression > Expression
//!    | Expression < Expression
//!    | Expression >= Expression
//!    | Expression <= Expression
//! ```
