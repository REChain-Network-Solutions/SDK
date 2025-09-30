# Contributing to REChain SDK

Thank you for your interest in contributing to the REChain SDK! We welcome contributions from everyone. This document will help you get started.

## 🚀 Quick Start

1. **Fork** the repository on GitHub
2. **Clone** your fork locally
3. **Create** a feature branch
4. **Make** your changes
5. **Test** your changes thoroughly
6. **Submit** a pull request

## 📋 Contribution Guidelines

### Code Standards

- Follow Rust best practices and idioms
- Use `rustfmt` for code formatting
- Run `clippy` to catch common mistakes
- Write comprehensive tests for new functionality
- Update documentation for API changes

### Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
type(scope): description

[optional body]

[optional footer]
```

**Types:**
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools

**Examples:**
```
feat(pallet-web3): add DApp registry functionality
fix(bridge): resolve cross-chain transfer validation
docs: update API documentation for governance pallet
```

### Pull Request Process

1. **Create** a feature branch from `main`
2. **Write** comprehensive tests
3. **Update** documentation if needed
4. **Run** the full test suite
5. **Submit** your pull request

### Pull Request Checklist

- [ ] Code follows project style guidelines
- [ ] Tests added/updated for new functionality
- [ ] Documentation updated
- [ ] Security implications considered
- [ ] Performance impact assessed
- [ ] Breaking changes documented with migration guide
- [ ] CI/CD passes all checks
- [ ] Manual testing completed

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run specific pallet tests
cargo test -p pallet-web3

# Run with release mode
cargo test --workspace --release

# Run benchmarks
cargo test --workspace --release --features runtime-benchmarks

# Run integration tests
cargo test --workspace --features integration-tests
```

### Writing Tests

- Write unit tests for all public functions
- Include integration tests for cross-pallet functionality
- Test both happy path and error conditions
- Use descriptive test names
- Follow the existing test patterns

## 📚 Documentation

### Documentation Requirements

- Update README files for new pallets
- Document all public APIs
- Include usage examples
- Update integration guides
- Add migration guides for breaking changes

### Building Documentation

```bash
# Generate Rust documentation
cargo doc --workspace --all-features --no-deps

# Check documentation links
cargo doc --workspace --all-features --no-deps --document-private-items
```

## 🔒 Security

### Security Considerations

- Be aware of the security implications of your changes
- Follow secure coding practices
- Validate all inputs
- Handle errors gracefully
- Consider denial-of-service vectors

### Reporting Security Issues

- Do NOT open public issues for security vulnerabilities
- Email security@rechain.network instead
- See our [Security Policy](SECURITY.md) for details

## 🏗️ Development Setup

### Prerequisites

- Rust 1.70.0 or later
- Node.js 18.0.0 or later (for SDK development)
- Docker (optional, for containerized testing)

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/REChain-Network-Solutions/SDK.git
cd SDK

# Build the project
cargo build --workspace

# Run tests
cargo test --workspace

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace --all-targets -- -D warnings
```

## 🎯 Pallet Development

### Creating a New Pallet

1. Create the pallet directory structure
2. Add the pallet to the workspace
3. Implement the pallet trait
4. Write comprehensive tests
5. Add documentation
6. Update CI/CD configuration

### Pallet Structure

```
substrate/frame/your-pallet/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs          # Main pallet implementation
    ├── mock.rs         # Unit testing mock
    └── tests.rs        # Unit tests
```

## 🔄 CI/CD

### Continuous Integration

Our CI/CD pipeline includes:
- Code formatting checks
- Linting with Clippy
- Unit and integration tests
- Security audits
- Performance benchmarks
- Documentation generation

### Pre-commit Hooks

We recommend setting up pre-commit hooks:

```bash
# Install pre-commit (if not already installed)
pip install pre-commit

# Install the hooks
pre-commit install

# Run manually
pre-commit run --all-files
```

## 🌍 Community

### Getting Help

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and discussions
- **Discord**: For real-time chat and support
- **Email**: For private inquiries

### Communication Channels

- **Development Discussions**: GitHub Discussions
- **Technical Support**: GitHub Issues
- **Security Issues**: security@rechain.network
- **Business Inquiries**: info@rechain.network

## 🎉 Recognition

### Contributor Recognition

We recognize and appreciate all contributors:

- **Code Contributions**: Listed in release notes
- **Bug Reports**: Public acknowledgment
- **Feature Requests**: Credit for suggestions
- **Documentation**: Recognition for improvements
- **Community Help**: Special mentions for helpful community members

### Hall of Fame

Contributors who make significant impacts may be inducted into our Hall of Fame.

## 📜 License

By contributing to the REChain SDK, you agree that your contributions will be licensed under the same license as the original project (GPL-3.0).

## 🙏 Thank You

Thank you for contributing to the REChain SDK! Your contributions help make the most advanced blockchain platform even better.

---

*REChain Network Solutions LLC - Building the Future of Decentralized Technology*