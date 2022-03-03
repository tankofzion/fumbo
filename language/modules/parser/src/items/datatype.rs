//! Type item parsing routine
//!
//! This module defines the parse tree structure and parsing algorithm for the [Datatype] item, that
//! is a dependent type in the vein of Martin-Löf's intuitionistic theory of types.
//!
//! In dependently programming languages, terms correspond to both programs and proofs, which means
//! that a type can equally be considered as a specification or a proposition.
//!
//!
//! ## Example
//! ```text
//! data Stack Nat -> Set {
//!     [] : Stack 0,
//!     _#_: Stack n -> Nat -> Stack(suc n)
//! }
//! ```
//!
//! ## Type operator
//!
//! A type operator is like a function acting at the type level. It is an infix operator inserted in
//! types providing a more natural syntax to write types. Let's have an example on how to use them:
//!
//! ```text
//!
//! ```
//!
//! A declaration statement of the form `[] # 1 # 2 # 3` will then result in a `Stack 3` data type.

use syn::{braced, Field, Ident, Token, token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use crate::primitives::ParseResult;

/// Parse tree structure for dependent data type item
pub struct ItemDatatype {
    keyword: Token![type],
    identifier: Ident,
    brace: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for ItemDatatype {

    fn parse(input: ParseStream) -> ParseResult<Self> {
        log_syntax!("Parse 'DataType' item");

        let content;
        Ok(
            ItemDatatype {
                keyword: input.parse()?,
                identifier: input.parse()?,
                brace: braced!(content in input),
                fields: content.parse_terminated(Field::parse_named)?
            }
        )
    }
}

// Unit tests
#[cfg(test)]
mod tests {

    use syn;
    use quote::quote;
    use crate::items::datatype::ItemDatatype;
    use crate::primitives::OuterTokenStream;

    #[test]
    fn can_parse_simple_datatype_item() {
        let input: OuterTokenStream = quote! {
            type Stack {
                field1: u32,
                field2: u32,
            }
        };

        let _item: ItemDatatype = syn::parse2( input ).unwrap();
    }
}