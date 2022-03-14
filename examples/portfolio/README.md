# Asset portfolio valuation example

## Introduction

This financial example shows how to compute the net asset value of a portfolio.

In your day-to-day life, you are constantly using zero-knowledge proof concepts, without even knowing it! For instance,
suppose that your children borrow your car to go to the cinema. You can later assume they are back home by seeing your
car in the driveway, even though you haven't seen your children. Here, the car provides you a zero-knowledge proof of their
presence.

Arithmetic circuits are a model for computing polynomials. Polynomials are sums of terms containing constants (such as 1,2,3), 
variables (such as x,y,z), and exponents of variables (such as x², y³). Arithmetic circuits are very similar to boolean  
circuits. Using `Woodoo` we can abstract over computation models, whether they are machine-based (like zk-STARK) or
circuit-based (like zk-SNARK). `Woodoo` uses category theory behind the scene.

## Proof systems

This example uses two different proof systems for its implementation, namely zk-SNARK and zk-STARK. Remember that z-SNARK
is a succinct non-interactive protocol which relies on arithmetic circuits for building proofs of solutions. On the other
hand, zk-STARK relies on machine-based model to 

The main interest of using these two different backends (or proof systems) is to compare performance between the two methods,
with the intuition that zk-STARK seems better for large calculation batches like the one encountered in this asset evaluation
example, where numerous are computed.

## Build

````shell
cargo build --release --package fumbo-portfolio-example
````

## Run

````shell
cargo run --release --package fumbo-portfolio-example
````

## References

1. Ronald **CRAMER** and Ivan B. **DAMGARD** (November 1997) Zero-Knowledge Proofs for Finite Field Arithmetic or: Can Zero-Knowledge be for Free?
Basic Research in Computer Science (BRICS) RS-97-27, https://www.brics.dk/RS/97/27/BRICS-RS-97-27.pdf

