# Welcome to Fumbo Verifiable Computing Toolkit

The `Fumbo` (which means *mystery* in Swahili) project aims at building an efficient and robust verifiable (and/or blind)
computing platform on Rust, that is agnostic of both blockchain platforms (e.g. Ethereum, Polkadot, Mina, Solana, Cosmos
and more in the future) and proof systems (e.g. SNARK, STARK, IOP, IP, ...).

## Project components

This table briefly describes the major components of the `Fumbo` project.

| Component                | Description                                                                  |
|--------------------------|------------------------------------------------------------------------------|
| [`language`](./language) | Implementation of the `Woodoo` language.                                     |
| [`platform`](./platform) | Integration with blockchain platforms (e.g. Polkadot, Ethereum, Solana, ...) |

## Project building

The `Fumbo` project adopts the trunk-based development practice, being organized as a mono-repository with few active
branches as possible (hopefully not exceeding three) and merging branches to trunk on a daily basis without any code
freeze periods.

For doing so, continuous integration tasks (i.e. compilation, packaging, delivery, ...) relies on [GitHub Actions](https://docs.github.com/en/actions) 
and [Bazel](https://bazel.build/). So that to facilitate the handling of Bazel, the [Bazelisk](https://github.com/bazelbuild/bazelisk)
tool is used. 

### Dependencies management

The [Blackjack](https://github.com/wildarch/blackjack) tool builds Cargo dependencies using Bazel. As it relies on standard
`Cargo.toml` configuration files, the project can be built using both Cargo and Bazel (see [WORKSPACE](./WORKSPACE) and 
[BUILD.bazel](./BUILD) for more information on Blackjack's configuration). When you modify dependencies in a `Cargo.toml`
file, hence modifying the dependency graph, simply run again `bazelisk run //:blackjack` to generate new `cargo_dependencies.bzl`
files.

### Local build

`Fumbo` uses GitHub actions for building and delivering packages and ensuring code continuous integration. Consequently,
you can readily build the project locally, using the [Act](https://github.com/nektos/act) tool for executing GitHub Actions 
scripts in a containerized environment on your host, using Docker behind the scene. Let's see how folks!

First, take a look at the official Act documentation for the necessary prerequisites and the [Act installation](https://github.com/nektos/act#installation)
procedure for your operating system.
