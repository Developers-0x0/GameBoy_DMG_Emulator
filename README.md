# GameBoy_DMG_Emulator

An emulator for the original Game Boy DMG-01 (launched in 1989), featuring accurate dot matrix graphics, audio support, and WebAssembly integration for seamless web-based gameplay.

## Features

- **Cycle-accurate emulation**: Faithful reproduction of the Game Boy DMG-01 hardware
- **Cross-platform support**: Windows, Linux, macOS, and web browsers
- **WebAssembly integration**: Play games directly in your browser
- **Modern tech stack**: Built with Rust for performance and safety
- **Modular architecture**: Clean separation between emulation core and platform-specific code

## Tech Stack

This project uses a modern, well-considered tech stack optimized for emulation:

- **Core Language**: Rust (performance, memory safety, excellent WebAssembly support)
- **Graphics**: WebGL for web, OpenGL for native platforms
- **Audio**: Web Audio API for web, SDL2 for native platforms
- **Build System**: Cargo with wasm-pack for WebAssembly builds
- **Testing**: Cargo test with criterion for benchmarking

For detailed information about the technology choices and architecture, see [TECH_STACK.md](TECH_STACK.md).

## Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- For WebAssembly: `wasm-pack` (install with `cargo install wasm-pack`)

### Building

```bash
# Clone the repository
git clone https://github.com/Developers-0x0/GameBoy_DMG_Emulator
cd GameBoy_DMG_Emulator

# Build the native version
cargo build --release

# Build for WebAssembly
wasm-pack build --target web
```

### Running

```bash
# Run with a ROM file
cargo run --release -- path/to/your/rom.gb

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Web Version

```bash
# Build for web
wasm-pack build --target web

# Serve the web version (requires a local server)
cd web && python -m http.server 8000
# Open http://localhost:8000 in your browser
```

## Development

### Project Structure

```
GameBoy_DMG_Emulator/
├── src/
│   ├── emulator/          # Core emulation logic
│   ├── graphics/          # Rendering abstractions
│   ├── platform/          # Platform-specific implementations
│   ├── lib.rs             # Library entry point
│   └── main.rs            # Native application entry point
├── web/                   # Web assets and HTML
├── tests/                 # Integration tests
├── benches/               # Performance benchmarks
├── examples/              # Usage examples
└── docs/                  # Documentation
```

### Building Components

The emulator is built around several key components:

- **CPU**: LR35902 (modified Z80) instruction set implementation
- **PPU**: Picture Processing Unit for graphics rendering
- **APU**: Audio Processing Unit for 4-channel sound synthesis
- **MMU**: Memory Management Unit for address space handling
- **Cartridge**: ROM loading and Memory Bank Controller support

### Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests for your changes
4. Ensure all tests pass: `cargo test`
5. Check code formatting: `cargo fmt`
6. Run linting: `cargo clippy`
7. Submit a pull request

### Testing

```bash
# Run all tests
cargo test

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out html

# Run benchmarks
cargo bench

# Test WebAssembly build
wasm-pack test --chrome
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Game Boy hardware documentation from the community
- Rust emulation community for inspiration and guidance
- WebAssembly working group for excellent tooling

## Status

This project is in early development. The tech stack has been established and basic project structure is in place. Core emulation features are currently being implemented.
