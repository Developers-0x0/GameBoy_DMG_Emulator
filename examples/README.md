# Game Boy DMG-01 Emulator Examples

This directory contains examples demonstrating how to use the Game Boy emulator library.

## Basic Usage

```rust
use gameboy_dmg_emulator::GameBoy;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom_data = fs::read("game.gb")?;
    let mut gameboy = GameBoy::new();
    gameboy.load_rom(&rom_data)?;
    
    // Simple emulation loop
    loop {
        gameboy.step();
        // Handle rendering and input
    }
}
```

## Web Usage

For WebAssembly usage, see the `web/index.html` file for a complete example.