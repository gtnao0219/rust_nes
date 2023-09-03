use crate::log;

pub struct ROM {
    data: Box<[u8]>,
}

impl ROM {
    pub fn new(data: Vec<u8>) -> Self {
        ROM {
            data: data.into_boxed_slice(),
        }
    }
    pub fn read(&self, address: u16) -> u8 {
        // if address as usize >= self.size() {
        //     log(&format!("ROM out of range: {:04X}", address));
        // }
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
    fn test_read() {
        let rom = ROM::new(vec![0x00, 0x01, 0x02, 0x03]);
        assert_eq!(rom.read(0x0000), 0x00);
        assert_eq!(rom.read(0x0001), 0x01);
        assert_eq!(rom.read(0x0002), 0x02);
        assert_eq!(rom.read(0x0003), 0x03);
    }

    #[test]
    fn test_size() {
        let rom = ROM::new(vec![0x00, 0x01, 0x02, 0x03]);
        assert_eq!(rom.size(), 4);
    }
}
