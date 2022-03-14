# Algebraic Arithmetic Circuits Verification Tool

This component implements a tool for formally verified arithmetic circuits using algebraic reasoning.

## Overview
An algebraic circuit (hereafter simply called *circuit*) over a finite (or Galois) field 𝔽 is a layered directed acyclic 
graph (or DAG) with one sink node called an output node, source nodes (called input nodes) and are labeled by variables 
or field constants. Non-input nodes are labeled by × (multiplication gate) and + (addition gate) in alternate layers. 
Sometimes edges may be labeled by field constants. 

The complexity parameters of a circuit are its **size** (i.e. number of edges and vertices, including the variables), 
**depth** (i.e. number of layers) and **degree** (i.e. maximum degree among all polynomials computed at each node). Note
that the degree of the computed polynomial may be much smaller than the degree of its circuit. The polynomial computed 
by  a circuit may have, in the worst case, an exponential number of monomials compared with its size.

## References

P. Beame and V. Liew. Towards verifying nonlinear integer arithmetic. In CAV, volume 10427 of LNCS, pages 238–258. Springer, 2017.

M. Clegg, J. Edmonds, and R. Impagliazzo. Using the groebner basis algorithm to
find proofs of unsatisfiability. In STOC, pages 174–183. ACM, 1996.

