# Contributing to Minecraft Map Factory

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- Linux: GTK3, WebKit2GTK, and related dev libraries (see CI workflow for exact packages)
- macOS: Xcode command line tools
- Windows: MSVC build tools

### Building

```bash
# Clone the repository
git clone https://github.com/jdogrocks/minecraft-map-factory.git
cd minecraft-map-factory

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test --all-targets --all-features
```

### Feature Flags

- `gui` (default) - Enables the Tauri-based GUI
- `bedrock` - Enables Bedrock Edition world output

To build without the GUI (CLI only):

```bash
cargo build --no-default-features
```

## Development Workflow

1. **Fork** the repository and create a feature branch from `main`
2. **Write code** following the existing style and conventions
3. **Run checks** before submitting:
   ```bash
   cargo fmt -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-targets --all-features
   cargo audit
   ```
4. **Open a Pull Request** against `main`

## Pull Request Requirements

- All CI checks must pass (build, test, clippy, cargo-audit)
- At least one review approval is required
- Keep PRs focused - one feature or fix per PR
- Write clear commit messages describing the change

## Code Style

- Follow standard Rust formatting (`cargo fmt`)
- All clippy warnings must be resolved (`-D warnings`)
- Add tests for new functionality where practical

## Reporting Issues

- Use GitHub Issues for bug reports and feature requests
- Include reproduction steps for bugs
- Include your OS, Rust version, and Minecraft version where relevant

## License

By contributing, you agree that your contributions will be licensed under the [Apache 2.0 License](LICENSE).
