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

- `src/ch04/` - Serialization
  - Serialization of secp256k1 points and signatures
  - SEC (Standards for Efficient Cryptography) format implementation

- `src/ch05/` - Transactions
  - Bitcoin transaction parsing and structure
  - Variable-length integer encoding/decoding
  - Transaction input/output handling

- `src/lib.rs` - Library entry point
- `src/main.rs` - Executable entry point
- `tests/` - Integration tests for each chapter

## Dependencies

This project uses the following Rust crates:
- `num-bigint` and `num-traits` - Arbitrary-precision arithmetic for large numbers
- `secp256k1` - secp256k1 elliptic curve cryptography library
- `hex` - Hexadecimal encoding/decoding
- `serde` and `serde_json` - Serialization/deserialization
- `sha2` - SHA-256 hashing
- `hmac` - HMAC (Hash-based Message Authentication Code)
- `rand` - Random number generation
- `bs58` - Base58 encoding (used in Bitcoin addresses)
- `ripemd` - RIPEMD-160 hashing
- `reqwest` - HTTP client for API interactions
- `tokio` - Asynchronous runtime
- `dotenvy` - Environment variable loading

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

## CI/CD

This project uses GitHub Actions for continuous integration and deployment. The CI pipeline includes:

- **Automated Testing**: Runs `cargo test` on multiple Rust versions and operating systems
- **Code Formatting**: Ensures code follows Rust formatting standards with `cargo fmt`
- **Linting**: Checks for common issues with `cargo clippy`
- **Security Auditing**: Scans dependencies for vulnerabilities

### CI Configuration

The CI workflow is defined in `.github/workflows/ci.yml` and includes jobs for:
- Building and testing on Linux, macOS, and Windows
- Code quality checks
- Dependency auditing

To set up CI/CD locally or contribute:
1. Ensure your code passes `cargo fmt` and `cargo clippy`
2. Run the full test suite with `cargo test`
3. Push to a branch to trigger CI checks

## Key Concepts Implemented

- **Finite Fields**: Custom FieldElement struct supporting modular arithmetic
- **Elliptic Curves**: Point addition, doubling, and scalar multiplication
- **secp256k1**: Bitcoin's elliptic curve with proper large number handling
- **Serialization**: SEC format for public key compression and uncompressed formats
- **Digital Signatures**: ECDSA signature creation and verification
- **Transactions**: Bitcoin transaction structure, parsing, and validation
- **Variable-Length Integers**: CompactSize encoding for Bitcoin protocol
- **Cryptographic Primitives**: Modular inverse, exponentiation, and reduction

## Learning Goals

This implementation serves as a learning tool to:
- Understand Bitcoin's mathematical foundations from finite fields to transaction validation
- Practice Rust programming with complex mathematical concepts and cryptography
- Implement cryptographic algorithms and protocols from scratch
- Compare Python and Rust approaches to the same problems
- Learn Bitcoin's serialization formats and transaction structure
- Gain experience with real-world cryptographic libraries and best practices

## References

- [Programming Bitcoin by Jimmy Song](https://programmingbitcoin.com/)
- [Bitcoin Wiki - Elliptic Curve Digital Signature Algorithm](https://en.bitcoin.it/wiki/Elliptic_Curve_Digital_Signature_Algorithm)
- [secp256k1 specification](https://www.secg.org/sec2-v2.pdf)

## License

This project is for educational purposes. The original book's code and concepts belong to Jimmy Song and O'Reilly Media.</content>
<parameter name="filePath">/home/wilfrid_k/projects/boss_challenge/programming_bitcoin/README.md