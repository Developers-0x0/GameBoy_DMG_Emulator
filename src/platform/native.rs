//! Native platform implementation
//!
//! This module provides native desktop implementations for the Game Boy emulator.

use std::time::{Duration, Instant};

use crate::emulator::GameBoy;
use crate::EmulatorError;

/// Native Game Boy emulator wrapper
pub struct NativeGameBoy {
    gameboy: GameBoy,
    last_frame_time: Instant,
    target_frame_time: Duration,
}

impl NativeGameBoy {
    /// Create a new native Game Boy emulator
    pub fn new() -> Self {
        Self {
            gameboy: GameBoy::new(),
            last_frame_time: Instant::now(),
            target_frame_time: Duration::from_nanos(16_666_667), // ~60 FPS
        }
    }

    /// Load a ROM into the emulator
    pub fn load_rom(&mut self, rom_data: &[u8]) -> Result<(), EmulatorError> {
        self.gameboy.load_rom(rom_data)
    }

    /// Run one frame of emulation with timing
    pub fn run_frame(&mut self) {
        let now = Instant::now();
        
        // Only run if enough time has passed
        if now.duration_since(self.last_frame_time) >= self.target_frame_time {
            // Run emulation for one frame (approximately 70224 cycles)
            for _ in 0..70224 {
                self.gameboy.step();
            }
            
            self.last_frame_time = now;
        }
    }

    /// Get the current frame buffer
    pub fn get_frame_buffer(&self) -> &[u8] {
        self.gameboy.get_frame_buffer()
    }

    /// Get audio samples for the current frame
    pub fn get_audio_samples(&self) -> &[i16] {
        self.gameboy.get_audio_samples()
    }

    /// Check if it's time for the next frame
    pub fn should_render(&self) -> bool {
        Instant::now().duration_since(self.last_frame_time) >= self.target_frame_time
    }
}

impl Default for NativeGameBoy {
    fn default() -> Self {
        Self::new()
    }
}

/// Input handling for native platforms
pub struct InputHandler {
    // Game Boy button states
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
}

impl InputHandler {
    /// Create a new input handler
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false,
        }
    }

    /// Get the current joypad state as a byte
    pub fn get_joypad_state(&self) -> u8 {
        let mut state = 0xFF;
        
        if self.right { state &= !0x01; }
        if self.left { state &= !0x02; }
        if self.up { state &= !0x04; }
        if self.down { state &= !0x08; }
        if self.a { state &= !0x10; }
        if self.b { state &= !0x20; }
        if self.select { state &= !0x40; }
        if self.start { state &= !0x80; }
        
        state
    }

    /// Reset all button states
    pub fn reset(&mut self) {
        self.up = false;
        self.down = false;
        self.left = false;
        self.right = false;
        self.a = false;
        self.b = false;
        self.start = false;
        self.select = false;
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Audio handling for native platforms
pub struct AudioHandler {
    // Audio system state
    sample_rate: u32,
    buffer: Vec<i16>,
}

impl AudioHandler {
    /// Create a new audio handler
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            buffer: Vec::new(),
        }
    }

    /// Add audio samples to the buffer
    pub fn add_samples(&mut self, samples: &[i16]) {
        self.buffer.extend_from_slice(samples);
    }

    /// Get the current audio buffer
    pub fn get_buffer(&self) -> &[i16] {
        &self.buffer
    }

    /// Clear the audio buffer
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    /// Get the sample rate
    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

/// Display handling for native platforms
pub struct DisplayHandler {
    width: u32,
    height: u32,
    scale: u32,
}

impl DisplayHandler {
    /// Create a new display handler
    pub fn new(scale: u32) -> Self {
        Self {
            width: 160,
            height: 144,
            scale,
        }
    }

    /// Get the display dimensions
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width * self.scale, self.height * self.scale)
    }

    /// Convert Game Boy pixel data to RGB
    pub fn convert_to_rgb(&self, frame_buffer: &[u8]) -> Vec<u8> {
        let mut rgb_data = Vec::with_capacity(frame_buffer.len() * 3);
        
        for &pixel in frame_buffer {
            let color = match pixel {
                0 => 255, // White
                1 => 192, // Light gray
                2 => 96,  // Dark gray
                3 => 0,   // Black
                _ => 0,
            };
            
            rgb_data.push(color); // Red
            rgb_data.push(color); // Green
            rgb_data.push(color); // Blue
        }
        
        rgb_data
    }

    /// Scale pixel data for display
    pub fn scale_pixels(&self, rgb_data: &[u8]) -> Vec<u8> {
        if self.scale == 1 {
            return rgb_data.to_vec();
        }
        
        let mut scaled_data = Vec::with_capacity(rgb_data.len() * (self.scale * self.scale) as usize);
        
        for y in 0..self.height {
            for _ in 0..self.scale {
                for x in 0..self.width {
                    let pixel_idx = (y * self.width + x) as usize * 3;
                    let pixel = &rgb_data[pixel_idx..pixel_idx + 3];
                    
                    for _ in 0..self.scale {
                        scaled_data.extend_from_slice(pixel);
                    }
                }
            }
        }
        
        scaled_data
    }
}

impl Default for DisplayHandler {
    fn default() -> Self {
        Self::new(4) // 4x scale by default
    }
}