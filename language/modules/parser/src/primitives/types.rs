//! Common types definitions
//!
//! This module defines generic types and data structures used to implement the Woodoo language
//! syntactic analyzer (or parser).

/// Inner token stream strictly used inside the body of a procedural macro.
///
/// Unlike [OuterTokenStream] type, this token stream is exclusively used inside the body (or entry
/// point function) of aprocedural macro. It cannot be used, for instance, in other "outer" modules
/// like in 'lib.rs' or 'main.rs' files. As a rule of thumb, [OuterTokenStream] must be used for
/// implementing the functionalities of a procedural macro, so that they can be be tested and used
/// in non-macro code.
pub type InnerTokenStream = proc_macro::TokenStream;

/// Outer token stream used for outside the procedural macro body.
///
/// The difference between `proc_macro::TokenStream` (hereafter aliased to [InnerTokenStream]) and
/// `proc_macro2::TokenStream` (hereafter aliased to [OuterTokenStream]) types is that `proc_macro`
/// types are specific to procedural macros and cannot ever exist in code outside of a procedural
/// macro's body, while `proc_macro2` types may exist anywhere including in tests and non-macro code
/// like `main.rs`, `lib.rs` or `build.rs`. This explains why procedural macros are mainly implemented
/// using `proc_macro2` types, so that the libraries are unit testable and accessible in non-macro
/// contexts.
///
/// ## Example
///
/// ```no_compile
/// use primitives::{OuterTokenStream, InnerTokenStream};
///
/// #[proc_macro]
/// pub fn program(input: InnerTokenStream) -> InnerTokenStream {
///     // cast input token stream as an outer (or proc_macro2) token stream
///     // type before calling "outer" method 'Program::parse'
///     let output = Program::parse(OuterTokenStream::from(input));
///     InnerTokenStream::from(output)
/// }
/// ```
pub type OuterTokenStream = proc_macro2::TokenStream;

pub type Attributes = Vec<syn::Attribute>;

/// Parsing result option
pub type ParseResult<T> = syn::Result<T>;

/// Parsing error
pub type ParseError = syn::Error;
