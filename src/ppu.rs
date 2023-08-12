use std::{cell::RefCell, rc::Rc};

use crate::{interrupt::Interrupt, ram::RAM, Byte, Cycle, Word, log};

mod background;
mod bus;
mod oam;
mod palette;
mod register;
mod sprite;
mod tile;
pub use background::{Background, BackgroundCell, BackgroundLine};
pub use bus::PPUBus;
pub use palette::Palette;
pub use sprite::Sprite;
pub use tile::Tile;

const VRAM_SIZE: usize = 2048;
pub type VRAM = RAM<VRAM_SIZE>;

#[derive(Debug)]
pub struct RenderingData {
    pub background: background::Background,
    pub sprites: Vec<sprite::Sprite>,
}

pub const SCREEN_WIDTH: u16 = 256;
pub const SCREEN_HEIGHT: u16 = 240;

pub const TILE_WIDTH_OF_BLOCK: u16 = 2;
pub const TILE_HEIGHT_OF_BLOCK: u16 = 2;
pub const SCREEN_BLOCK_WIDTH: u16 = tile::SCREEN_TILE_WIDTH / TILE_WIDTH_OF_BLOCK;
pub const SCREEN_BLOCK_HEIGHT: u16 = tile::SCREEN_TILE_HEIGHT / TILE_HEIGHT_OF_BLOCK;

#[derive(Debug)]
pub struct PPU {
    bus: PPUBus,
    registers: register::PPURegisters,
    oam: oam::OAM,
    cycle: Cycle,
    line: u16,
    background: background::Background,
    sprites: Vec<sprite::Sprite>,
    interrupt: Rc<RefCell<Interrupt>>,
}

impl PPU {
    pub fn new(bus: PPUBus, interrupt: Rc<RefCell<Interrupt>>) -> Self {
        PPU {
            bus,
            registers: register::PPURegisters::default(),
            oam: oam::OAM::default(),
            cycle: 0,
            line: 0,
            background: background::Background::default(),
            sprites: Vec::new(),
            interrupt,
        }
    }
    pub fn run(&mut self, cycle: Cycle) -> Option<RenderingData> {
        self.cycle += cycle;
        if self.line == 0 {
            self.background.lines.clear();
            self.build_sprites();
        }
        if self.cycle >= 341 {
            self.cycle -= 341;
            self.line += 1;

            if self.has_sprite_hit() {
                self.registers.set_sprite_zero_hit();
            }

            // TODO: check scrollY
            if self.line <= 240 && (self.line - 1) % 8 == 0 {
                self.build_background_line();
            }
            if self.line == 241 {
                self.registers.set_vblank();
                if self.registers.has_vblank_nmi() {
                    self.interrupt.borrow_mut().set_nmi();
                }
            }
            if self.line == 262 {
                self.registers.clear_vblank();
                self.registers.clear_sprite_zero_hit();
                self.line = 0;
                self.interrupt.borrow_mut().clear_nmi();

                log(&format!("palette: {:?}", self.bus.palette));

                return Some(RenderingData {
                    background: self.background.clone(),
                    sprites: self.sprites.clone(),
                });
            }
        }
        None
    }

    pub fn read_register(&self, addr: Word) -> u8 {
        match addr {
            0x2002 => self.registers.read_status(),
            0x2007 => {
                let address = self.registers.get_address();
                self.bus.read(address)
            }
            _ => panic!("invalid ppu read address: {:04X}", addr),
        }
    }
    pub fn write_register(&mut self, addr: Word, data: u8) {
        match addr {
            0x2000 => self.registers.write_ctrl(data),
            0x2001 => self.registers.write_mask(data),
            0x2003 => self.registers.write_oam_address(data),
            0x2004 => {
                let address = self.registers.get_oam_address();
                self.oam.write(address, data);
            }
            0x2005 => self.registers.write_scroll(data),
            0x2006 => self.registers.write_address(data),
            0x2007 => {
                let address = self.registers.get_address();
                self.bus.write(address, data);
                self.registers.increment_address();
            }
            _ => panic!("invjlid ppu write address: {:04X}", addr),
        }
    }

