use crate::{log, rom::ROM, Byte, Word};

use super::{palette::Palette, CRAM, VRAM};

pub struct PPUBus {
    cram: CRAM,
    vram: VRAM,
    palette: Palette,
    is_horizontal_mirroring: bool,
}

impl PPUBus {
    pub fn new(character_rom: ROM, is_horizontal_mirroring: bool) -> Self {
        let mut cram = CRAM::default();
        for i in 0..character_rom.size() {
            cram.write(i as Word, character_rom.read(i as Word));
        }
        PPUBus {
            cram,
            vram: VRAM::default(),
            palette: Palette::default(),
            is_horizontal_mirroring,
        }
    }
    pub fn read(&self, addr: Word) -> Byte {
        match addr {
            0x0000..=0x1FFF => self.cram.read(addr),
            0x2000..=0x23FF => self.vram.read(addr - 0x2000),
            0x2400..=0x27FF => {
                let addr = addr - 0x2400;
                if self.is_horizontal_mirroring {
                    self.vram.read(addr)
                } else {
                    self.vram.read(addr + 0x0400)
                }
            }
            0x2800..=0x2BFF => {
                let addr = addr - 0x2800;
                if self.is_horizontal_mirroring {
                    self.vram.read(addr + 0x0400)
                } else {
                    self.vram.read(addr)
                }
            }
            0x2C00..=0x2FFF => {
                let addr = addr - 0x2C00;
                self.vram.read(addr + 0x0400)
            }
            0x3F00..=0x3FFF => self.palette.read(((addr - 0x3F00) % 0x0020) as Byte),
            _ => {
                log(&format!("invalid ppu bus address: {:04X}", addr));
                panic!();
            }
        }
    }
    pub fn write(&mut self, addr: u16, data: u8) -> () {
        match addr {
            0x0000..=0x1FFF => self.cram.write(addr, data),
            0x2000..=0x23FF => self.vram.write(addr - 0x2000, data),
            0x2400..=0x27FF => {
                let addr = addr - 0x2400;
                if self.is_horizontal_mirroring {
                    self.vram.write(addr, data)
                } else {
                    self.vram.write(addr + 0x0400, data)
                }
            }
            0x2800..=0x2BFF => {
                let addr = addr - 0x2800;
                if self.is_horizontal_mirroring {
                    self.vram.write(addr + 0x0400, data)
                } else {
                    self.vram.write(addr, data)
                }
            }
            0x2C00..=0x2FFF => {
                let addr = addr - 0x2C00;
                self.vram.write(addr + 0x0400, data)
            }
            0x3F00..=0x3FFF => self.palette.write(((addr - 0x3F00) % 0x0020) as Byte, data),
            _ => {
                log(&format!("invalid ppu bus address: {:04X}", addr));
                panic!();
            }
        }
    }
}
