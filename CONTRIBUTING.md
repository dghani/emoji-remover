# Contributing to emoji-remover

First off, thank you for considering contributing to emoji-remover!

## How Can I Contribute?

### Reporting Bugs

- Check if the bug has already been reported in [Issues](https://github.com/jamiepine/emoji-remover/issues)
- If not, create a new issue with:
  - A clear title and description
  - Steps to reproduce
  - Expected vs actual behavior
  - Your environment (OS, Rust version)

### Suggesting Enhancements

- Check existing issues first
- Create a new issue describing:
  - The enhancement and its benefits
  - Possible implementation approach
  - Any potential drawbacks

### Pull Requests

1. Fork the repo and create your branch from `main`
2. Make your changes
3. Add tests if applicable
4. Ensure the test suite passes: `cargo test`
5. Make sure your code follows Rust conventions: `cargo clippy`
6. Format your code: `cargo fmt`
7. Commit with clear, descriptive messages
8. Push to your fork and submit a pull request

## Development Setup

```bash
# Clone your fork
git clone https://github.com/your-username/emoji-remover.git
cd emoji-remover

# Build the project
cargo build

# Run tests
cargo test

# Run with sample files
cargo run -- --dry-run test.txt
```

## Code Style

- Follow standard Rust naming conventions
- Use `cargo fmt` before committing
- Address all `cargo clippy` warnings
- Write clear comments for complex logic
- Keep functions focused and small

## License

By contributing, you agree that your contributions will be licensed under the MIT License.