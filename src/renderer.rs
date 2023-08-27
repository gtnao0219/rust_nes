use crate::{
    ppu::{Background, BackgroundCell, RenderingData, Sprite, SCREEN_HEIGHT, SCREEN_WIDTH},
    render_canvas,
};

pub struct Renderer {
    result: [u8; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize * 4],
}
impl Renderer {
    pub fn new() -> Self {
        Renderer {
            result: [0xFF; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize * 4],
        }
    }
    pub fn render(&mut self, rendering_data: RenderingData) {
        self.set_background(&rendering_data.background);
        self.set_sprites(&rendering_data.sprites);
        render_canvas(&self.result);
    }
    fn set_background(&mut self, background: &Background) {
        for tile_row in 0..30 {
            for tile_column in 0..32 {
                let cell = &background.lines[tile_row][tile_column];
                self.set_cell(cell, tile_row, tile_column);
            }
        }
    }
    fn set_cell(&mut self, cell: &BackgroundCell, tile_row: usize, tile_column: usize) {
        if !cell.is_visible {
            return;
        }
        for offset_y in 0..8 {
            for offset_x in 0..8 {
                let y = tile_row as isize * 8 + offset_y - (cell.scroll_y % 8) as isize;
                let x = tile_column as isize * 8 + offset_x - (cell.scroll_x % 8) as isize;
                if 0 <= x && x < 256 && 0 <= y && y < 240 {
                    let palette_offset = cell.tile.palette_offset(offset_x as u8, offset_y as u8);
                    let color_id = cell.palette_value[palette_offset as usize];
                    let color = COLORS[color_id as usize];
                    let index = (y * 256 + x) * 4;
                    self.result[index as usize] = color.0;
                    self.result[index as usize + 1] = color.1;
                    self.result[index as usize + 2] = color.2;
                    if x < 8 {
                        self.result[index as usize + 3] = 0;
                    }
                }
            }
        }
    }
    fn set_sprites(&mut self, sprites: &Vec<Sprite>) {
        for sprite in sprites.iter() {
            self.set_sprite(&sprite);
        }
    }
    fn set_sprite(&mut self, sprite: &Sprite) {
        if sprite.attribute.is_low_priority {
            return;
        }
        for original_offset_y in 0..8 {
            for original_offset_x in 0..8 {
                let offset_y = if sprite.attribute.is_flip_vertical {
                    7 - original_offset_y
                } else {
                    original_offset_y
                };
                let offset_x = if sprite.attribute.is_flip_horizontal {
                    7 - original_offset_x
                } else {
                    original_offset_x
                };
                let y = sprite.y as isize + original_offset_y;
                let x = sprite.x as isize + original_offset_x;
                if y < 0 || 240 <= y || x < 0 || 256 <= x {
                    continue;
                }
                let palette_offset = sprite.tile.palette_offset(offset_x as u8, offset_y as u8);
                let color_id = sprite.attribute.palette_value[palette_offset as usize];
                let color = COLORS[color_id as usize];
                let index = (y * 256 + x) * 4;
                if palette_offset == 0 {
                    continue;
                }
                self.result[index as usize] = color.0;
                self.result[index as usize + 1] = color.1;
                self.result[index as usize + 2] = color.2;
                if x < 8 {
                    self.result[index as usize + 3] = 0;
                }
            }
        }
    }
}

pub type Color = (u8, u8, u8);
pub const COLORS: [Color; 64] = [
    (0x80, 0x80, 0x80),
    (0x00, 0x3D, 0xA6),
    (0x00, 0x12, 0xB0),
    (0x44, 0x00, 0x96),
    (0xA1, 0x00, 0x5E),
    (0xC7, 0x00, 0x28),
    (0xBA, 0x06, 0x00),
    (0x8C, 0x17, 0x00),
    (0x5C, 0x2F, 0x00),
    (0x10, 0x45, 0x00),
    (0x05, 0x4A, 0x00),
    (0x00, 0x47, 0x2E),
    (0x00, 0x41, 0x66),
    (0x00, 0x00, 0x00),
    (0x05, 0x05, 0x05),
    (0x05, 0x05, 0x05),
    (0xC7, 0xC7, 0xC7),
    (0x00, 0x77, 0xFF),
    (0x21, 0x55, 0xFF),
    (0x82, 0x37, 0xFA),
    (0xEB, 0x2F, 0xB5),
    (0xFF, 0x29, 0x50),
    (0xFF, 0x22, 0x00),
    (0xD6, 0x32, 0x00),
    (0xC4, 0x62, 0x00),
    (0x35, 0x80, 0x00),
    (0x05, 0x8F, 0x00),
    (0x00, 0x8A, 0x55),
    (0x00, 0x99, 0xCC),
    (0x21, 0x21, 0x21),
    (0x09, 0x09, 0x09),
    (0x09, 0x09, 0x09),
    (0xFF, 0xFF, 0xFF),
    (0x0F, 0xD7, 0xFF),
    (0x69, 0xA2, 0xFF),
    (0xD4, 0x80, 0xFF),
    (0xFF, 0x45, 0xF3),
    (0xFF, 0x61, 0x8B),
    (0xFF, 0x88, 0x33),
    (0xFF, 0x9C, 0x12),
    (0xFA, 0xBC, 0x20),
    (0x9F, 0xE3, 0x0E),
    (0x2B, 0xF0, 0x35),
    (0x0C, 0xF0, 0xA4),
    (0x05, 0xFB, 0xFF),
    (0x5E, 0x5E, 0x5E),
    (0x0D, 0x0D, 0x0D),
    (0x0D, 0x0D, 0x0D),
    (0xFF, 0xFF, 0xFF),
    (0xA6, 0xFC, 0xFF),
    (0xB3, 0xEC, 0xFF),
    (0xDA, 0xAB, 0xEB),
    (0xFF, 0xA8, 0xF9),
    (0xFF, 0xAB, 0xB3),
    (0xFF, 0xD2, 0xB0),
    (0xFF, 0xEF, 0xA6),
    (0xFF, 0xF7, 0x9C),
    (0xD7, 0xE8, 0x95),
    (0xA6, 0xED, 0xAF),
    (0xA2, 0xF2, 0xDA),
    (0x99, 0xFF, 0xFC),
    (0xDD, 0xDD, 0xDD),
    (0x11, 0x11, 0x11),
    (0x11, 0x11, 0x11),
];
