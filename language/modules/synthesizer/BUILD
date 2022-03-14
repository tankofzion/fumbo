# Rust compilation rules
load("@rules_rust//rust:defs.bzl", "rust_library")

package(default_visibility = ["//visibility:public"])

# Build synthesizer library
rust_library(
    name = "woodoo-synthesizer",
    edition = "2021",
    srcs = [
        "src/lib.rs"
    ]
)

# Execute unit tests
rust_test(
    name = "woodoo-synthesizer-test",
    deps = [":woodoo-synthesizer"]
)
