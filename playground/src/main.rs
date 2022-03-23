//! Language playground application
//!
//! Nowadays also called REPL (for Read-Eval-Print-Loop) this module provides a playground to play with
//! so that to discover `Woodoo` through an interactive shell.

use std::env;
use fumbo_playground::Playground;
use fumbo_playground::Command;
use fumbo_playground::Context;

fn show_help(context: &mut Context) {
    context.playground.display_help();
}

fn show_version(context: &mut Context) {
    context.playground.display_version();
}

fn show_history(context: &mut Context) {
    context.playground.display_history();
}

/// Run playground standalone application
fn main() {

    // Create commands
    let version_command = Command::new("version")
        .description("Show playground version")
        .action(show_version);

    let help_command = Command::new("help")
        .description("Display general help message")
        .action(show_help);

    let history_command = Command::new("history")
        .description("List of executed commands")
        .action(show_history);

    // Create a playground instance
    let mut playground = Playground::new()
        .name("Fumbo Playground")
        .description("Interactive sandbox for experimenting with Woodoo language")
        .version("0.1.0")
        .command(version_command)
        .command(help_command)
        .command(history_command);

    // Collect command-line arguments into a vector of strings
    let arguments = env::args().collect::<Vec<String>>();

    if let Err(error) = playground.run(arguments.as_slice()) {
        eprintln!("Error: {:?}", error);
        std::process::exit(1);
    }
}
