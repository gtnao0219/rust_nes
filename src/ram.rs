use crate::log;

pub struct RAM<const N: usize> {
    data: Box<[u8; N]>,
}

impl<const N: usize> Default for RAM<N> {
    fn default() -> Self {
        RAM {
            data: Box::new([0; N]),
        }
    }
}

impl<const N: usize> RAM<N> {
    pub fn read(&self, address: u16) -> u8 {
        // if address as usize >= N {
        //     log(&format!("RAM out of range: {:04X}", address));
        // }
        self.data[address as usize]
    }
    pub fn write(&mut self, address: u16, data: u8) -> () {
        // if address as usize >= N {
        //     log(&format!("RAM out of range: {:04X}", address));
        // }
        self.data[address as usize] = data;
    }
    pub fn reset(&mut self) -> () {
        self.data.fill(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let mut ram = RAM::<1024>::default();
        ram.write(0x0000, 0x01);
        ram.write(0x0001, 0x02);
        ram.write(0x0002, 0x03);
        assert_eq!(ram.read(0x0000), 0x01);
        assert_eq!(ram.read(0x0001), 0x02);
        assert_eq!(ram.read(0x0002), 0x03);
    }

    #[test]
    fn test_reset() {
        let mut ram = RAM::<1024>::default();
        ram.write(0x0000, 0x01);
        ram.write(0x0001, 0x02);
        ram.write(0x0002, 0x03);
        ram.reset();
        assert_eq!(ram.read(0x0000), 0x00);
        assert_eq!(ram.read(0x0001), 0x00);
        assert_eq!(ram.read(0x0002), 0x00);
    }
}
