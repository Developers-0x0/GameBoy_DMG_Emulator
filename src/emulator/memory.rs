//! Memory Management Unit
//!
//! Handles memory mapping and access for the Game Boy's address space.

use crate::EmulatorError;

/// Game Boy memory map constants
pub const ROM_BANK_0_START: u16 = 0x0000;
pub const ROM_BANK_0_END: u16 = 0x3FFF;
pub const ROM_BANK_N_START: u16 = 0x4000;
pub const ROM_BANK_N_END: u16 = 0x7FFF;
pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;
pub const EXTERNAL_RAM_START: u16 = 0xA000;
pub const EXTERNAL_RAM_END: u16 = 0xBFFF;
pub const WRAM_START: u16 = 0xC000;
pub const WRAM_END: u16 = 0xDFFF;
pub const ECHO_RAM_START: u16 = 0xE000;
pub const ECHO_RAM_END: u16 = 0xFDFF;
pub const OAM_START: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;
pub const UNUSED_START: u16 = 0xFEA0;
pub const UNUSED_END: u16 = 0xFEFF;
pub const IO_REGISTERS_START: u16 = 0xFF00;
pub const IO_REGISTERS_END: u16 = 0xFF7F;
pub const HRAM_START: u16 = 0xFF80;
pub const HRAM_END: u16 = 0xFFFE;
pub const INTERRUPT_ENABLE_REGISTER: u16 = 0xFFFF;

/// Memory management unit
pub struct Mmu {
    // ROM banks (cartridge)
    rom_bank_0: [u8; 0x4000],
    rom_bank_n: [u8; 0x4000],
    
    // Video RAM
    vram: [u8; 0x2000],
    
    // External RAM (cartridge)
    external_ram: [u8; 0x2000],
    
    // Work RAM
    wram: [u8; 0x2000],
    
    // Object Attribute Memory (sprites)
    oam: [u8; 0xA0],
    
    // I/O Registers
    io_registers: [u8; 0x80],
    
    // High RAM
    hram: [u8; 0x7F],
    
    // Interrupt Enable Register
    interrupt_enable: u8,
}

impl Mmu {
    /// Create a new MMU instance
    pub fn new() -> Self {
        Self {
            rom_bank_0: [0; 0x4000],
            rom_bank_n: [0; 0x4000],
            vram: [0; 0x2000],
            external_ram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io_registers: [0; 0x80],
            hram: [0; 0x7F],
            interrupt_enable: 0,
        }
    }

    /// Read a byte from memory
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            ROM_BANK_0_START..=ROM_BANK_0_END => {
                self.rom_bank_0[(address - ROM_BANK_0_START) as usize]
            }
            ROM_BANK_N_START..=ROM_BANK_N_END => {
                self.rom_bank_n[(address - ROM_BANK_N_START) as usize]
            }
            VRAM_START..=VRAM_END => {
                self.vram[(address - VRAM_START) as usize]
            }
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => {
                self.external_ram[(address - EXTERNAL_RAM_START) as usize]
            }
            WRAM_START..=WRAM_END => {
                self.wram[(address - WRAM_START) as usize]
            }
            ECHO_RAM_START..=ECHO_RAM_END => {
                // Echo of WRAM
                self.wram[(address - ECHO_RAM_START) as usize]
            }
            OAM_START..=OAM_END => {
                self.oam[(address - OAM_START) as usize]
            }
            UNUSED_START..=UNUSED_END => {
                // Unused memory space
                0xFF
            }
            IO_REGISTERS_START..=IO_REGISTERS_END => {
                self.io_registers[(address - IO_REGISTERS_START) as usize]
            }
            HRAM_START..=HRAM_END => {
                self.hram[(address - HRAM_START) as usize]
            }
            INTERRUPT_ENABLE_REGISTER => {
                self.interrupt_enable
            }
        }
    }

    /// Write a byte to memory
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            ROM_BANK_0_START..=ROM_BANK_0_END => {
                // ROM is read-only, but MBC may handle this
                // For now, ignore writes to ROM
            }
            ROM_BANK_N_START..=ROM_BANK_N_END => {
                // ROM is read-only, but MBC may handle this
                // For now, ignore writes to ROM
            }
            VRAM_START..=VRAM_END => {
                self.vram[(address - VRAM_START) as usize] = value;
            }
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => {
                self.external_ram[(address - EXTERNAL_RAM_START) as usize] = value;
            }
            WRAM_START..=WRAM_END => {
                self.wram[(address - WRAM_START) as usize] = value;
            }
            ECHO_RAM_START..=ECHO_RAM_END => {
                // Echo of WRAM
                self.wram[(address - ECHO_RAM_START) as usize] = value;
            }
            OAM_START..=OAM_END => {
                self.oam[(address - OAM_START) as usize] = value;
            }
            UNUSED_START..=UNUSED_END => {
                // Unused memory space, ignore writes
            }
            IO_REGISTERS_START..=IO_REGISTERS_END => {
                self.io_registers[(address - IO_REGISTERS_START) as usize] = value;
            }
            HRAM_START..=HRAM_END => {
                self.hram[(address - HRAM_START) as usize] = value;
            }
            INTERRUPT_ENABLE_REGISTER => {
                self.interrupt_enable = value;
            }
        }
    }

    /// Read a 16-bit word from memory (little-endian)
    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;
        (high << 8) | low
    }

    /// Write a 16-bit word to memory (little-endian)
    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }

    /// Load ROM data into memory
    pub fn load_rom(&mut self, rom_data: &[u8]) -> Result<(), EmulatorError> {
        if rom_data.len() < 0x4000 {
            return Err(EmulatorError::InvalidRom);
        }

        // Load ROM bank 0
        self.rom_bank_0.copy_from_slice(&rom_data[0..0x4000]);
        
        // Load ROM bank 1 if available
        if rom_data.len() >= 0x8000 {
            self.rom_bank_n.copy_from_slice(&rom_data[0x4000..0x8000]);
        }

        Ok(())
    }
}

impl Default for Mmu {
    fn default() -> Self {
        Self::new()
    }
}