//! Program (or verifiable computation) entity parsing routines

use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use crate::primitives::{OuterTokenStream, ParseResult};
use crate::items::ItemInputs;

/// Program (or verifiable computation) parse tree structure.
///
/// This data structure (or model or parse tree) is the root of a verifiable computation definition.
/// It is used, for instance, for generating final arithmetic circuit representations.
///
/// ## Syntax definition
/// ```text
/// Program ::=
///       (Expression)?               // list of zero or more expressions
///     | (Statement)+                // list of one or more statements
/// ```
pub struct Program {
    pub index: u16,
    pub inputs: ItemInputs,
}

impl Program {

    /// Generate an outer token stream from program's parse tree structure
    ///
    /// This method generate out source code from the inner program's parse tree structure.
    pub fn expand() -> OuterTokenStream {
        quote!()
    }
}

impl Parse for Program {

    fn parse(input: ParseStream) -> ParseResult<Self> {
        log_syntax!("Enter Program::parse() method");

        Ok(
            Program {
                index: 0,
                inputs: input.parse()?
            }
        )
    }
}

// Cast the given program model to an outer token stream type
impl From<&Program> for OuterTokenStream {
    fn from(_program: &Program) -> Self {
        Program::expand()
    }
}

// Convert program model into tokens of source code
//
// This function turns Rust syntax tree data structures into tokens of
impl ToTokens for Program {
    fn to_tokens(&self, tokens: &mut OuterTokenStream) {
        tokens.extend::<OuterTokenStream>(self.into());
    }
}
