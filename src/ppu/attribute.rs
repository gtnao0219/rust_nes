#[derive(Debug)]
pub struct Attribute {
    data: [u8; 4],
}

impl Attribute {
    pub fn new(raw_data: u8) -> Self {
        let mut data = [0; 4];
        for i in 0..4 {
            data[i as usize] = (raw_data >> (i * 2)) & 0b11;
        }
        Attribute { data }
    }
    pub fn palette_id(&self, tile_x: u16, tile_y: u16) -> u8 {
        self.data[Self::block_number(tile_x, tile_y) as usize]
    }
    fn block_number(tile_x: u16, tile_y: u16) -> u8 {
        (tile_x % 4 / 2 + tile_y % 4 / 2 * 2) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_id() {
        let attr = Attribute::new(0b1110_0100);
        assert_eq!(attr.palette_id(0, 0), 0);
        assert_eq!(attr.palette_id(1, 0), 0);
        assert_eq!(attr.palette_id(2, 0), 1);
        assert_eq!(attr.palette_id(3, 0), 1);
        assert_eq!(attr.palette_id(0, 1), 0);
        assert_eq!(attr.palette_id(1, 1), 0);
        assert_eq!(attr.palette_id(2, 1), 1);
        assert_eq!(attr.palette_id(3, 1), 1);
        assert_eq!(attr.palette_id(0, 2), 2);
        assert_eq!(attr.palette_id(1, 2), 2);
        assert_eq!(attr.palette_id(2, 2), 3);
        assert_eq!(attr.palette_id(3, 2), 3);
        assert_eq!(attr.palette_id(0, 3), 2);
        assert_eq!(attr.palette_id(1, 3), 2);
        assert_eq!(attr.palette_id(2, 3), 3);
        assert_eq!(attr.palette_id(3, 3), 3);
    }

    #[test]
    // over 4x4
    fn test_palette_id_over_4_4() {
        let attr = Attribute::new(0b1110_0100);
        assert_eq!(attr.palette_id(4, 4), 0);
        assert_eq!(attr.palette_id(5, 4), 0);
        assert_eq!(attr.palette_id(6, 4), 1);
        assert_eq!(attr.palette_id(7, 4), 1);
        assert_eq!(attr.palette_id(4, 5), 0);
        assert_eq!(attr.palette_id(5, 5), 0);
        assert_eq!(attr.palette_id(6, 5), 1);
        assert_eq!(attr.palette_id(7, 5), 1);
        assert_eq!(attr.palette_id(4, 6), 2);
        assert_eq!(attr.palette_id(5, 6), 2);
        assert_eq!(attr.palette_id(6, 6), 3);
        assert_eq!(attr.palette_id(7, 6), 3);
        assert_eq!(attr.palette_id(4, 7), 2);
        assert_eq!(attr.palette_id(5, 7), 2);
        assert_eq!(attr.palette_id(6, 7), 3);
        assert_eq!(attr.palette_id(7, 7), 3);
    }

}
