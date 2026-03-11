# Programming Bitcoin - Test Suite

This directory contains comprehensive integration tests for the Programming Bitcoin implementation in Rust.

## Test Organization

Tests are organized by chapter, with each file containing both unit tests and integration tests for that chapter's concepts.

### Chapter 1: Finite Fields (`ch01_finite_fields_tests.rs`)
Tests for basic finite field arithmetic operations:
- Field element construction and validation
- Addition, subtraction, multiplication, division
- Modular inverse and exponentiation
- Field properties (associativity, commutativity, distributivity)
- Edge cases and error handling

### Chapter 2: Elliptic Curves (`ch02_elliptic_curves_tests.rs`)
Tests for elliptic curve operations over simple u64 fields:
- Point construction and validation
- Point equality and comparison
- Point addition (including point at infinity)
- Curve equation validation
- Overflow detection and handling
- Identity element behavior

### Chapter 3: Bitcoin Cryptography (`ch03_bitcoin_crypto_tests.rs`)
Tests for secp256k1 curve and Bitcoin cryptographic operations:
- S256Field (256-bit field elements)
- S256Point (points on secp256k1 curve)
- Scalar multiplication
- Private key generation
- Signature creation and verification
- Deterministic k-value generation (RFC 6979)
- Field and point arithmetic properties

### Chapter 4: Serialization (`ch04_serialization_tests.rs`)
Tests for Bitcoin serialization formats:
- SEC format (Serialized Elliptic Curve) - compressed and uncompressed
- DER format (Distinguished Encoding Rules) for signatures
- Base58 encoding with checksum
- WIF format (Wallet Import Format)
- Bitcoin address generation (mainnet/testnet, compressed/uncompressed)
- Round-trip serialization/deserialization

## Test Structure

Each test file follows this structure:

```
// ============================================================
// UNIT TESTS - [Category]
// ============================================================
[Unit tests for specific functions/methods]

// ============================================================
// INTEGRATION TESTS - [Category]
// ============================================================
[Integration tests for complete workflows]
```

## Running Tests

Run all tests:
```bash
cargo test
```

Run tests for a specific chapter:
```bash
cargo test --test ch01_finite_fields_tests
cargo test --test ch02_elliptic_curves_tests
cargo test --test ch03_bitcoin_crypto_tests
cargo test --test ch04_serialization_tests
```

Run a specific test:
```bash
cargo test test_field_element_creation
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Run tests in parallel (default):
```bash
cargo test
```

Run tests sequentially:
```bash
cargo test -- --test-threads=1
```

## Test Coverage

The test suite covers:

1. **Happy Path Testing**: Valid inputs and expected outputs
2. **Edge Cases**: Boundary values, zero, one, infinity
3. **Error Handling**: Invalid inputs, overflow detection
4. **Mathematical Properties**: Associativity, commutativity, distributivity
5. **Cryptographic Properties**: Signature verification, deterministic behavior
6. **Serialization Round-trips**: Encode/decode consistency
7. **Format Validation**: Correct byte structure for all formats

## Notes

- Tests do NOT modify main code logic
- Some tests may fail if the implementation has bugs - this is expected
- Tests are written to validate correct behavior, not to make failing code pass
- Integration tests verify complete workflows across multiple components
- Unit tests focus on individual functions and methods

## Test Conventions

- Test names follow the pattern: `test_<what>_<scenario>`
- Each test is independent and can run in any order
- Tests use descriptive assertions with clear failure messages
- Edge cases and error conditions are explicitly tested
- Mathematical properties are verified through property-based assertions

## Future Improvements

Potential additions to the test suite:
- Property-based testing with quickcheck
- Benchmark tests for performance-critical operations
- Fuzz testing for serialization/deserialization
- Cross-validation with reference implementations
- Test vectors from Bitcoin test suite
