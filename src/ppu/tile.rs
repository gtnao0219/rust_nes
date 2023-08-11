use super::{palette::PaletteId, SCREEN_HEIGHT, SCREEN_WIDTH};

pub const TILE_BYTE_SIZE: usize = 16;
pub const TILE_WIDTH: u16 = 8;
pub const TILE_HEIGHT: u16 = 8;
pub const SCREEN_TILE_WIDTH: u16 = SCREEN_WIDTH / TILE_WIDTH;
pub const SCREEN_TILE_HEIGHT: u16 = SCREEN_HEIGHT / TILE_HEIGHT;

pub type TileId = u8;
pub type TileData = [[PaletteId; TILE_WIDTH as usize]; TILE_HEIGHT as usize];

#[derive(Debug, Clone)]
pub struct Tile {
    pub data: TileData,
}

impl Tile {
    pub fn new(raw_data: [u8; TILE_BYTE_SIZE]) -> Self {
        let mut data = [[0; TILE_WIDTH as usize]; TILE_HEIGHT as usize];
        for y in 0..TILE_HEIGHT {
            let right = raw_data[y as usize];
            let left = raw_data[y as usize + TILE_BYTE_SIZE / 2];
            for x in 0..TILE_WIDTH {
                let right_bit = (right >> (7 - x)) & 0x1;
                let left_bit = (left >> (7 - x)) & 0x1;
                let pallet_id = (left_bit << 1) | right_bit;
                data[y as usize][x as usize] = pallet_id;
            }
        }
        Tile { data }
    }
}
