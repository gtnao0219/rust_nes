use std::{cell::RefCell, rc::Rc};

use crate::{interrupt::Interrupt, log, ram::RAM, Byte, Cycle, Word};

mod attribute;
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
const CRAM_SIZE: usize = 8192;
pub type CRAM = RAM<CRAM_SIZE>;

#[derive(Debug)]
pub struct RenderingData {
    pub background: background::Background,
    pub sprites: Vec<sprite::Sprite>,
}

pub const SCREEN_WIDTH: u16 = 256;
pub const SCREEN_HEIGHT: u16 = 240;

pub struct PPU {
    bus: PPUBus,
    registers: register::PPURegisters,
    oam: oam::OAM,
    cycle: Cycle,
    row: u16,
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
            row: 0,
            background: background::Background::default(),
            sprites: Vec::new(),
            interrupt,
        }
    }
    pub fn run(&mut self, cycle: Cycle) -> Option<RenderingData> {
        self.cycle += cycle;
        if self.cycle >= 341 {
            self.cycle -= 341;

            if self.row == 0 {
                self.background.lines.clear();
                self.build_sprites();
            }

            if self.has_sprite_hit() {
                self.registers.set_sprite_zero_hit();
            }

            if self.row < 240 && self.row % 8 == 0 {
                self.build_background_line();
            }
            if self.row == 240 {
                self.registers.set_vblank();
                if self.registers.has_vblank_nmi() {
                    self.interrupt.borrow_mut().set_nmi();
                }
            }
            if self.row == 261 {
                self.registers.clear_vblank();
                self.registers.clear_sprite_zero_hit();
                self.row = 0;
                self.interrupt.borrow_mut().clear_nmi();

                return Some(RenderingData {
                    background: self.background.clone(),
                    sprites: self.sprites.clone(),
                });
            }
            self.row += 1;
        }
        None
    }
    pub fn read_register(&mut self, addr: Word) -> Byte {
        match addr {
            0x2002 => {
                let data = self.registers.read_status();
                self.registers.clear_vblank();
                self.registers.clear_scroll_latch();
                self.registers.clear_address_latch();
                data
            }
            0x2004 => {
                // Write OAM data here. Writes will increment OAMADDR after the write
                // reads during vertical or forced blanking return the value from OAM at that address but do not increment.
                let oam_address = self.registers.oam_address();
                self.oam.read(oam_address)
            }
            0x2007 => {
                let address = self.registers.address();
                self.bus.read(address)
            }
            _ => {
                log(&format!("invalid ppu read address: {:04X}", addr));
                panic!();
            }
        }
    }
    pub fn write_register(&mut self, addr: Word, data: Byte) -> () {
        match addr {
            0x2000 => self.registers.write_ctrl(data),
            0x2001 => self.registers.write_mask(data),
            0x2003 => self.registers.write_oam_address(data),
            0x2004 => {
                let oam_address = self.registers.oam_address();
                self.oam.write(oam_address, data);
                self.registers.increment_oam_address();
            }
            0x2005 => self.registers.write_scroll(data),
            0x2006 => self.registers.write_address(data),
            0x2007 => {
                let address = self.registers.address();
                self.bus.write(address, data);
                self.registers.increment_address();
            }
            _ => {
                log(&format!("invalid ppu write address: {:04X}", addr));
                panic!();
            }
        }
    }
    pub fn transfer_sprite(&mut self, index: Byte, data: Byte) -> () {
        let oam_address = self.registers.oam_address();
        self.oam
            .write(((oam_address as u16 + index as u16) % 0x100) as u8, data);
    }

    fn build_sprites(&mut self) -> () {
        self.sprites.clear();
        for sprite_data in self.oam.iter() {
            let sprite = sprite::Sprite::new(&self, sprite_data);
            self.sprites.push(sprite);
        }
    }
    fn build_background_line(&mut self) -> () {
        let mut background_line: background::BackgroundLine = Vec::new();
        for column in 0..32 {
            let tile_x = self.tile_x(column);
            let tile_y = self.tile_y();
            let tile_id = self.fetch_tile_id(tile_x, tile_y);
            let tile = self.fetch_tile(tile_id, false);
            let attribute = self.fetch_attribute(tile_x, tile_y);
            let palette_id = attribute.palette_id(tile_x, tile_y);
            let palette_value = self.fetch_palette_value(palette_id, false);
            let background_cell = background::BackgroundCell {
                tile,
                palette_value,
                scroll_x: self.registers.real_scroll_x(),
                scroll_y: self.registers.real_scroll_y(),
                is_visible: self.registers.is_background_visible(),
            };
            background_line.push(background_cell);
        }
        self.background.lines.push(background_line);
    }

    fn has_sprite_hit(&self) -> bool {
        self.oam.head_y() == self.row as u8
            && self.registers.is_background_visible()
            && self.registers.is_sprite_visible()
    }

    // utils
    fn tile_y(&self) -> u16 {
        ((self.row + self.registers.real_scroll_y()) / 8) % 60
    }
    fn tile_x(&self, column: u8) -> u16 {
        ((column as u16 * 8 + self.registers.real_scroll_x()) / 8) % 64
    }
    fn name_table_address(&self, tile_x: u16, tile_y: u16) -> Word {
        let offset = self.name_table_offset(tile_x, tile_y);
        offset + tile_x % 32 + tile_y % 30 * 32
    }
    fn name_table_offset(&self, tile_x: u16, tile_y: u16) -> Word {
        0x2000 + tile_x / 32 * 0x400 + tile_y / 30 * 0x800
    }
    fn attribute_table_address(&self, tile_x: u16, tile_y: u16) -> Word {
        let offset = self.attribute_table_offset(tile_x, tile_y);
        offset + tile_x % 32 / 4 + tile_y % 30 / 4 * 8
    }
    fn attribute_table_offset(&self, tile_x: u16, tile_y: u16) -> Word {
        self.name_table_offset(tile_x, tile_y) + 0x3C0
    }
    fn pattern_table_address(&self, tile_id: u8, is_sprite: bool) -> Word {
        let offset = self.pattern_table_offset(is_sprite);
        offset + tile_id as Word * 16 as Word
    }
    fn pattern_table_offset(&self, is_sprite: bool) -> Word {
        if is_sprite {
            self.registers.sprite_table_offset()
        } else {
            self.registers.background_table_offset()
        }
    }
    fn fetch_tile_id(&self, tile_x: u16, tile_y: u16) -> u8 {
        self.bus.read(self.name_table_address(tile_x, tile_y))
    }
    fn fetch_tile(&self, tile_id: u8, is_sprite: bool) -> tile::Tile {
        let address = self.pattern_table_address(tile_id, is_sprite);
        let mut tile_data = [0; 16];
        for i in 0..16 {
            tile_data[i as usize] = self.bus.read(address + i as Word);
        }
        tile::Tile::new(tile_data)
    }
    fn fetch_attribute(&self, tile_x: u16, tile_y: u16) -> attribute::Attribute {
        let address = self.attribute_table_address(tile_x, tile_y);
        let data = self.bus.read(address);
        attribute::Attribute::new(data)
    }
    fn fetch_palette_value(&self, palette_id: u8, is_sprite: bool) -> [u8; 4] {
        let offset = if is_sprite { 0x3F10 } else { 0x3F00 };
        let address = offset + palette_id as Word * 4;
        let mut palette_value = [0; 4];
        for i in 0..4 {
            palette_value[i] = self.bus.read(address + i as Word);
        }
        palette_value
    }
}
