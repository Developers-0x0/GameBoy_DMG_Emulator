//! Game Boy DMG-01 Emulator Library
//!
//! This library provides the core emulation functionality for the Game Boy DMG-01.
//! It's designed to be platform-agnostic and can be used in both native applications
//! and WebAssembly environments.

use thiserror::Error;

pub mod emulator;
pub mod graphics;
pub mod platform;

// Re-export main types for convenience
pub use emulator::GameBoy;

/// Emulator error types
#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("Invalid ROM format")]
    InvalidRom,
    #[error("Unsupported cartridge type: {0}")]
    UnsupportedCartridge(u8),
    #[error("Memory access error: {0}")]
    MemoryError(String),
    #[error("Audio initialization failed: {0}")]
    AudioError(String),
    #[error("Graphics initialization failed: {0}")]
    GraphicsError(String),
}

#[cfg(target_arch = "wasm32")]
pub use platform::web::*;

#[cfg(not(target_arch = "wasm32"))]
pub use platform::native::*;