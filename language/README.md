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

Building verifiable computation solutions is a complex task. A computation problem must first be translated into a 
constraint-based representation, such as, for instance, R1CS or QAP, before any computation proof can be made. Circuits 
can be constructed either through compilation from a high-level language (e.g. Buffet, Pinocchio, Geppetto, TinyRAM...) 
or manually, using low-level circuits engineering frameworks (e.g. Hawk, [jsnark](https://github.com/akosba/jsnark), 
[libsnark](https://github.com/scipr-lab/libsnark)). While the handcrafted circuits offer better performance (i.e. fewer
constraints) and lower-level control over the circuit structure, it is a tedious task for developers. In fact, it requires
the developer to understand the semantics between native constraint ad non-native constraint type operations. `Woodoo` 
aims at filling the gap, providing developers with a more abstract view of circuits and an expressive type-driven 
compiler-assisted engineering toolkit which generates optimized and compact circuits.

## Verifiable computation models

The main intent of `Woodoo` is to abstract over verifiable computation and proof of computational integrity models. The
latter are usually classified as **circuit-based** (e.g. SNARK) or **machine-based** (e.g. STARK). The computation-to-circuit 
and computation-to-machine transformation processes is a crucial part, that or the sequence of transformations for converting
a computation to a mathematical statement. Circuits are represented as algebraic structures that can be transformed to various 
underlying representations, including R1CS/QAP, for SNARK, and AIR (Algebraic Intermediate Representation), for STARK, 
and are amenable to formal verification by means of algebraic reasoning techniques (like the polynomial calculus).
Note that in STARK, the reduction of computations to AIR is coined [*arithmetization*](https://medium.com/starkware/arithmetization-i-15c046390862).
It operates in two successive steps by first generating an execution trace and polynomial constraints and then
transforming them to a single low-degree polynomial.

In a circuit, a (arithmetic) constraint is represented as a polynomial that is applied over the wires of the circuits, as used in R1CS,
for instance. Concretely, R1CS is a structured system of equations over a large finite field (also called Galois field).
R1CS can also express constraints on types, like applying type constraints on circuit variables.

In order to build a ZKP proof, the synopsis is this one:

Statement    -->    transformation      --> Intermediate       -->  proof system    -->  Proof
y = f(x,w)          (frontend)            representation           (backend)

Intermediate representation (e.g. AIR, R1CS, or any logical constraints' system) is crucial, as it can incur big overhead
if not properly structured. One big advantage of R1CS is that it provides linear-algebraic structure to work with, hence 
simplifying the backend (or proof system).

Let's get back to the intermediate representation (IR) notion and compare algebraic intermediate representation (AIR) as
used in STARK and R1CS (Rank 1 Constraint System), that is used by SNARK flavors.

|                        |                                                    AIR                                                    |                          R1CS                           |
|------------------------|:---------------------------------------------------------------------------------------------------------:|:-------------------------------------------------------:|
| **Computation model**  |                                               machine-based                                               |                      circuit-based                      |
| **Technique**          |                                              convert program                                              |                     unroll circuit                      |
| **Polynomials degree** |                                             arbitrary degree                                              |                   degree 2 and rank 1                   |
| **System size**        | as large as the size of the program describing the system (program is often much shorter than the system) | as large as the computation size (i.e. number of steps) |

Proof recursion means that a prover ran a verifier and that verifier verifies some other provers that verify the prover
and so on, recursively. Both R1CS and AIR formalisms can express proof recursions.

### Succinct programs

## Category theory and computations

Relying on category theory, formal models of computation and cryptography are put together, providing a mathematical way 
to turn categorical computational models (including circuits and machines) into zk-SNARKs or zk-STARKs, that verify how 
a sequence of inputs leading to a state change follows the rules specified by the categorical computation model (using
morphisms).

In this project, a **computation** is a convenient abstraction over circuits and machines, so that to ease the mapping to 
different proof systems (also called backends).

## Type checking and inference

## Proof system backend

## References

Brendan **FONG** and David I. **SPIVAK** (2019) An Invitation to Applied Category Theory: Seven Sketchees in Compositionality. Cambridge University Press.

Fabrizio **GENOVESE**, Andre **KNIPSEL** and Joshua **FITZGERALD** (2019) Mapping finite state machines to zk-SNARKS Using Category Theory. ArXiv:1909.02893, https://arxiv.org/abs/1909.02893

Bartosz **MILEWSKI** (2019) Category Theory for Programmers. 

David I. **SPIVAK** (2014) Category Theory for the Sciences. The MIT Press.

Bobbin **THREADBARE**. AirAssembly: a low-level language for encoding AIR of computations

### Useful Links

- Awesome zero knowledge proofs (zkp) - https://github.com/matter-labs/awesome-zero-knowledge-proofs
- CirC compiler - https://github.com/circify/circ
- Typedefs - Language agnostic type system based on category theory and polynomials. https://typedefs.com/