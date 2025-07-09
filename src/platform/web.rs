//! WebAssembly platform implementation
//!
//! This module provides WebAssembly bindings for the Game Boy emulator.

use wasm_bindgen::prelude::*;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, ImageData};
use js_sys::Uint8ClampedArray;

use crate::emulator::GameBoy;
use crate::EmulatorError;

/// WebAssembly Game Boy emulator wrapper
#[wasm_bindgen]
pub struct WebGameBoy {
    gameboy: GameBoy,
    canvas: Option<HtmlCanvasElement>,
    context: Option<CanvasRenderingContext2d>,
}

#[wasm_bindgen]
impl WebGameBoy {
    /// Create a new WebAssembly Game Boy emulator
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        
        Self {
            gameboy: GameBoy::new(),
            canvas: None,
            context: None,
        }
    }

    /// Initialize the emulator with a canvas element
    #[wasm_bindgen]
    pub fn init(&mut self, canvas_id: &str) -> Result<(), JsValue> {
        let document = web_sys::window()
            .ok_or("No window object")?
            .document()
            .ok_or("No document object")?;
        
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("Canvas element not found")?
            .dyn_into::<HtmlCanvasElement>()?;
        
        canvas.set_width(160);
        canvas.set_height(144);
        
        let context = canvas
            .get_context("2d")?
            .ok_or("Failed to get 2d context")?
            .dyn_into::<CanvasRenderingContext2d>()?;
        
        self.canvas = Some(canvas);
        self.context = Some(context);
        
        Ok(())
    }

    /// Load a ROM into the emulator
    #[wasm_bindgen]
    pub fn load_rom(&mut self, rom_data: &[u8]) -> Result<(), JsValue> {
        self.gameboy.load_rom(rom_data)
            .map_err(|e| JsValue::from_str(&format!("Failed to load ROM: {}", e)))
    }

    /// Run one frame of emulation
    #[wasm_bindgen]
    pub fn run_frame(&mut self) {
        // Run emulation for one frame (approximately 70224 cycles)
        for _ in 0..70224 {
            self.gameboy.step();
        }
    }

    /// Render the current frame to the canvas
    #[wasm_bindgen]
    pub fn render(&self) -> Result<(), JsValue> {
        let context = self.context.as_ref().ok_or("Context not initialized")?;
        
        let frame_buffer = self.gameboy.get_frame_buffer();
        let mut rgba_data = vec![0u8; 160 * 144 * 4];
        
        // Convert grayscale to RGBA
        for i in 0..frame_buffer.len() {
            let gray = match frame_buffer[i] {
                0 => 255, // White
                1 => 192, // Light gray
                2 => 96,  // Dark gray
                3 => 0,   // Black
                _ => 0,
            };
            
            let rgba_idx = i * 4;
            rgba_data[rgba_idx] = gray;     // Red
            rgba_data[rgba_idx + 1] = gray; // Green
            rgba_data[rgba_idx + 2] = gray; // Blue
            rgba_data[rgba_idx + 3] = 255;  // Alpha
        }
        
        let uint8_array = Uint8ClampedArray::new(&rgba_data.into());
        let image_data = ImageData::new_with_u8_clamped_array(uint8_array, 160)?;
        
        context.put_image_data(&image_data, 0.0, 0.0)?;
        
        Ok(())
    }

    /// Handle key down events
    #[wasm_bindgen]
    pub fn key_down(&mut self, key_code: u32) {
        // Handle input mapping
        match key_code {
            // Arrow keys
            37 => { /* Left */ }
            38 => { /* Up */ }
            39 => { /* Right */ }
            40 => { /* Down */ }
            // Action buttons
            90 => { /* A (Z key) */ }
            88 => { /* B (X key) */ }
            // Start/Select
            13 => { /* Start (Enter) */ }
            32 => { /* Select (Space) */ }
            _ => {}
        }
    }

    /// Handle key up events
    #[wasm_bindgen]
    pub fn key_up(&mut self, key_code: u32) {
        // Handle input release
        match key_code {
            37 => { /* Left */ }
            38 => { /* Up */ }
            39 => { /* Right */ }
            40 => { /* Down */ }
            90 => { /* A (Z key) */ }
            88 => { /* B (X key) */ }
            13 => { /* Start (Enter) */ }
            32 => { /* Select (Space) */ }
            _ => {}
        }
    }

    /// Get audio samples for the current frame
    #[wasm_bindgen]
    pub fn get_audio_samples(&self) -> js_sys::Float32Array {
        let samples = self.gameboy.get_audio_samples();
        let float_samples: Vec<f32> = samples.iter().map(|&s| s as f32 / 32768.0).collect();
        js_sys::Float32Array::from(float_samples.as_slice())
    }
}

/// Initialize the WebAssembly module
#[wasm_bindgen(start)]
pub fn init() {
    console::log_1(&"Game Boy emulator initialized".into());
}

/// WebAssembly utility functions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Console logging macro for WebAssembly
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Error handling for WebAssembly
impl From<EmulatorError> for JsValue {
    fn from(error: EmulatorError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}