use crate::{Word, Byte};

#[derive(Debug)]
pub struct ROM {
    pub data: Vec<u8>,
}

impl ROM {
    pub fn new(data: Vec<u8>) -> Self {
        ROM { data }
    }
    pub fn read(&self, address: Word) -> Byte {
        self.data[address as usize]
    }
    pub fn write(&mut self, address: Word, data: Byte) {
        self.data[address as usize] = data;
    }
}
