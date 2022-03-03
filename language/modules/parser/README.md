# Syntax analyzer

## Introduction

The module implement the parser (or syntax analyzer) for the `Woodoo` language. The latter leverages on Rust compiler infrastructure, such
as, for instance, the [`syn`]() and `quote` crates and procedural macros, to implement syntactic analysis.

The code is organized so that to stick as much as possible to the syntactic structure of the language, splitting source 
code into major components such as, for instance, expressions, statements, or primitives. 

| Module                         | Description                                                                                                                                                     |
|--------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------|
 | [`program`](./program)         | This module is the main language module. It implements the data structure and algorithms of a verifiable computing [Program](./mod) defines  entry point module | 
| [`items`](./items)             | Module implementing parsing routines for items declared by declaration statements.                                                                              |
| [`primitives`](./primitives)   | Common data structures and algorithms used for implementing the parser.                                                                                         |
| [`statements`](./statements)   | Module implementing parsing routines for analysing syntactic structure of language statements.                                                                  |
| [`expressions`](./expressions) | Syntax model and parsing routines of language expressions.                                                                                                      |
