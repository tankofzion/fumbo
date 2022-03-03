# ZeroCash Example

## Introduction

This sample computation implements the ZeroCash *Pour* circuit (as described in [Eli **BEN-SASSON** et al. 2014]), that
aims providing Bitcoin with strong privacy-preserving guarantees by hiding the flow of money.

This implementation is the exact replica of the seminal *Pour* circuit which allows to appreciate not only the conciseness of
`Woodoo` language, but more importantly, the efficiency of its generator and optimizer when it comes to compare the 
initial hand coded circuit performance and size versus the one that is automatically generated and optimized by `Woodoo`.

## Build

````shell
cargo build --release --package woodoo-zerocash-example
````

## Run

````shell
cargo run --release --package woodoo-zerocash-example
````

## References

1. Eli **BEN-SASSON**, Alessandro **CHIESA**, Christina **GARMAN**, Matthew **GREEN**, Ian **MIERS**, Eran **TROMER** and Madars **VIRZA** (2014) Zerocash: Decentralized anonymous payments from
bitcoin. IARC Cryptology ePrint Archive 2014/349, https://eprint.iacr.org/2014/349.