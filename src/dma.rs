use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    rc::Rc,
};

use crate::{cpu::WRAM, ppu::PPU, Cycle};

pub struct DMA<P: PPU> {
    wram: Rc<RefCell<WRAM>>,
    ppu: Rc<RefCell<P>>,
    ram_address: Option<u16>,
}

impl<P: PPU> Debug for DMA<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DMA")
            .field("ram_address", &self.ram_address)
            .finish()
    }
}

impl<P: PPU> DMA<P> {
    pub fn new(wram: Rc<RefCell<WRAM>>, ppu: Rc<RefCell<P>>) -> Self {
        DMA {
            wram,
            ppu,
            ram_address: None,
        }
    }
    pub fn run(&mut self) -> Cycle {
        match self.ram_address {
            Some(ram_address) => {
                for i in 0..=0xff {
                    let data = self.wram.borrow().read(ram_address + i);
                    self.ppu.borrow_mut().transfer_sprite(i as u8, data);
                }
                self.ram_address = None;
                513
            }
            None => 0,
        }
    }
    pub fn write(&mut self, data: u8) {
        self.ram_address = Some((data as u16) << 8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::WRAM;
    use crate::ppu::MockPPU;
    use mockall::predicate;

    #[test]
    fn test_dma() {
        let wram = Rc::new(RefCell::new(WRAM::default()));
        for i in 0..=0xff {
            wram.borrow_mut().write(i, i as u8);
        }
        for i in 0..=0xff {
            wram.borrow_mut().write(i + 0x100, (255 - i) as u8);
        }
        let ppu = Rc::new(RefCell::new(MockPPU::new()));
        let mut dma = DMA::new(wram.clone(), ppu.clone());

        assert_eq!(dma.run(), 0);

        dma.write(0x00);
        for i in 0..=255 {
            ppu.borrow_mut()
                .expect_transfer_sprite()
                .with(predicate::eq(i), predicate::eq(i))
                .return_const(());
        }
        assert_eq!(dma.run(), 513);

        assert_eq!(dma.run(), 0);

        dma.write(0x01);
        for i in 0..=255 {
            ppu.borrow_mut()
                .expect_transfer_sprite()
                .with(predicate::eq(i), predicate::eq((255 - i) as u8))
                .return_const(());
        }
        assert_eq!(dma.run(), 513);
    }
}
