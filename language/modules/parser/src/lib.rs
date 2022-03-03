#![feature(log_syntax)]
//! Syntax analyzer
//!
//! This crate implements the syntax analyzer for `Woodoo` language.
//!
//! ## Note
//! Rust compiler requires that entry point functions of procedural macros be defined at the root of
//! the crate (e.g. in the `lib.rs` file), which explains why all macros are defined in this single
//! "entry" file.
//! What is the difference between `proc_macro` and `proc_macro2`? The latter is a drop-in replacement
//! for the former and is available outside of macros, which means that `proc_macro2` macros are testable.
//! Types and methods of `proc_macro2` are the same as those of `proc_macro`, and hence, can be used
//! interchangeably.
//! The usual pattern for writing a nontrivial procedural macro is to use [OuterTokenStream] (hence
//! an alias on `proc_macro::TokenStream` type) the entry point function, and use [InnerTokenStream]
//! (concretely, an alias on `proc_macro2::TokenStream` type) inside internal transformation routines
//! of procedural macros. This motivated by the fact that components of `proc_macro` crate can only
//! be used inside a procedural macro,

mod expressions;                        // expressions parsing routines
mod statements;                         // statements parsing routines
mod items;                              // items data structures
mod primitives;                         // common types
mod program;                            // verifiable computation model and algorithms

use quote::ToTokens;
use syn::parse_macro_input;
use crate::primitives::{InnerTokenStream, OuterTokenStream, ParseResult};
use crate::program::Program;

/// Program (or verifiable computation) entry point
///
/// Your verifiable computing adventure begins with this [procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html).
/// It is the entry point of a verifiable computation (hereafter coined as a [`Program`].
///
/// ## Example
/// ```no_compile
/// program! (name) {
///
///     witness {
///     }
///
///     inputs {
///     }
///
///     outputs {
///     }
/// }
/// ```
#[proc_macro]
pub fn program(input: InnerTokenStream) -> InnerTokenStream {
    let program = parse_macro_input!(input as Program);
    InnerTokenStream::from(program.to_token_stream())
}