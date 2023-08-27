use crate::{log, Byte, Word};

pub struct RAM<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Default for RAM<N> {
    fn default() -> Self {
        RAM { data: [0; N] }
    }
}

impl<const N: usize> RAM<N> {
    pub fn read(&self, address: Word) -> Byte {
        if address as usize >= N {
            log(&format!("RAM out of range: {:04X}", address));
            panic!();
        }
        self.data[address as usize]
    }
    pub fn write(&mut self, address: Word, data: Byte) -> () {
        if address as usize >= N {
            log(&format!("RAM out of range: {:04X}", address));
            panic!();
        }
        self.data[address as usize] = data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_read_write() {
        let mut ram = RAM::<1024>::default();
        ram.write(0x0000, 0x01);
        ram.write(0x0001, 0x02);
        ram.write(0x0002, 0x03);
        assert_eq!(ram.read(0x0000), 0x01);
        assert_eq!(ram.read(0x0001), 0x02);
        assert_eq!(ram.read(0x0002), 0x03);
    }
}
