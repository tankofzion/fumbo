# Woodoo language components

The `Woodoo` language relies on several Rust modules for its implementation, as a *surface* idiom (also called domain-
specific language - DSL) relying on Rust language's compiler infrastructure to implement its features, `Woodoo` stands on
the  shoulders of a giant while adding some interesting new features like [dependent types](https://en.wikipedia.org/wiki/Dependent_type#:~:text=In%20computer%20science%20and%20logic,%22%20and%20%22there%20exists%22.).

| Module                         | Description                                                                        |
|--------------------------------|------------------------------------------------------------------------------------|
| [`parser`](./parser/README.md) | Syntactic analyzer, implemented using Pratt parsing strategy.                      |
| [`typer`](./typer/README.md)   | Type checking routines. `Woodoo` type checker (or typer) supports dependent types. |
