# Asset portfolio valuation example

## Introduction

This financial example shows how to compute the net asset value of a portfolio.

In your day-to-day life, you are constantly using zero-knowledge proof concepts, without even knowing it! For instance,
suppose that your children borrow your car to go to the cinema. You can later assume they are back home by seeing your
car in the driveway, even though you haven't seen your children. Here, the car provides you a zero-knowledge proof of their
presence.

Arithmetic circuits are a model for computing polynomials. Polynomials are sums of terms containing constants (such as 1,2,3), 
variables (such as x,y,z), and exponents of variables (such as x², y³). The latter are similar to boolean
circuits.

## Cryptographic backend
This example relies on zk-SNARK cryptographic protocol for its implementation. z-SNARK is a succinct and non-interactive
protocol which relies on arithmetic circuits for building proofs of solutions. Succinctness implies that it uses elliptic 
curves and non-interactivity, that it requires an oracle or a common reference string (CRS).


## Build

````shell
cargo build --release --package woodoo-portfolio-example
````

## Run

````shell
cargo run --release --package woodoo-portfolio-example
````

## References

1. Ronald **CRAMER** and Ivan B. **DAMGARD** (November 1997) Zero-Knowledge Proofs for Finite Field Arithmetic or: Can Zero-Knowledge be for Free?
Basic Research in Computer Science (BRICS) RS-97-27, https://www.brics.dk/RS/97/27/BRICS-RS-97-27.pdf

