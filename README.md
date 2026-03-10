# Programming Bitcoin in Rust

This repository contains a Rust implementation of the exercises and concepts from the book ["Programming Bitcoin" by Jimmy Song](https://programmingbitcoin.com/). As a Rust developer, I am using the book to learn Bitcoin, but doing the same exercises in Rust, not by copying the python code, but understanding the logic and rebuilding.

## Overview

"Programming Bitcoin" teaches the mathematical foundations of Bitcoin, including finite fields, elliptic curves, and the secp256k1 curve used in Bitcoin. This implementation follows the book's structure, chapter by chapter, with each chapter containing exercises that build upon the previous ones.

## Project Structure

The code is organized by chapters, mirroring the book's progression:

- `src/ch01/` - Finite Fields
  - Basic field arithmetic operations
  - FieldElement struct with addition, subtraction, multiplication, division, and exponentiation

- `src/ch02/` - Elliptic Curves over Finite Fields
  - Point operations on elliptic curves
  - Point addition and scalar multiplication

- `src/ch03/` - Elliptic Curves over secp256k1
  - Implementation of the secp256k1 curve used in Bitcoin
  - Large number handling with BigUint/BigInt for cryptographic operations

- `src/lib.rs` - Library entry point
- `src/main.rs` - Executable entry point

## Dependencies

This project uses:
- `num-bigint` and `num-traits` for arbitrary-precision arithmetic
- Standard Rust libraries

## Building and Running

### Prerequisites
- Rust 1.70+ (install via [rustup](https://rustup.rs/))

### Build
```bash
cargo build
```

### Run Tests
```bash
cargo test
```

### Run the Executable
```bash
cargo run
```

## Key Concepts Implemented

- **Finite Fields**: Custom FieldElement struct supporting modular arithmetic
- **Elliptic Curves**: Point addition, doubling, and scalar multiplication
- **secp256k1**: Bitcoin's elliptic curve with proper large number handling
- **Cryptographic Primitives**: Modular inverse, exponentiation, and reduction

## Learning Goals

This implementation serves as a learning tool to:
- Understand Bitcoin's mathematical foundations
- Practice Rust programming with complex mathematical concepts
- Implement cryptographic algorithms from scratch
- Compare Python and Rust approaches to the same problems

## References

- [Programming Bitcoin by Jimmy Song](https://programmingbitcoin.com/)
- [Bitcoin Wiki - Elliptic Curve Digital Signature Algorithm](https://en.bitcoin.it/wiki/Elliptic_Curve_Digital_Signature_Algorithm)
- [secp256k1 specification](https://www.secg.org/sec2-v2.pdf)

## License

This project is for educational purposes. The original book's code and concepts belong to Jimmy Song and O'Reilly Media.</content>
<parameter name="filePath">/home/wilfrid_k/projects/boss_challenge/programming_bitcoin/README.md