//! Picture Processing Unit (PPU)
//!
//! Handles the Game Boy's graphics rendering including background, window, and sprites.

/// PPU LCD dimensions
pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;

/// PPU rendering states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    Drawing = 3,
}

/// Picture Processing Unit
pub struct Ppu {
    /// Frame buffer (160x144 pixels, 4 shades of gray)
    pub frame_buffer: [u8; LCD_WIDTH * LCD_HEIGHT],
    
    /// Current scanline being processed
    pub scanline: u8,
    
    /// Current PPU mode
    pub mode: PpuMode,
    
    /// Cycle counter for timing
    pub cycles: u32,
    
    /// LCD control register
    pub lcdc: u8,
    
    /// LCD status register
    pub stat: u8,
    
    /// Scroll registers
    pub scroll_x: u8,
    pub scroll_y: u8,
    
    /// Window position
    pub window_x: u8,
    pub window_y: u8,
    
    /// Background and window palettes
    pub bg_palette: u8,
    pub obj_palette_0: u8,
    pub obj_palette_1: u8,
}

impl Ppu {
    /// Create a new PPU instance
    pub fn new() -> Self {
        Self {
            frame_buffer: [0; LCD_WIDTH * LCD_HEIGHT],
            scanline: 0,
            mode: PpuMode::OamScan,
            cycles: 0,
            lcdc: 0x91,
            stat: 0x00,
            scroll_x: 0,
            scroll_y: 0,
            window_x: 0,
            window_y: 0,
            bg_palette: 0xFC,
            obj_palette_0: 0xFF,
            obj_palette_1: 0xFF,
        }
    }

    /// Step the PPU by one cycle
    pub fn step(&mut self) {
        self.cycles += 1;

        match self.mode {
            PpuMode::OamScan => {
                if self.cycles >= 80 {
                    self.cycles = 0;
                    self.mode = PpuMode::Drawing;
                }
            }
            PpuMode::Drawing => {
                if self.cycles >= 172 {
                    self.cycles = 0;
                    self.mode = PpuMode::HBlank;
                    self.render_scanline();
                }
            }
            PpuMode::HBlank => {
                if self.cycles >= 204 {
                    self.cycles = 0;
                    self.scanline += 1;
                    
                    if self.scanline >= 144 {
                        self.mode = PpuMode::VBlank;
                        // Trigger VBlank interrupt
                    } else {
                        self.mode = PpuMode::OamScan;
                    }
                }
            }
            PpuMode::VBlank => {
                if self.cycles >= 456 {
                    self.cycles = 0;
                    self.scanline += 1;
                    
                    if self.scanline >= 154 {
                        self.scanline = 0;
                        self.mode = PpuMode::OamScan;
                    }
                }
            }
        }
    }

    /// Render the current scanline
    fn render_scanline(&mut self) {
        if self.scanline >= 144 {
            return;
        }

        // Clear scanline
        let line_start = self.scanline as usize * LCD_WIDTH;
        for x in 0..LCD_WIDTH {
            self.frame_buffer[line_start + x] = 0;
        }

        // Render background if enabled
        if self.lcdc & 0x01 != 0 {
            self.render_background();
        }

        // Render window if enabled
        if self.lcdc & 0x20 != 0 {
            self.render_window();
        }

        // Render sprites if enabled
        if self.lcdc & 0x02 != 0 {
            self.render_sprites();
        }
    }

    /// Render background tiles for the current scanline
    fn render_background(&mut self) {
        // Background rendering logic will be implemented here
        // This involves reading tile data from VRAM and applying palettes
    }

    /// Render window tiles for the current scanline
    fn render_window(&mut self) {
        // Window rendering logic will be implemented here
    }

    /// Render sprites for the current scanline
    fn render_sprites(&mut self) {
        // Sprite rendering logic will be implemented here
        // This involves reading OAM data and rendering sprite tiles
    }

    /// Get the current frame buffer
    pub fn get_frame_buffer(&self) -> &[u8] {
        &self.frame_buffer
    }

    /// Check if VBlank has occurred
    pub fn is_vblank(&self) -> bool {
        self.mode == PpuMode::VBlank
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}