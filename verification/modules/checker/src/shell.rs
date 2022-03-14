//! Compiler shell
//!
//! This module implements a command-line interface for the `Woodoo` language.
//!
//! ## Usage
//! Here's how to compile a source file in verbose mode, for instance:
//! ```sh
//! fumbo-checker -vvv examples/sudoku
//! ```

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "woodoo", about="Verifiable computing language")]
struct ShellOptions {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

fn main() {

    // Parse command-line parameters
    let options = ShellOptions::from_args();

    println!("{:#?}", options);
}