# Contributing to tjpgdec-rs

Thank you for your interest in contributing to tjpgdec-rs! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Reporting Issues](#reporting-issues)

## Code of Conduct

This project follows the Rust Code of Conduct. Please be respectful and constructive in all interactions.

## How to Contribute

Contributions are welcome in many forms:

- **Bug reports**: Help us identify and fix issues
- **Feature requests**: Suggest new functionality
- **Code contributions**: Fix bugs or implement features
- **Documentation**: Improve README, code comments, or examples
- **Testing**: Verify functionality on different platforms

## Development Setup

### Prerequisites

- Rust 1.70.0 or later
- Cargo (comes with Rust)
- Git

### Clone the Repository

```bash
git clone https://github.com/planet0104/tjpgdec-rs.git
cd tjpgdec-rs
```

### Build the Project

```bash
# Build the library
cargo build

# Build with specific features
cargo build --features fast-decode-2

# Build all examples
cargo build --examples
```

### Run Tests

```bash
# Run all tests
cargo test

# Run specific example
cargo run --example basic
```

## Coding Standards

### Rust Style

- Follow standard Rust formatting: use `cargo fmt` before committing
- Run `cargo clippy` and address all warnings
- Use meaningful variable and function names
- Add comprehensive documentation comments for public APIs

### Code Organization

- Keep functions focused and concise
- Maintain `no_std` compatibility for core functionality
- Avoid unnecessary heap allocations
- Consider embedded system constraints (memory, performance)

### Documentation

- All public APIs must have documentation comments
- Include usage examples in doc comments where helpful
- Use English for all code comments and documentation
- Update README.md if adding new features or changing behavior

### Example of Good Documentation

```rust
/// Decodes a JPEG image from the input buffer.
///
/// # Arguments
///
/// * `input` - Raw JPEG data as byte slice
/// * `output` - Buffer to store decoded pixels
/// * `format` - Desired output pixel format
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error code on failure.
///
/// # Example
///
/// ```no_run
/// use tjpgdec_rs::{JpegDecoder, OutputFormat, Result};
///
/// fn decode_jpeg(data: &[u8]) -> Result<()> {
///     let mut decoder = JpegDecoder::new();
///     // ... decoding logic
///     Ok(())
/// }
/// ```
pub fn decompress(&mut self, output: &mut [u8], format: OutputFormat) -> Result<()> {
    // Implementation
}
```

## Testing

### Test Requirements

- All new features must include tests
- Bug fixes should include regression tests
- Tests must pass on stable Rust
- Examples should compile without errors

### Running Tests

```bash
# Run unit tests
cargo test

# Test with different features
cargo test --features fast-decode-2
cargo test --no-default-features

# Run examples
cargo run --example basic
cargo run --example jpg2bmp -- input.jpg output.bmp
```

### Performance Testing

For performance-critical changes:

```bash
# Compare memory usage
cargo run --example memory_comparison

# Test buffer sizes
cargo run --example size_check
```

## Pull Request Process

### Before Submitting

1. **Create a branch**: Use a descriptive branch name
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**:
   - Write clean, well-documented code
   - Follow the coding standards above
   - Add tests for new functionality

3. **Test thoroughly**:
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   cargo build --examples
   ```

4. **Update documentation**:
   - Update README.md if needed
   - Update CHANGELOG.md following the existing format
   - Ensure all doc comments are accurate

### Submitting the Pull Request

1. **Commit your changes**:
   ```bash
   git add .
   git commit -m "Brief description of changes"
   ```

2. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

3. **Create Pull Request**:
   - Provide a clear title and description
   - Reference any related issues
   - Explain the motivation for changes
   - List any breaking changes

### Pull Request Checklist

- [ ] Code follows project style guidelines
- [ ] `cargo fmt` has been run
- [ ] `cargo clippy` produces no warnings
- [ ] All tests pass (`cargo test`)
- [ ] New tests added for new functionality
- [ ] Documentation updated (README, CHANGELOG, doc comments)
- [ ] Examples compile successfully
- [ ] Commit messages are clear and descriptive

## Reporting Issues

### Bug Reports

When reporting bugs, please include:

- **Description**: Clear description of the issue
- **Steps to Reproduce**: Minimal code example that demonstrates the bug
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Environment**:
  - Rust version (`rustc --version`)
  - Operating system
  - Target platform (if embedded)
  - Feature flags used
- **Additional Context**: Any other relevant information

### Feature Requests

When requesting features:

- **Use Case**: Describe the problem you're trying to solve
- **Proposed Solution**: How you envision the feature working
- **Alternatives**: Other approaches you've considered
- **Impact**: How this would benefit the project

## Project Priorities

When contributing, keep in mind the project's core goals:

1. **Embedded-first**: Optimized for resource-constrained systems
2. **No heap allocation**: Use stack-based buffers when possible
3. **Performance**: Fast decoding with minimal overhead
4. **Compatibility**: Support various MCU platforms (ESP32, ARM, etc.)
5. **Safety**: Leverage Rust's safety features

## Questions?

If you have questions about contributing:

- Open an issue with the "question" label
- Check existing issues and pull requests
- Review the README.md and documentation

## License

By contributing to tjpgdec-rs, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

---

Thank you for contributing to tjpgdec-rs! 🎉
