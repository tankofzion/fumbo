// Testing program declaration statement parsing macro.
//
// This user interaction (UI) test aims at testing parser's expressions and statements parsing macros.

use woodoo_parser::program;

// Sample computation declaration
program! {

    inputs {
        bitfields: u32,
        value: u16,
    }
}

fn main() {
}