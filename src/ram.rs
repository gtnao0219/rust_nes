use crate::{Byte, Word};

#[derive(Debug)]
pub struct RAM<const N: usize> {
    pub data: [u8; N],
}

impl<const N: usize> Default for RAM<N> {
    fn default() -> Self {
        RAM { data: [0; N] }
    }
}

impl<const N: usize> RAM<N> {
    pub fn read(&self, address: Word) -> Byte {
        self.data[address as usize]
    }
    pub fn write(&mut self, address: Word, data: Byte) {
        self.data[address as usize] = data;
    }
}
