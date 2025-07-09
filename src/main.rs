//! Game Boy DMG-01 Emulator
//!
//! A cycle-accurate emulator for the original Game Boy (DMG-01) with WebAssembly support.

use std::env;
use std::fs;

use gameboy_dmg_emulator::{GameBoy, EmulatorError};

fn main() -> Result<(), EmulatorError> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <rom_file>", args[0]);
        std::process::exit(1);
    }

    let rom_path = &args[1];
    let rom_data = fs::read(rom_path)
        .map_err(|e| EmulatorError::MemoryError(format!("Failed to read ROM file: {}", e)))?;

    let mut gameboy = GameBoy::new();
    gameboy.load_rom(&rom_data)?;

    println!("Game Boy emulator started");
    println!("ROM loaded: {}", rom_path);
    println!("Press Ctrl+C to exit");

    // Simple emulation loop (will be replaced with proper platform-specific implementation)
    loop {
        gameboy.step();
        
        // In a real implementation, this would be handled by the platform-specific code
        // For now, just run a few cycles and exit
        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
    }
}
