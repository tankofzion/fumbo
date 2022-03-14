# Decentralized autonomous credit scoring organization

## Introduction

Rather than relying on central [credit bureaus](https://www.investopedia.com/terms/c/creditbureau.asp) (like  Experian, 
Equifax, and TransUnion, in United States) to collect and synthesize information regarding credit risk for lending institutions, 
a decentralized approach can be considered, that could leverage on collaborative proof systems and multi-party computation 
techniques (Alex **OZDEMIR** and Dan **BONEH**, February 22, 2022) so that to build a **decentralized autonomous credit scoring 
organization**.

Suppose a lender is reviewing a loan application. A simple use case would be the lender wanting to evaluate the borrowing 
capacity of a credit applicant by checking her/his financial situation (i.e. credits and debits) with the bank(s) where 
he/she has one or more accounts. The applicant could request a collaborative proof from the bank(s) to prove that the 
ratio between her/his debts and liabilities does not exceed an expected threshold.

## Build

````shell
cargo build --release --package fumbo-credit-example
````

## Run

````shell
cargo run --release --package fumbo-credit-example
````

## References

Alex **OZDEMIR** and Dan **BONEH** (February 22, 2022) Experimenting with collaborative zk-SNARKs: Zero-knowledge proofs
for distributed secrets. Cryptology ePrint Archive: Report 2021/1530. https://eprint.iacr.org/2021/1530.pdf.
