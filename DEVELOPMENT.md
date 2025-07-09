# Development Workflow

This document outlines the development workflow for the Game Boy DMG-01 emulator project.

## Development Environment Setup

### Prerequisites

1. **Rust**: Install the latest stable version from [rustup.rs](https://rustup.rs/)
2. **WebAssembly Tools**: Install wasm-pack for WebAssembly builds
   ```bash
   cargo install wasm-pack
   ```
3. **Additional Tools** (optional but recommended):
   ```bash
   # Code formatting and linting
   rustup component add rustfmt clippy
   
   # Coverage analysis
   cargo install cargo-tarpaulin
   
   # Documentation generation
   cargo install mdbook
   ```

### IDE Configuration

#### VS Code
Recommended extensions:
- `rust-analyzer`: Rust language server
- `crates`: Crate dependency management
- `CodeLLDB`: Debugging support

#### Other IDEs
- **CLion**: Built-in Rust support
- **IntelliJ IDEA**: Rust plugin available
- **Vim/Neovim**: rust.vim plugin

## Build Process

### Native Build
```bash
# Debug build
cargo build

# Release build
cargo build --release

# With specific features
cargo build --features "sdl2,opengl"
```

### WebAssembly Build
```bash
# Build for web
wasm-pack build --target web

# Build with optimization
wasm-pack build --target web --release

# Test WebAssembly
wasm-pack test --chrome
```

## Testing Strategy

### Unit Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Integration Tests
```bash
# Run integration tests only
cargo test --test integration_tests

# Run with coverage
cargo tarpaulin --out html
```

### Benchmarks
```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench emulator_step
```

## Code Quality

### Formatting
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting
```bash
# Run clippy
cargo clippy

# Clippy with all features
cargo clippy --all-features

# Fail on warnings
cargo clippy -- -D warnings
```

### Documentation
```bash
# Generate documentation
cargo doc --open

# Test documentation examples
cargo test --doc
```

## Git Workflow

### Branch Strategy
- `main`: Stable, production-ready code
- `develop`: Integration branch for features
- `feature/*`: Feature branches
- `bugfix/*`: Bug fix branches
- `hotfix/*`: Emergency fixes

### Commit Messages
Follow conventional commits format:
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test additions/changes
- `chore`: Maintenance tasks

### Example Commits
```
feat(cpu): implement LR35902 instruction set
fix(ppu): correct scanline timing
docs(readme): add build instructions
test(memory): add MMU boundary tests
```

## Release Process

### Version Management
- Use semantic versioning (MAJOR.MINOR.PATCH)
- Update version in `Cargo.toml`
- Tag releases with `v` prefix (e.g., `v0.1.0`)

### Release Checklist
1. Update version numbers
2. Update CHANGELOG.md
3. Run full test suite
4. Build and test WebAssembly version
5. Update documentation
6. Create release tag
7. Generate release notes

## Performance Optimization

### Profiling
```bash
# Profile with perf (Linux)
cargo build --release
perf record --call-graph=dwarf ./target/release/gameboy_dmg_emulator rom.gb
perf report

# Profile with Instruments (macOS)
cargo instruments -t "Time Profiler" --release --bin gameboy_dmg_emulator -- rom.gb
```

### Benchmarking
```bash
# Run criterion benchmarks
cargo bench

# Generate benchmark report
cargo bench -- --output-format html
```

## Debugging

### Native Debugging
```bash
# Debug build with symbols
cargo build
gdb ./target/debug/gameboy_dmg_emulator

# Or with lldb
lldb ./target/debug/gameboy_dmg_emulator
```

### WebAssembly Debugging
```bash
# Build with debug symbols
wasm-pack build --dev --target web

# Use browser developer tools
# Enable WebAssembly debugging in browser settings
```

## Continuous Integration

### GitHub Actions
The project uses GitHub Actions for CI/CD:
- Build verification for multiple platforms
- Test execution with coverage reporting
- WebAssembly build verification
- Documentation generation
- Release automation

### Local CI Simulation
```bash
# Run the same checks as CI
cargo fmt -- --check
cargo clippy --all-features -- -D warnings
cargo test --all-features
wasm-pack build --target web
```

## Documentation

### Code Documentation
- Document all public APIs
- Include usage examples in doc comments
- Use `cargo doc` to generate documentation

### Project Documentation
- Keep README.md up to date
- Document architecture decisions
- Maintain TECH_STACK.md

### Game Boy Documentation
- Document hardware behavior
- Reference materials and sources
- Explain emulation accuracy decisions

## Contributing Guidelines

### Before Contributing
1. Read the README and TECH_STACK documentation
2. Check existing issues and pull requests
3. Set up the development environment
4. Run tests to ensure everything works

### Pull Request Process
1. Create a feature branch
2. Make your changes
3. Add/update tests as needed
4. Ensure all tests pass
5. Update documentation if necessary
6. Submit a pull request with clear description

### Code Review
- All changes require code review
- Maintain high code quality standards
- Consider performance implications
- Ensure backward compatibility

## Resources

### Game Boy Documentation
- [Pan Docs](https://gbdev.io/pandocs/) - Comprehensive Game Boy documentation
- [Game Boy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
- [Game Boy Memory Map](https://gbdev.gg8.se/wiki/articles/Memory_Map)

### Rust Resources
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [WebAssembly with Rust](https://rustwasm.github.io/docs/book/)

### Emulation Resources
- [Emulation Development Community](https://emulation.gametechwiki.com/)
- [Blargg's Test ROMs](https://github.com/retrio/gb-test-roms)
- [Mooneye GB Test Suite](https://github.com/Gekkio/mooneye-gb)