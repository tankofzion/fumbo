# Welcome to Fumbo Verifiable Computing Toolkit

The `Fumbo` (which means *mystery* in Swahili) project aims at building an efficient and robust verifiable (and/or blind)
computing platform on Rust, that is agnostic of both blockchain platforms (e.g. Ethereum, Polkadot, Mina, Solana, Cosmos
and more in the future) and proof systems (e.g. SNARK, STARK, IOP, IP, ...). `Fumbo` allows building collaborative proof
systems with multiple parties' secrets (Alex **OZDEMIR** and Dan **BONEH** 2021). Such auditable multi-party computing (MPC)
systems could be used for implementing decentralized applications such as, for instance, for credit scoring or insurance
and financial underwriting.

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
Take a look at the Act documentation for its prerequisites and [installation](https://github.com/nektos/act#installation)
procedure, as well as available virtual environments.

Once Act is installed on your host, building the project is as simple as entering the following command in a terminal so
that to run the CI `build` job defined in the [project's workflow](.github/workflows/ci.yml):

```shell
act --job build
```

This command can also be executed using [`cargo-make`](https://github.com/sagiegurari/cargo-make), as shown below:

```shell
cargo make --no-workspace build-with-act
```

## Project dependency graph

The [Blackjack](https://github.com/wildarch/blackjack) tool builds Cargo dependencies using Bazel. As it relies on standard
`Cargo.toml` configuration files, the project can be built using both Cargo and Bazel (see [WORKSPACE](./WORKSPACE) and 
[BUILD](./_BUILD.old) for more information on Blackjack's configuration). When you modify dependencies in a `Cargo.toml`
file, hence modifying the dependency graph, simply run again `bazelisk run //:blackjack` to generate new `cargo_dependencies.bzl`
files.

## Discovering continuous integration (CI) pipeline

This project's continuous integration (CI) pipeline relies on Bazel built-in support for incremental [remote caching](https://bazel.build/docs/remote-caching) 
in order to speed up compilation processes.

## Project Documentation

The `Fumbo` project provides a thorough documentation on its concepts and components. The following sources of information are available:

- A complete [book](./documents/book) entitled `Fumbo In-Depth`, that is the most extensive source of information for newbies and experts, covering verifiable computing fundamentals, architecture and components and more.
  This book relies on [Markdown](https://en.wikipedia.org/wiki/Markdown) format and can be rendered using [mdBook](https://github.com/rust-lang/mdBook) tool.
  See [Building and serving the Book](#building-and-serving-the-book) chapter later in this document to learn how to generate a HTML or a PDF representation of this book.

- **Source code** is thoroughly documented, with more technical details on how data structures and algorims are implemented.

- `README.md` files provide a documentation of a technical obedience, as a kind of "land-and-work" information. 

### Building and serving the Book

The [mdBook](https://github.com/rust-lang/mdBook) tool must be available on your host. If not, the easiest procedure to 
[install mdBook](https://rust-lang.github.io/mdBook/guide/installation.html) is by means of Cargo, as shown below:

```shell
cargo install mdbook
```
 
In order to generate and serve the HTML version of the `Fumbo In-Depth` book, you should proceed as follows:

```shell
cargo make --no-workspace serve-book
```

The book's HTML version is now accessible on [http://localhost:3000](https://localhost:3000).

You can also only build the book, as follows:

```shell
cargo make --no-workspace build-book
```

The resulting book is generated in [./target/documents/book](./target/documents/book) folder.

## References

Alex **OZDEMIR** and Dan **BONEH** (February 22, 2022) Experimenting wit h Collaborative zk-SNARKs: Zero-Knowledge Proofs for Distributed 
Secrets. Cryptology ePrint Archive: Report 2021/1530. https://eprint.iacr.org/2021/1530.pdf.