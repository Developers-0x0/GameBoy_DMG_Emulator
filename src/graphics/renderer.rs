//! Common rendering interface
//!
//! This module defines the common interface for rendering Game Boy graphics.

use crate::EmulatorError;

/// Common renderer trait for different graphics backends
pub trait Renderer {
    /// Initialize the renderer
    fn init(&mut self) -> Result<(), EmulatorError>;
    
    /// Render a frame buffer to the screen
    fn render(&mut self, frame_buffer: &[u8]) -> Result<(), EmulatorError>;
    
    /// Clear the screen
    fn clear(&mut self);
    
    /// Present the rendered frame
    fn present(&mut self);
    
    /// Get the renderer's viewport dimensions
    fn get_viewport(&self) -> (u32, u32);
    
    /// Set the renderer's viewport dimensions
    fn set_viewport(&mut self, width: u32, height: u32);
}

/// Vertex data for quad rendering
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coord: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 2], tex_coord: [f32; 2]) -> Self {
        Self { position, tex_coord }
    }
}

/// Quad vertices for full-screen rendering
pub const QUAD_VERTICES: [Vertex; 4] = [
    Vertex { position: [-1.0, -1.0], tex_coord: [0.0, 1.0] }, // Bottom-left
    Vertex { position: [1.0, -1.0], tex_coord: [1.0, 1.0] },  // Bottom-right
    Vertex { position: [1.0, 1.0], tex_coord: [1.0, 0.0] },   // Top-right
    Vertex { position: [-1.0, 1.0], tex_coord: [0.0, 0.0] },  // Top-left
];

/// Quad indices for triangle rendering
pub const QUAD_INDICES: [u16; 6] = [
    0, 1, 2, // First triangle
    2, 3, 0, // Second triangle
];

/// Shader source code
pub struct ShaderSource {
    pub vertex: &'static str,
    pub fragment: &'static str,
}

/// Basic vertex shader for Game Boy rendering
pub const VERTEX_SHADER: &str = r#"
#version 300 es
precision mediump float;

in vec2 a_position;
in vec2 a_tex_coord;

out vec2 v_tex_coord;

void main() {
    gl_Position = vec4(a_position, 0.0, 1.0);
    v_tex_coord = a_tex_coord;
}
"#;

/// Basic fragment shader for Game Boy rendering
pub const FRAGMENT_SHADER: &str = r#"
#version 300 es
precision mediump float;

in vec2 v_tex_coord;
out vec4 frag_color;

uniform sampler2D u_texture;
uniform vec3 u_palette[4];

void main() {
    float pixel = texture(u_texture, v_tex_coord).r;
    int index = int(pixel * 3.0);
    frag_color = vec4(u_palette[index], 1.0);
}
"#;

/// Game Boy color palette
pub const DMG_PALETTE: [[f32; 3]; 4] = [
    [0.95, 0.95, 0.95], // White
    [0.75, 0.75, 0.75], // Light gray
    [0.375, 0.375, 0.375], // Dark gray
    [0.0, 0.0, 0.0],    // Black
];

/// Rendering configuration
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub vsync: bool,
    pub filter: FilterMode,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 160,
            height: 144,
            scale: 4,
            vsync: true,
            filter: FilterMode::Nearest,
        }
    }
}

/// Texture filtering modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterMode {
    Nearest,
    Linear,
}

/// Utility functions for texture handling
pub fn create_texture_data(frame_buffer: &[u8]) -> Vec<u8> {
    // Convert Game Boy pixel values to texture data
    frame_buffer.iter().map(|&pixel| {
        match pixel {
            0 => 255, // White
            1 => 192, // Light gray
            2 => 96,  // Dark gray
            3 => 0,   // Black
            _ => 0,
        }
    }).collect()
}

/// Convert pixel coordinates to normalized device coordinates
pub fn pixel_to_ndc(x: f32, y: f32, width: f32, height: f32) -> (f32, f32) {
    let ndc_x = (x / width) * 2.0 - 1.0;
    let ndc_y = 1.0 - (y / height) * 2.0;
    (ndc_x, ndc_y)
}

/// Convert normalized device coordinates to pixel coordinates
pub fn ndc_to_pixel(ndc_x: f32, ndc_y: f32, width: f32, height: f32) -> (f32, f32) {
    let x = (ndc_x + 1.0) * width / 2.0;
    let y = (1.0 - ndc_y) * height / 2.0;
    (x, y)
}