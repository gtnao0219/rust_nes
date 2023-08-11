pub const OAM_SIZE: usize = 256;
pub const OAM_ENTRY_SIZE: usize = 4;
pub const OAM_ENTRY_NUM: usize = OAM_SIZE / OAM_ENTRY_SIZE;

pub type OAMData = [u8; OAM_SIZE];
pub type OAMEntry = [u8; OAM_ENTRY_SIZE];

#[derive(Debug)]
pub struct OAM {
    data: OAMData,
}
impl Default for OAM {
    fn default() -> Self {
        OAM {
            data: [0; OAM_SIZE],
        }
    }
}

impl OAM {
    pub fn write(&mut self, addr: u8, data: u8) {
        self.data[addr as usize] = data;
    }
    pub fn head_y(&self) -> u16 {
        self.data[0] as u16
    }
    pub fn iter(&self) -> OAMIterator {
        OAMIterator {
            oam: self,
            entry_index: 0,
        }
    }
}

pub struct OAMIterator<'a> {
    oam: &'a OAM,
    entry_index: u8,
}
impl<'a> Iterator for OAMIterator<'a> {
    type Item = OAMEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.entry_index >= OAM_ENTRY_NUM as u8 {
            return None;
        }
        let data = [
            self.oam.data[self.entry_index as usize * OAM_ENTRY_SIZE],
            self.oam.data[self.entry_index as usize * OAM_ENTRY_SIZE + 1],
            self.oam.data[self.entry_index as usize * OAM_ENTRY_SIZE + 2],
            self.oam.data[self.entry_index as usize * OAM_ENTRY_SIZE + 3],
        ];
        self.entry_index += 1;
        Some(data)
    }
}
