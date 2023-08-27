use crate::{log, Byte, Word};

pub struct ROM {
    pub data: Vec<u8>,
}

impl ROM {
    pub fn new(data: Vec<u8>) -> Self {
        ROM { data }
    }
    pub fn read(&self, address: Word) -> Byte {
        if address as usize >= self.size() {
            log(&format!("ROM out of range: {:04X}", address));
            panic!();
        }
        self.data[address as usize]
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // normal
    fn test_read() {
        let rom = ROM::new(vec![0x00, 0x01, 0x02, 0x03]);
        assert_eq!(rom.read(0x00), 0x00);
        assert_eq!(rom.read(0x01), 0x01);
        assert_eq!(rom.read(0x02), 0x02);
        assert_eq!(rom.read(0x03), 0x03);
    }

    #[test]
    // size
    fn test_size() {
        let rom = ROM::new(vec![0x00, 0x01, 0x02, 0x03]);
        assert_eq!(rom.size(), 4);
    }
}
