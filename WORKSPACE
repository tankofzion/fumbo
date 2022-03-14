workspace( name = "fumbo" )

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

# ----------------------------------------------------------------------------------------------------------------------
# Bazel tools
# ----------------------------------------------------------------------------------------------------------------------

# Skylib provides Starlark functions for manipulating collections, file paths, and various other data types.
git_repository(
    name          = "bazel_skylib",
    remote        = "https://github.com/bazelbuild/bazel-skylib.git",
    commit        = "5bffd04256c07a935c884339b433ca9e1d5c9a8e",                 # February 24th, 2022
    shallow_since = "1645664775 -0500",                                         # Allow for more shallow clone (saving bandwith and wall-clock tome)
)

# Bazel BUILD files formatter
git_repository(
    name          = "com_github_bazelbuild_buildtools",
    remote        = "https://github.com/bazelbuild/buildtools",
    commit        = "5d33f3736e81051942209b6239c615a9b66bf550",                 # February 24th, 2022
)

# ----------------------------------------------------------------------------------------------------------------------
# Rust tools
# ----------------------------------------------------------------------------------------------------------------------

# Rust toolchain configuration
git_repository(
    name = "rules_rust",
    commit = "1b1f21ac5c1b188bf658cb408afdb58f1668baef",                        # March 8th, 2022
    remote = "https://github.com/bazelbuild/rules_rust.git",
    shallow_since = "1646750620 -0800",
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")
rules_rust_dependencies()
rust_register_toolchains(edition="2021", version="nightly", iso_date = "2022-03-08")

# ----------------------------------------------------------------------------------------------------------------------
# Cargo crates dependencies resolution
# ----------------------------------------------------------------------------------------------------------------------

# Crate universe rules (used to manage crates dependencies)
load("@rules_rust//crate_universe:crates.bzl", "crate_deps_repository")
crate_deps_repository()

load("@rules_rust//crate_universe:crates_deps.bzl", "crate_repositories")
crate_repositories()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")
crates_repository(
    name = "crate_index",
    lockfile_kind = "bazel",
    lockfile = "//:Cargo.bazel.lock",
    manifests = [
        "//:Cargo.toml",
#        "//language/modules/parser:Cargo.toml",
#        "//language/modules/compiler:Cargo.toml",
#        "//verification/modules/checker:Cargo.toml",
    ],
)

load("@crate_index//:defs.bzl", crate_index_repositories = "crate_repositories")
crate_index_repositories()

# ----------------------------------------------------------------------------------------------------------------------
# Packaging tools
# ----------------------------------------------------------------------------------------------------------------------

# Rust packaging (zip, tar, deb, rpm) tool configuration
git_repository(
    name = "rules_pkg",
    commit = "61018b85819d57feb56886316e76e8ed8a4ce378",                        # March 6th, 2021
    remote = "https://github.com/bazelbuild/rules_pkg.git",
)
