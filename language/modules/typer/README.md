# Type checker

This module implements the type checker (hereafter coined `typer`) for the `Woodoo` language. It implements a **dependent type**
type-level computation using macro expansions.

The `Woodoo` language is considered as a *surface* language that relies on Rust host language's
typer.

woodoo syntax -> woodoo typer -> rust typer

In a language with macros (or meta-programming capabilities), a macro expansion compiler phase repeatedly rewrites a program’s 
surface syntax according to all macro definitions, until no macro invocations remain. During expansion, if any of a macro’s 
side-conditions are not satisfied, compilation fails with an error. By implementing type rules as these side-conditions, 
one may implement a type checker as macros. Since macro definitions are modular components, a macro-based type checker 
interleaves checking and transformation.


## Type-level development

The aim of type-level programming is to use the type system to encode computations on type. First trying to dip your toes
in the type-level programming jungle could be an impressive adventure. 


## Dependently-type programming

Dependent types constrain types with values that specify intrinsic properties of programs.
These sorts of types can represent concisely a number of invariants and prevent some classes
of errors at compile time, rather than at runtime, which constitutes a step towards more reliable software. For instance, 
a vector with three items is not the same as a vector of four items, for instance, where the length value
allows discriminating between the two. Here the vector, as a (dependent) type, depends on the value of its length.
The nature of dependently-typed systems makes it possible not only to specify functions with intrinsic constraints, such 
as length of the stack, but also to prove some properties about existing functions as theorems.

Implementing dependent type as macro expansions, where normalization by macro expansion provides
a neat API for type-level programming.



## References

1. Stephen **CHANG**, Michael **BALLANTYNE**, Milo **TURNER** and William J. **BOWMAN** (2020) Dependent Type Systems as Macros. Proc. ACM Program. Lang., Vol. 4, No. POPL, Article 3. Publication date: January 2020
2. The little typer. https://mitpress.mit.edu/books/little-typer.
3. Artjoms **ŠINKAROVS** and Jesper **COCKX** (October 2021) Extracting the Power of Dependent Types.