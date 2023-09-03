#[derive(Debug, Default)]
pub struct Palette {
    data: [u8; 32],
}

impl Palette {
    pub fn read(&self, addr: u8) -> u8 {
        self.data[Self::real_read_addr(addr) as usize]
    }
    pub fn write(&mut self, addr: u8, data: u8) -> () {
        self.data[Self::real_write_addr(addr) as usize] = data;
    }
    fn real_read_addr(addr: u8) -> u8 {
        if Self::is_background_mirror(addr) {
            0x00
        } else if Self::is_sprite_mirror(addr) {
            addr - 0x10
        } else {
            addr
        }
    }
    fn real_write_addr(addr: u8) -> u8 {
        if Self::is_sprite_mirror(addr) {
            addr - 0x10
        } else {
            addr
        }
    }
    fn is_background_mirror(addr: u8) -> bool {
        addr == 0x04 || addr == 0x08 || addr == 0x0c
    }
    fn is_sprite_mirror(addr: u8) -> bool {
        addr == 0x10 || addr == 0x14 || addr == 0x18 || addr == 0x1c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let mut palette = Palette::default();

        palette.write(0x00, 0x01);
        assert_eq!(palette.read(0x00), 0x01);
        assert_eq!(palette.read(0x04), 0x01);
        assert_eq!(palette.read(0x08), 0x01);
        assert_eq!(palette.read(0x0c), 0x01);
        assert_eq!(palette.read(0x10), 0x01);
        palette.write(0x10, 0x02);
        assert_eq!(palette.read(0x00), 0x02);
        assert_eq!(palette.read(0x04), 0x02);
        assert_eq!(palette.read(0x08), 0x02);
        assert_eq!(palette.read(0x0c), 0x02);
        assert_eq!(palette.read(0x10), 0x02);

        palette.write(0x04, 0x03);
        assert_eq!(palette.read(0x14), 0x03);
        palette.write(0x14, 0x04);
        assert_eq!(palette.read(0x14), 0x04);

        palette.write(0x08, 0x05);
        assert_eq!(palette.read(0x18), 0x05);
        palette.write(0x18, 0x06);
        assert_eq!(palette.read(0x18), 0x06);

        palette.write(0x0c, 0x07);
        assert_eq!(palette.read(0x1c), 0x07);
        palette.write(0x1c, 0x08);
        assert_eq!(palette.read(0x1c), 0x08);

        palette.write(0x01, 0x09);
        palette.write(0x02, 0x0a);
        palette.write(0x03, 0x0b);
        palette.write(0x05, 0x0c);
        palette.write(0x06, 0x0d);
        palette.write(0x07, 0x0e);
        palette.write(0x09, 0x0f);
        palette.write(0x0a, 0x10);
        palette.write(0x0b, 0x11);
        palette.write(0x0d, 0x12);
        palette.write(0x0e, 0x13);
        palette.write(0x0f, 0x14);
        palette.write(0x11, 0x15);
        palette.write(0x12, 0x16);
        palette.write(0x13, 0x17);
        palette.write(0x15, 0x18);
        palette.write(0x16, 0x19);
        palette.write(0x17, 0x1a);
        palette.write(0x19, 0x1b);
        palette.write(0x1a, 0x1c);
        palette.write(0x1b, 0x1d);
        palette.write(0x1d, 0x1e);
        palette.write(0x1e, 0x1f);
        palette.write(0x1f, 0x20);
        assert_eq!(palette.read(0x01), 0x09);
        assert_eq!(palette.read(0x02), 0x0a);
        assert_eq!(palette.read(0x03), 0x0b);
        assert_eq!(palette.read(0x05), 0x0c);
        assert_eq!(palette.read(0x06), 0x0d);
        assert_eq!(palette.read(0x07), 0x0e);
        assert_eq!(palette.read(0x09), 0x0f);
        assert_eq!(palette.read(0x0a), 0x10);
        assert_eq!(palette.read(0x0b), 0x11);
        assert_eq!(palette.read(0x0d), 0x12);
        assert_eq!(palette.read(0x0e), 0x13);
        assert_eq!(palette.read(0x0f), 0x14);
        assert_eq!(palette.read(0x11), 0x15);
        assert_eq!(palette.read(0x12), 0x16);
        assert_eq!(palette.read(0x13), 0x17);
        assert_eq!(palette.read(0x15), 0x18);
        assert_eq!(palette.read(0x16), 0x19);
        assert_eq!(palette.read(0x17), 0x1a);
        assert_eq!(palette.read(0x19), 0x1b);
        assert_eq!(palette.read(0x1a), 0x1c);
        assert_eq!(palette.read(0x1b), 0x1d);
        assert_eq!(palette.read(0x1d), 0x1e);
        assert_eq!(palette.read(0x1e), 0x1f);
        assert_eq!(palette.read(0x1f), 0x20);
    }
}
