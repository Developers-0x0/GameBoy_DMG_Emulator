//! Cartridge and Memory Bank Controller (MBC) support
//!
//! Handles different cartridge types and memory bank switching.

use crate::EmulatorError;

/// Cartridge header information
#[derive(Debug, Clone)]
pub struct CartridgeHeader {
    pub title: String,
    pub cartridge_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub destination_code: u8,
    pub old_licensee_code: u8,
    pub mask_rom_version: u8,
    pub header_checksum: u8,
    pub global_checksum: u16,
}

/// Memory Bank Controller types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MbcType {
    None,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
    Unsupported(u8),
}

/// Cartridge implementation
pub struct Cartridge {
    pub header: CartridgeHeader,
    pub mbc_type: MbcType,
    pub rom_data: Vec<u8>,
    pub ram_data: Vec<u8>,
    
    // MBC state
    pub current_rom_bank: usize,
    pub current_ram_bank: usize,
    pub ram_enabled: bool,
    pub banking_mode: u8,
}

impl Cartridge {
    /// Create a new cartridge from ROM data
    pub fn new(rom_data: Vec<u8>) -> Result<Self, EmulatorError> {
        if rom_data.len() < 0x8000 {
            return Err(EmulatorError::InvalidRom);
        }

        let header = Self::parse_header(&rom_data)?;
        let mbc_type = Self::determine_mbc_type(header.cartridge_type)?;
        let ram_size = Self::get_ram_size(header.ram_size);
        
        Ok(Self {
            header,
            mbc_type,
            rom_data,
            ram_data: vec![0; ram_size],
            current_rom_bank: 1,
            current_ram_bank: 0,
            ram_enabled: false,
            banking_mode: 0,
        })
    }

    /// Parse the cartridge header
    fn parse_header(rom_data: &[u8]) -> Result<CartridgeHeader, EmulatorError> {
        if rom_data.len() < 0x150 {
            return Err(EmulatorError::InvalidRom);
        }

        // Extract title (0x134-0x143)
        let title_bytes = &rom_data[0x134..0x144];
        let title_end = title_bytes.iter().position(|&b| b == 0).unwrap_or(title_bytes.len());
        let title = String::from_utf8_lossy(&title_bytes[..title_end]).to_string();

        let header = CartridgeHeader {
            title,
            cartridge_type: rom_data[0x147],
            rom_size: rom_data[0x148],
            ram_size: rom_data[0x149],
            destination_code: rom_data[0x14A],
            old_licensee_code: rom_data[0x14B],
            mask_rom_version: rom_data[0x14C],
            header_checksum: rom_data[0x14D],
            global_checksum: ((rom_data[0x14E] as u16) << 8) | rom_data[0x14F] as u16,
        };

        // Verify header checksum
        let mut checksum: u8 = 0;
        for i in 0x134..0x14D {
            checksum = checksum.wrapping_sub(rom_data[i]).wrapping_sub(1);
        }
        
        if checksum != header.header_checksum {
            log::warn!("Header checksum mismatch: expected {}, got {}", header.header_checksum, checksum);
        }

        Ok(header)
    }

    /// Determine MBC type from cartridge type byte
    fn determine_mbc_type(cartridge_type: u8) -> Result<MbcType, EmulatorError> {
        match cartridge_type {
            0x00 => Ok(MbcType::None),
            0x01..=0x03 => Ok(MbcType::Mbc1),
            0x05..=0x06 => Ok(MbcType::Mbc2),
            0x0F..=0x13 => Ok(MbcType::Mbc3),
            0x19..=0x1E => Ok(MbcType::Mbc5),
            _ => Err(EmulatorError::UnsupportedCartridge(cartridge_type)),
        }
    }

    /// Get RAM size in bytes
    fn get_ram_size(ram_size_code: u8) -> usize {
        match ram_size_code {
            0x00 => 0,
            0x01 => 2 * 1024,     // 2KB
            0x02 => 8 * 1024,     // 8KB
            0x03 => 32 * 1024,    // 32KB
            0x04 => 128 * 1024,   // 128KB
            0x05 => 64 * 1024,    // 64KB
            _ => 0,
        }
    }

