# Contributing to Programming Bitcoin

Thank you for your interest in contributing to this project! This is an educational implementation of Bitcoin cryptography in Rust.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/programming_bitcoin.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Format code: `cargo fmt`
7. Run clippy: `cargo clippy`
8. Commit your changes: `git commit -am 'Add some feature'`
9. Push to the branch: `git push origin feature/your-feature-name`
10. Create a Pull Request

## Development Setup

### Prerequisites
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git

### Building
```bash
cargo build
```

### Running Tests
```bash
# Run all tests
cargo test

# Run specific chapter tests
cargo test --test ch01_finite_fields_tests
cargo test --test ch02_elliptic_curves_tests
cargo test --test ch03_bitcoin_crypto_tests
cargo test --test ch04_serialization_tests

# Run a specific test
cargo test test_field_element_creation

# Run with output
cargo test -- --nocapture
```

### Code Quality

Before submitting a PR, ensure:

1. **All tests pass:**
   ```bash
   cargo test
   ```

2. **Code is formatted:**
   ```bash
   cargo fmt
   ```

3. **No clippy warnings:**
   ```bash
   cargo clippy -- -D warnings
   ```

4. **Documentation builds:**
   ```bash
   cargo doc --no-deps
   ```

## Project Structure

```
src/
├── ch01_finite_fields/    # Finite field arithmetic
├── ch02_elliptic_curves/  # Basic elliptic curve operations
├── ch03_ecc/              # Bitcoin's secp256k1 curve
└── ch04_serialization/    # Bitcoin serialization formats

tests/
├── ch01_finite_fields_tests.rs
├── ch02_elliptic_curves_tests.rs
├── ch03_bitcoin_crypto_tests.rs
└── ch04_serialization_tests.rs
```

## Coding Guidelines

### Style
- Follow Rust standard style (enforced by `rustfmt`)
- Use meaningful variable names
- Add comments for complex algorithms
- Keep functions focused and small

### Testing
- Write tests for new functionality
- Maintain or improve code coverage
- Test edge cases and error conditions
- Use descriptive test names: `test_<what>_<scenario>`

### Documentation
- Add doc comments for public APIs
- Include examples in doc comments when helpful
- Update README.md if adding new features

### Commits
- Write clear, descriptive commit messages
- Use present tense ("Add feature" not "Added feature")
- Reference issues when applicable (#123)

## Types of Contributions

### Bug Reports
- Use GitHub Issues
- Include steps to reproduce
- Provide expected vs actual behavior
- Include Rust version and OS

### Feature Requests
- Use GitHub Issues
- Explain the use case
- Describe the proposed solution
- Consider alternatives

### Code Contributions
- Follow the development setup above
- Ensure all checks pass
- Update tests and documentation
- Keep PRs focused on a single concern

### Documentation
- Fix typos and improve clarity
- Add examples
- Improve README or inline docs

## Pull Request Process

1. **Update tests** - Add or update tests for your changes
2. **Update documentation** - Update README.md, doc comments, etc.
3. **Run checks locally** - Ensure tests, fmt, and clippy pass
4. **Create PR** - Provide a clear description of changes
5. **Address feedback** - Respond to review comments
6. **Wait for CI** - All GitHub Actions must pass

## Code Review

All submissions require review. We use GitHub pull requests for this purpose.

Reviewers will check:
- Code quality and style
- Test coverage
- Documentation
- Performance implications
- Security considerations

## Questions?

Feel free to:
- Open an issue for questions
- Start a discussion
- Reach out to maintainers

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (see LICENSE file).

## Learning Resources

This project follows the book "Programming Bitcoin" by Jimmy Song:
- [Programming Bitcoin](https://programmingbitcoin.com/)
- [Bitcoin Wiki](https://en.bitcoin.it/)
- [secp256k1 specification](https://www.secg.org/sec2-v2.pdf)

## Code of Conduct

Be respectful, inclusive, and constructive. This is a learning project - questions and mistakes are welcome!
