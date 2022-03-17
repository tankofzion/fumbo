//! Interactive playground module
//!
//! An interactive playground (or playground) allowing to land and play with the `Woodoo` language.
//! The playground can be executed as a standalone terminal application or in a browser, using
//! WebAssembly.
//!
//! ## How to Use
//!
//! ```
//! use std::collections::HashMap;
//! use fumbo::playground::{Command, Parameter, Result, Value};
//!
//! // Write "Hello"
//! fn hello<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
//!     Ok(Some(format!("Hello, {}", args["who"])))
//! }
//!
//! fn main() {
//!    let mut playground = Playground::new(())
//!         .name(crate_name!())
//!         .version(crate_version!())
//!         .description(crate_description!())
//!         .command(
//!              Command::new("hello", hello)
//!                  .parameter(Parameter::new("who").set_required(true)?)?
//!                  .help("Greetings!"),
//!     );
//!
//!     if let Err(error) = playground.execute() {
//!         eprintln!("Error: {:?}", error);
//!         std::process::exit(1);
//!     }
//! }
//! ```

mod command;
mod error;
mod playground;

// Re-export main components in crate namespace
pub use command::Command;
pub use playground::Playground;