    /// Read from cartridge memory space
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                // ROM Bank 0
                self.rom_data[address as usize]
            }
            0x4000..=0x7FFF => {
                // ROM Bank N
                let bank_offset = self.current_rom_bank * 0x4000;
                let local_address = (address - 0x4000) as usize;
                self.rom_data[bank_offset + local_address]
            }
            0xA000..=0xBFFF => {
                // External RAM
                if self.ram_enabled && !self.ram_data.is_empty() {
                    let bank_offset = self.current_ram_bank * 0x2000;
                    let local_address = (address - 0xA000) as usize;
                    self.ram_data[bank_offset + local_address]
                } else {
                    0xFF
                }
            }
            _ => 0xFF,
        }
    }

    /// Write to cartridge memory space
    pub fn write(&mut self, address: u16, value: u8) {
        match self.mbc_type {
            MbcType::None => {
                // No MBC, ignore writes
            }
            MbcType::Mbc1 => {
                self.handle_mbc1_write(address, value);
            }
            MbcType::Mbc2 => {
                self.handle_mbc2_write(address, value);
            }
            MbcType::Mbc3 => {
                self.handle_mbc3_write(address, value);
            }
            MbcType::Mbc5 => {
                self.handle_mbc5_write(address, value);
            }
            MbcType::Unsupported(_) => {
                // Unsupported MBC, ignore writes
            }
        }
    }

    /// Handle MBC1 writes
    fn handle_mbc1_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                // RAM Enable
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            0x2000..=0x3FFF => {
                // ROM Bank Number
                let bank = (value & 0x1F) as usize;
                self.current_rom_bank = if bank == 0 { 1 } else { bank };
            }
            0x4000..=0x5FFF => {
                // RAM Bank Number or Upper ROM Bank bits
                if self.banking_mode == 0 {
                    // ROM banking mode
                    let upper_bits = ((value & 0x03) as usize) << 5;
                    self.current_rom_bank = (self.current_rom_bank & 0x1F) | upper_bits;
                } else {
                    // RAM banking mode
                    self.current_ram_bank = (value & 0x03) as usize;
                }
            }
            0x6000..=0x7FFF => {
                // Banking Mode Select
                self.banking_mode = value & 0x01;
            }
            0xA000..=0xBFFF => {
                // External RAM Write
                if self.ram_enabled && !self.ram_data.is_empty() {
                    let bank_offset = self.current_ram_bank * 0x2000;
                    let local_address = (address - 0xA000) as usize;
                    self.ram_data[bank_offset + local_address] = value;
                }
            }
            _ => {}
        }
    }

    /// Handle MBC2 writes (simplified)
    fn handle_mbc2_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x3FFF => {
                if address & 0x100 == 0 {
                    // RAM Enable
                    self.ram_enabled = (value & 0x0F) == 0x0A;
                } else {
                    // ROM Bank Number
                    let bank = (value & 0x0F) as usize;
                    self.current_rom_bank = if bank == 0 { 1 } else { bank };
                }
            }
            0xA000..=0xBFFF => {
                // MBC2 internal RAM (only lower 4 bits)
                if self.ram_enabled && !self.ram_data.is_empty() {
                    let local_address = (address - 0xA000) as usize;
                    if local_address < 0x200 {
                        self.ram_data[local_address] = value & 0x0F;
                    }
                }
            }
            _ => {}
        }
    }

    /// Handle MBC3 writes (simplified)
    fn handle_mbc3_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                // RAM and Timer Enable
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            0x2000..=0x3FFF => {
                // ROM Bank Number
                let bank = (value & 0x7F) as usize;
                self.current_rom_bank = if bank == 0 { 1 } else { bank };
            }
            0x4000..=0x5FFF => {
                // RAM Bank Number or RTC Register Select
                if value <= 0x03 {
                    self.current_ram_bank = value as usize;
                }
                // RTC registers (0x08-0x0C) would be handled here
            }
            0x6000..=0x7FFF => {
                // Latch Clock Data (RTC)
                // RTC latching would be handled here
            }
            0xA000..=0xBFFF => {
                // External RAM Write
                if self.ram_enabled && !self.ram_data.is_empty() {
                    let bank_offset = self.current_ram_bank * 0x2000;
                    let local_address = (address - 0xA000) as usize;
                    self.ram_data[bank_offset + local_address] = value;
                }
            }
            _ => {}
        }
    }

    /// Handle MBC5 writes (simplified)
    fn handle_mbc5_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                // RAM Enable
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            0x2000..=0x2FFF => {
                // ROM Bank Number (lower 8 bits)
                self.current_rom_bank = (self.current_rom_bank & 0x100) | (value as usize);
            }
            0x3000..=0x3FFF => {
                // ROM Bank Number (upper 1 bit)
                self.current_rom_bank = (self.current_rom_bank & 0xFF) | (((value & 0x01) as usize) << 8);
            }
            0x4000..=0x5FFF => {
                // RAM Bank Number
                self.current_ram_bank = (value & 0x0F) as usize;
            }
            0xA000..=0xBFFF => {
                // External RAM Write
                if self.ram_enabled && !self.ram_data.is_empty() {
                    let bank_offset = self.current_ram_bank * 0x2000;
                    let local_address = (address - 0xA000) as usize;
                    self.ram_data[bank_offset + local_address] = value;
                }
            }
            _ => {}
        }
    }

    /// Get cartridge information
    pub fn get_info(&self) -> String {
        format!(
            "Title: {}\nType: {:?}\nROM Size: {}KB\nRAM Size: {}KB",
            self.header.title,
            self.mbc_type,
            self.rom_data.len() / 1024,
            self.ram_data.len() / 1024
        )
    }
}