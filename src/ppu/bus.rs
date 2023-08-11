use crate::{rom::ROM, Byte, Word};

use super::{VRAM, palette::Palette};

#[derive(Debug)]
pub struct PPUBus {
    character_rom: ROM,
    vram: VRAM,
    pub palette: Palette,
}

impl PPUBus {
    pub fn new(character_rom: ROM) -> Self {
        PPUBus {
            character_rom,
            vram: VRAM::default(),
            palette: Palette::default(),
        }
    }
    pub fn read(&self, addr: Word) -> Byte {
        match addr {
            0x0000..=0x1FFF => self.character_rom.read(addr),
            0x2000..=0x3EFF => self.vram.read((addr - 0x2000) % 0x0800),
            0x3F00..=0x3FFF => self.palette.read(((addr - 0x3F00) % 0x0020) as Byte),
            _ => panic!("invalid ppu bus address: {:04X}", addr),
        }
    }
    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x2000..=0x3EFF => self.vram.write((addr - 0x2000) % 0x0800, data),
            0x3F00..=0x3FFF => self.palette.write(((addr - 0x3F00) % 0x0020) as Byte, data),
            _ => panic!("invalid ppu bus address: {:04X}", addr),
        }
    }
}
