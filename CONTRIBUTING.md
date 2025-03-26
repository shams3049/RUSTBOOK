# Contributing to RUSTBOOK

Thank you for your interest in contributing to RUSTBOOK! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```
   git clone https://github.com/your-username/RUSTBOOK.git
   ```
3. Create a branch for your changes:
   ```
   git checkout -b feature/your-feature-name
   ```

## Development Process

1. Ensure you have the latest code:
   ```
   git pull origin main
   ```

2. Install dependencies:
   ```
   cargo build
   ```

3. Make your changes

4. Run tests:
   ```
   cargo test
   ```

5. Format code:
   ```
   cargo fmt
   ```

6. Check with Clippy:
   ```
   cargo clippy
   ```

## Pull Requests

1. Push your changes to your fork:
   ```
   git push origin feature/your-feature-name
   ```

2. Create a pull request from your branch to the main repository

3. In your pull request description, clearly describe the changes and link any related issues

## Commit Message Guidelines

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests after the first line

Example:
```
Add user authentication service

- Implement JWT token generation
- Add password hashing
- Create login and register endpoints

Resolves #123
```

## Code Style

- Follow Rust's official style guidelines
- Use meaningful variable and function names
- Document public functions and modules

## Reporting Bugs

When reporting bugs, please include:
- Steps to reproduce the bug
- Expected behavior
- Actual behavior
- Environment details (OS, Rust version, etc.)
- Screenshots if applicable

## Feature Requests

Feature requests are welcome. Please provide:
- A clear description of the feature
- The motivation or use case for the feature
- Any implementation ideas you might have

## License

By contributing to RUSTBOOK, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).
