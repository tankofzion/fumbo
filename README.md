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

As we also rely on [cargo-make](https://github.com/sagiegurari/cargo-make) (rather than traditional obscure Makefiles) 
to build and test the project's components, the following command can be used to build the overall project's binaries and 
libraries, with Bazel and Bazelisk being the puppet masters behind the scenes:

```shell
cargo make build
```

The [cargo-make](https://github.com/sagiegurari/cargo-make) tasks are defined in [Makefile.tom] file. We let you take
a glance at it.

### Local build using Act

`Fumbo` uses GitHub Actions for building and orchestrating its continuous integration pipeline. Consequently, you can 
readily build the project locally using [Act](https://github.com/nektos/act), a Dockerized (i.e. containerized) GitHub 
Actions platform allowing to test actions locally, without the necessity to push the code to the GitHub repository. 
Take a look at the official Act documentation for the necessary prerequisites, its [installation](https://github.com/nektos/act#installation)
procedure, and available virtual environments.

Once Act is installed on your host, building the project is as simple as entering the following command in a terminal so
that to run the `build` job defined in the [project's workflow](.github/workflows/main.yml):

```shell
act --job build
```

This command can also be executed using [`cargo-make`](https://github.com/sagiegurari/cargo-make), as shown below:

```shell
cargo make --no-workspace build-with-act
```

## Project dependencies

The [Blackjack](https://github.com/wildarch/blackjack) tool builds Cargo dependencies using Bazel. As it relies on standard
`Cargo.toml` configuration files, the project can be built using both Cargo and Bazel (see [WORKSPACE](./WORKSPACE) and 
[BUILD](./_BUILD.old) for more information on Blackjack's configuration). When you modify dependencies in a `Cargo.toml`
file, hence modifying the dependency graph, simply run again `bazelisk run //:blackjack` to generate new `cargo_dependencies.bzl`
files.
