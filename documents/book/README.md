# Fumbo in Depth

This book is an in-depth description of Funbo project's components, including the [language](../../language)
and the [platform](../../platform), for instance.

## Requirements

This book is built using [mdBook](https://github.com/rust-lang/mdBook) tool. The latter can be installed using Cargo as 
follows:

```shell
cargo install mdbook
```

You can upgrade [mdBook](https://github.com/rust-lang/mdBook) via Cargo as follows:

```shell
cargo install --force mdbook
```

If the [mdBook](https://github.com/rust-lang/mdBook) installation went well, you should be able to display its version
using this command:

```shell
mdbook --version
```

## Building the Book

Building the book is as simple as this, folks:

```shell
mdbook --version
```

## Serving the Book

The book can be rendered as HTML pages and served by a local webserver in a browser using this command:

```shell
mdbook serve
```
