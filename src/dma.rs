use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    rc::Rc,
};

use crate::{cpu::WRAM, ppu::PPU, Byte, Cycle, Word};

pub struct DMA {
    wram: Rc<RefCell<WRAM>>,
    ppu: Rc<RefCell<PPU>>,
    ram_address: Option<Word>,
}

impl Debug for DMA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DMA")
            .field("ram_address", &self.ram_address)
            .finish()
    }
}

impl DMA {
    pub fn new(wram: Rc<RefCell<WRAM>>, ppu: Rc<RefCell<PPU>>) -> Self {
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
                    self.ppu.borrow_mut().transfer_sprite(i as Byte, data);
                }
                self.ram_address = None;
                513
            }
            None => 0,
        }
    }
    pub fn write(&mut self, data: Byte) {
        self.ram_address = Some((data as Word) << 8);
    }
}
