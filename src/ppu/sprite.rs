use super::{oam::OAMEntry, tile::Tile, PPU};

#[derive(Debug, Clone)]
pub struct Sprite {
    pub x: u8,
    pub y: u8,
    pub tile: Tile,
    pub attribute: SpriteAttribute,
}
#[derive(Debug, Clone)]
pub struct SpriteAttribute {
    pub palette_value: [u8; 4],
    pub is_low_priority: bool,
    pub is_flip_horizontal: bool,
    pub is_flip_vertical: bool,
}

impl Sprite {
    pub fn new(ppu: &PPU, oam_entry: OAMEntry) -> Self {
        let tile_id = oam_entry[1];
        let tile = ppu.get_tile(tile_id, true);
        Sprite {
            x: oam_entry[3],
            y: oam_entry[0],
            tile,
            attribute: SpriteAttribute::new(ppu, oam_entry[2]),
        }
    }
}

impl SpriteAttribute {
    pub fn new(ppu: &PPU, data: u8) -> Self {
        let palette_id = data & 0x03;
        SpriteAttribute {
            palette_value: ppu.get_palette_value(palette_id, true),
            is_low_priority: data & 0x20 != 0,
            is_flip_horizontal: data & 0x40 != 0,
            is_flip_vertical: data & 0x80 != 0,
        }
    }
}
