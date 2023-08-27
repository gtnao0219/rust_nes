use crate::{Byte, Word};

use super::SCREEN_WIDTH;

#[derive(Debug)]
pub struct PPURegisters {
    ctrl: Byte,
    mask: Byte,
    status: Byte,
    oam_address: Byte,
    scroll_x: Byte,
    scroll_y: Byte,
    address: Word,

    is_first_scroll_write: bool,
    is_first_address_write: bool,
}
impl Default for PPURegisters {
    fn default() -> Self {
        PPURegisters {
            ctrl: 0,
            mask: 0,
            status: 0,
            oam_address: 0,
            scroll_x: 0,
            scroll_y: 0,
            address: 0,

            is_first_scroll_write: true,
            is_first_address_write: true,
        }
    }
}

impl PPURegisters {
    pub fn write_ctrl(&mut self, data: Byte) {
        self.ctrl = data;
    }
    pub fn write_mask(&mut self, data: Byte) {
        self.mask = data;
    }
    pub fn read_status(&self) -> Byte {
        self.status
    }
    pub fn write_oam_address(&mut self, data: Byte) {
        self.oam_address = data;
    }
    pub fn write_scroll(&mut self, data: Byte) {
        if self.is_first_scroll_write {
            self.scroll_x = data;
        } else {
            self.scroll_y = data;
        }
        self.is_first_scroll_write = !self.is_first_scroll_write;
    }
    pub fn clear_scroll_latch(&mut self) {
        self.is_first_scroll_write = true;
    }
    pub fn write_address(&mut self, data: Byte) {
        if self.is_first_address_write {
            self.address = (data as Word) << 8;
        } else {
            self.address |= data as Word;
        }
        self.is_first_address_write = !self.is_first_address_write;
    }
    pub fn clear_address_latch(&mut self) {
        self.is_first_address_write = true;
    }

    pub fn oam_address(&self) -> Byte {
        self.oam_address
    }
    pub fn increment_oam_address(&mut self) {
        self.oam_address += 1;
    }
    pub fn address(&self) -> Word {
        self.address
    }
    pub fn increment_address(&mut self) {
        self.address += self.vram_address_increment() as Word;
    }

    pub fn name_table_id(&self) -> u8 {
        self.ctrl & 0b11
    }
    pub fn vram_address_increment(&self) -> u8 {
        if self.ctrl & 0b100 == 0 {
            1
        } else {
            32
        }
    }
    pub fn sprite_table_offset(&self) -> Word {
        if self.ctrl & 0b1000 == 0 {
            0x0000
        } else {
            0x1000
        }
    }
    pub fn background_table_offset(&self) -> Word {
        if self.ctrl & 0b10000 == 0 {
            0x0000
        } else {
            0x1000
        }
    }
    pub fn has_vblank_nmi(&self) -> bool {
        self.ctrl & 0b10000000 != 0
    }
    pub fn is_background_visible(&self) -> bool {
        self.mask & 0b1000 != 0
    }
    pub fn is_sprite_visible(&self) -> bool {
        self.mask & 0b10000 != 0
    }
    pub fn is_sprite_overflow(&self) -> bool {
        self.status & 0b100000 != 0
    }
    pub fn set_sprite_overflow(&mut self) {
        self.status |= 0b100000;
    }
    pub fn clear_sprite_overflow(&mut self) {
        self.status &= 0b011111;
    }
    pub fn is_sprite_zero_hit(&self) -> bool {
        self.status & 0b1000000 != 0
    }
    pub fn set_sprite_zero_hit(&mut self) {
        self.status |= 0b1000000;
    }
    pub fn clear_sprite_zero_hit(&mut self) {
        self.status &= 0b01111111;
    }
    pub fn is_vblank(&self) -> bool {
        self.status & 0b10000000 != 0
    }
    pub fn set_vblank(&mut self) {
        self.status |= 0b10000000;
    }
    pub fn clear_vblank(&mut self) {
        self.status &= 0b01111111;
    }

    pub fn scroll_x(&self) -> Byte {
        self.scroll_x
    }
    pub fn scroll_y(&self) -> Byte {
        self.scroll_y
    }
    pub fn real_scroll_x(&self) -> u16 {
        self.scroll_x as u16 + (self.name_table_id() as u16 % 2 * SCREEN_WIDTH)
    }
    pub fn real_scroll_y(&self) -> u16 {
        self.scroll_y as u16 + (self.name_table_id() as u16 / 2 * SCREEN_WIDTH)
    }
}
