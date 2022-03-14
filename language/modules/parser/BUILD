# Crate dependencies
load("@crate_index//:defs.bzl", "all_crate_deps")

# Rust compilation rules
load("@rules_rust//rust:defs.bzl", "rust_proc_macro", "rust_test", "rust_doc")

rust_proc_macro(
    name = "woodoo-parser",
    version = "0.1.0",
    edition = "2021",
    visibility = ["//visibility:public"],

    srcs = [
        "src/lib.rs",
        "src/expressions/mod.rs",
        "src/expressions/conditionals.rs",
        "src/expressions/loops.rs",
        "src/items/mod.rs",
        "src/items/datatype.rs",
        "src/items/inputs.rs",
        "src/primitives/mod.rs",
        "src/primitives/keywords.rs",
        "src/primitives/traits.rs",
        "src/primitives/types.rs",
        "src/program/mod.rs",
        "src/statements/mod.rs",
        "src/statements/declarations.rs",
        "src/statements/evaluations.rs",
    ],

    deps = all_crate_deps(),
#    deps = crate_deps([
#        "serde",
#        "syn",
#        "quote",
#        "proc_macro2"
#    ]),
)
