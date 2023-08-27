use crate::log;

#[derive(Debug, Default)]
pub struct Palette {
    data: [u8; 32],
}

impl Palette {
    pub fn read(&self, addr: u8) -> u8 {
        self.data[Self::real_addr(addr) as usize]
    }
    pub fn write(&mut self, addr: u8, data: u8) -> () {
        self.data[Self::real_addr(addr) as usize] = data;
    }
    fn real_addr(addr: u8) -> u8 {
        if addr >= 0x20 {
            log(&format!("Palette out of range: {:02X}", addr));
            panic!();
        }
        if Self::is_background_mirror(addr) {
            0x00
        } else if Self::is_sprite_mirror(addr) {
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
