//! Language playground application
//!
//! Nowadays also called REPL (for Read-Eval-Print-Loop) this module provides a playground to play with
//! so that to discover `Woodoo` through an interactive shell.

use std::error::Error;
use fumbo_playground::Playground;
use anyhow::{Context, Result};


/// Run playground standalone application
fn main() {

    let mut playground = Playground::new(())
        .name("fumbo-playground")
        .description("Interactive sandbox for experimenting with Woodoo language")
        .version("0.1.0");

    if let Err(error) = playground.execute() {
        eprintln!("Error: {:?}", error);
        std::process::exit(1);
    }
}