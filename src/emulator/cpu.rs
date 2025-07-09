//! LR35902 CPU Implementation
//!
//! The Game Boy uses a modified Z80 CPU called the LR35902.
//! This module implements the CPU instruction set and execution logic.

use bitflags::bitflags;

bitflags! {
    /// CPU flags register
    pub struct Flags: u8 {
        const ZERO = 0b1000_0000;
        const NEGATIVE = 0b0100_0000;
        const HALF_CARRY = 0b0010_0000;
        const CARRY = 0b0001_0000;
    }
}

/// LR35902 CPU state
pub struct Cpu {
    // Registers
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub flags: Flags,
    
    // Special registers
    pub pc: u16,  // Program counter
    pub sp: u16,  // Stack pointer
    
    // Internal state
    pub cycles: u64,
    pub halted: bool,
    pub ime: bool,  // Interrupt master enable
}

impl Cpu {
    /// Create a new CPU instance
    pub fn new() -> Self {
        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            flags: Flags::from_bits_truncate(0xB0),
            pc: 0x0100,
            sp: 0xFFFE,
            cycles: 0,
            halted: false,
            ime: false,
        }
    }

    /// Execute one CPU instruction
    pub fn step(&mut self) {
        if self.halted {
            // Handle interrupts when halted
            return;
        }

        // Fetch, decode, and execute instruction
        // This will be implemented with the full instruction set
    }

    /// Get combined AF register
    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | self.flags.bits() as u16
    }

    /// Set combined AF register
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.flags = Flags::from_bits_truncate(value as u8 & 0xF0);
    }

    /// Get combined BC register
    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    /// Set combined BC register
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    /// Get combined DE register
    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    /// Set combined DE register
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    /// Get combined HL register
    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    /// Set combined HL register
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}