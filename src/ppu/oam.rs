#[derive(Debug)]
pub struct OAM {
    data: [u8; 256],
}
impl Default for OAM {
    fn default() -> Self {
        OAM { data: [0; 256] }
    }
}

impl OAM {
    pub fn write(&mut self, addr: u8, data: u8) {
        self.data[addr as usize] = data;
    }
    pub fn read(&self, addr: u8) -> u8 {
        self.data[addr as usize]
    }
    pub fn head_y(&self) -> u8 {
        self.data[0]
    }
    pub fn iter(&self) -> OAMIterator {
        OAMIterator {
            oam: self,
            entry_index: 0,
        }
    }
}

#[derive(Debug)]
pub struct OAMIterator<'a> {
    oam: &'a OAM,
    entry_index: usize,
}
impl<'a> Iterator for OAMIterator<'a> {
    type Item = [u8; 4];

    fn next(&mut self) -> Option<Self::Item> {
        if self.entry_index >= 64 {
            return None;
        }
        let data = [
            self.oam.data[self.entry_index * 4],
            self.oam.data[self.entry_index * 4 + 1],
            self.oam.data[self.entry_index * 4 + 2],
            self.oam.data[self.entry_index * 4 + 3],
        ];
        self.entry_index += 1;
        Some(data)
    }
}
