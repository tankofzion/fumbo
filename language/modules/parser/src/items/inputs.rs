//! Program inputs item
//!
//! The [Program] inputs block declare input parameters a verifiable computation takes.

use syn::{braced, Field, Token, token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use crate::primitives::{ParseResult, keywords};

/// Program's inputs block parse tree
pub struct ItemInputs {
    keyword: keywords::inputs,
    brace: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for ItemInputs {

    fn parse(input: ParseStream) -> ParseResult<Self> {
        log_syntax!("Parse 'ItemInputs' item");

        let content;
        Ok(
            ItemInputs {
                keyword: input.parse::<keywords::inputs>()?,
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
    use crate::items::ItemInputs;
    use crate::primitives::OuterTokenStream;

    #[test]
    fn can_parse_inputs_item() {
        let input: OuterTokenStream = quote! {
            inputs {
                first_field: u32,
                second_field: u16,
            }
        };

        let _item: ItemInputs = syn::parse2( input ).unwrap();
    }
}