    pub fn transfer_sprite(&mut self, index: Byte, data: Byte) {
        let address = self.registers.get_oam_address();
        self.oam
            .write(((address as u16 + index as u16) % 0x100) as u8, data);
    }

    fn build_sprites(&mut self) {
        self.sprites.clear();
        for sprite_data in self.oam.iter() {
            let sprite = sprite::Sprite::new(&self, sprite_data);
            self.sprites.push(sprite);
        }
    }
    fn build_background_line(&mut self) {
        let mut background_line: background::BackgroundLine = Vec::new();
        for tile_x in 0..tile::SCREEN_TILE_WIDTH {
            let name_table_address = self.get_name_table_address(tile_x);
            let tile_id = self.bus.read(name_table_address);
            let tile = self.get_tile(tile_id, false);
            let attribute_table_address = self.get_attribute_table_address(tile_x);
            let attribute = self.bus.read(attribute_table_address);
            let block_number = self.get_block_number(tile_x);
            let palette_id = self.get_palette_id(attribute, block_number);
            let palette_value = self.get_palette_value(palette_id, false);
            let background_cell = background::BackgroundCell {
                tile,
                palette_value,
            };
            background_line.push(background_cell);
        }
        self.background.lines.push(background_line);
    }

    fn get_name_table_address(&self, tile_x: u16) -> Word {
        let offset = 0x2000 + self.registers.name_table_id() as Word * 0x400;
        offset as Word
            + ((self.line - 1) / 8) as Word * tile::SCREEN_TILE_WIDTH as Word
            + tile_x as Word
    }
    fn get_tile(&self, tile_id: tile::TileId, is_sprite: bool) -> tile::Tile {
        let offset = if is_sprite {
            self.registers.sprite_table_offset()
        } else {
            self.registers.background_table_offset()
        };
        let tile_addr = offset as Word + tile_id as Word * tile::TILE_BYTE_SIZE as Word;
        let mut tile_data = [0; tile::TILE_BYTE_SIZE as usize];
        for i in 0..tile::TILE_BYTE_SIZE {
            tile_data[i as usize] = self.bus.read(tile_addr + i as Word);
        }
        tile::Tile::new(tile_data)
    }
    fn get_attribute_table_address(&self, tile_x: u16) -> Word {
        let offset = 0x23C0 + self.registers.name_table_id() as Word * 0x400;
        offset as Word + self.get_attribute_index(tile_x) as Word
    }
    fn get_attribute_index(&self, tile_x: u16) -> u16 {
        let attribute_x = tile_x / 4;
        let attribute_y = (self.line - 1) / 8 / 4;
        attribute_x + attribute_y * 8
    }
    fn get_block_number(&self, tile_x: u16) -> u16 {
        tile_x % 4 / 2 + ((self.line - 1) / 8) % 4 / 2 * 2
    }
    fn get_palette_id(&self, attribute: u8, block_number: u16) -> palette::PaletteId {
        match block_number {
            0 => attribute & 0b00000011,
            1 => (attribute & 0b00001100) >> 2,
            2 => (attribute & 0b00110000) >> 4,
            3 => (attribute & 0b11000000) >> 6,
            _ => panic!("invalid block number: {}", block_number),
        }
    }
    fn get_palette_value(&self, palette_id: palette::PaletteId, is_sprite: bool) -> [u8; 4] {
        let offset = if is_sprite { 0x3F10 } else { 0x3F00 };
        let addr = offset as Word + palette_id as Word;
        let mut palette_value = [0; 4];
        for i in 0..4 {
            palette_value[i] = self.bus.read(addr + i as Word);
        }
        palette_value
    }

    fn has_sprite_hit(&self) -> bool {
        self.oam.head_y() == (self.line - 1)
            && self.registers.is_background_visible()
            && self.registers.is_sprite_visible()
    }
}
