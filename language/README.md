# Woodoo Language Component

The aim is to provide blockchain engineers a modern and robust stack for building robust and efficient verifiable computing 
solutions without the hassle of writing boilerplate or being stuck to a particular blockchain. At the time of this writing, 
`Fumbo` only supports Polkadot/Substrate as a target blockchain. However, in the near future, it will target  platforms 
(e.g. Ethereum, Mina, Solana, Cosmos).

At the heart of the `Fumbo` project lies `Woodoo`, a Rust-based [embedded domain specific language (eDSL)](https://wiki.haskell.org/Embedded_domain_specific_language) 
for efficient, scalable and robust verifiable computing solutions engineering.
`Woodoo` leverages on Rust meta-programming features (i.e. procedural macros) for building features of great help for engineering
robust and efficient verifiable computing solutions using Rust, such as for instance, extending the Rust language, with 
dependently typed programming or type-level and type-driven development tools. Types are not only used for checking, but also
for guiding (with type-driven concepts) engineers

## Aims and Objectives

Building verifiable computation solutions is a complex task, mainly due to the fact  that a computation problem must first 
be translated into a constraint-based representation, such as, for instance, R1CS or QAP.

Unlike low-level circuits engineering tools, `Woodoo` is a mid-level abstraction making development
easier. For instance, in the low-level [libsnark](https://github.com/scipr-lab/libsnark) library, a programmer represents 
the verifiable program as [gadgets](https://github.com/scipr-lab/libsnark#gadget-libraries) that are connected together,
each defining a set of constraints and how to set the value of its output variables. In `Woodoo`, an intuitive and efficient
interface is provided to the developer, abstracting over the cryptographic backend and
likely generating faster circuits than if they were hand coded.

## Syntax definition

The syntax of the `Woodoo` language is defined using abstract notation, each
term being implemented as a Rust [procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html).

## Formal verification

There are currently two methods for formal verification. The traditional form uses a branch of mathematics called 
type theory, such as, for instance, the Martin-Löf intuitionist approach. If the code works, the verification will 
generate a mathematical proof of correctness. This method is therefore rigorous but, for various reasons, impractical to
use. A second method is **type systems for refinements**, that can be developed in common programming languages, even
using their respective meta-programming features (like we do in this project with Rust procedural
macros). This is a practical form of verification and can be integrated in the software, but it lacks the strong mathematical 
foundation of type theory.

## Type checking and inference

## Proof system backend

## References

