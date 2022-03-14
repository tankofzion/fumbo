# Mystic square example

## Introduction

The [**mystic square**](https://en.wikipedia.org/wiki/15_puzzle) is a sliding puzzle having 15 square tiles numbered 1–15 in a frame that 
is 4 tiles high and 4 tiles wide, leaving one unoccupied tile position. Tiles in the same row or column of the open 
position can be moved by sliding them horizontally or vertically, respectively. The goal of the puzzle is to place the 
tiles in numerical order.

This example is a replica of the one proposed in [Cairo](https://www.cairo-lang.org/docs/hello_cairo/puzzle.html#what-we-need-to-check) language.
It aims at verifying a solution to the mystic square given an initial state. You can prove that you know the solution to 
that initial state without necessarily revealing the solution to the person verifying the proof.

## Build

````shell
cargo build --release --package fumbo-square-example
````

## Run

````shell
cargo run --release --package fumbo-square-example
````

## References
