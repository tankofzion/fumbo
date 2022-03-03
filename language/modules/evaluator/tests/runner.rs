//! Semantics evaluator library testing runner.
//!
//! The herein tests allows to test errors detected by macros or those detected by the Rust compiler
//! in the resulting expanded code of a macro. These tests are commonly called user interaction (UI)
//! tests, as they allow to test user's interaction with functionalities implemented in a library.
#[test]
fn tests() {
    let t = trybuild::TestCases::new();

    // execute successful test cases
    t.pass("tests/cases/pass/test_*.rs");

    // execute failing test cases
    t.compile_fail("tests/cases/fail/test_*.rs");
}