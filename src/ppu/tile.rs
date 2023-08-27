#[derive(Debug, Clone)]
pub struct Tile {
    data: [[u8; 8]; 8],
}

impl Tile {
    pub fn new(raw_data: [u8; 16]) -> Self {
        let mut data = [[0; 8]; 8];
        for offset_y in 0..8 {
            let right = raw_data[offset_y];
            let left = raw_data[offset_y + 8];
            for offset_x in 0..8 {
                let right_bit = (right >> (7 - offset_x)) & 0b1;
                let left_bit = (left >> (7 - offset_x)) & 0b1;
                let palette_offset = (left_bit << 1) | right_bit;
                data[offset_y][offset_x] = palette_offset;
            }
        }
        Tile { data }
    }
    pub fn palette_offset(&self, offset_x: u8, offset_y: u8) -> u8 {
        self.data[offset_y as usize][offset_x as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_offset() {
        // mario
        let raw_data = [
            0b1110_0000,
            0b1100_0000,
            0b1000_0000,
            0b1111_1100,
            0b1000_0000,
            0b1100_0000,
            0b0000_0000,
            0b0010_0000,
            0b0000_0000,
            0b0010_0000,
            0b0110_0000,
            0b0000_0000,
            0b1111_0000,
            0b1111_1100,
            0b1111_1110,
            0b1111_1110,
        ];
        let tile = Tile::new(raw_data);

        let calculated_data = [
            [1, 1, 1, 0, 0, 0, 0, 0],
            [1, 1, 2, 0, 0, 0, 0, 0],
            [1, 2, 2, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 0, 0],
            [3, 2, 2, 2, 0, 0, 0, 0],
            [3, 3, 2, 2, 2, 2, 0, 0],
            [2, 2, 2, 2, 2, 2, 2, 0],
            [2, 2, 3, 2, 2, 2, 2, 0],
        ];
        for offset_y in 0..8 {
            for offset_x in 0..8 {
                assert_eq!(
                    tile.palette_offset(offset_x, offset_y),
                    calculated_data[offset_y as usize][offset_x as usize]
                );
            }
        }
    }
}
