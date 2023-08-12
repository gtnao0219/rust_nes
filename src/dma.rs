use std::{cell::RefCell, rc::Rc};

use crate::{cpu::WRAM, ppu::PPU, Byte, Word};

#[derive(Debug)]
pub struct DMA {
    wram: Rc<RefCell<WRAM>>,
    ppu: Rc<RefCell<PPU>>,
    is_processing: bool,
    ram_address: Word,
}

impl DMA {
    pub fn new(wram: Rc<RefCell<WRAM>>, ppu: Rc<RefCell<PPU>>) -> Self {
        DMA {
            wram,
            ppu,
            is_processing: false,
            ram_address: 0,
        }
    }
    pub fn run(&mut self) {
        if !self.is_processing {
            return;
        }
        for i in 0..=0xff {
            let data = self.wram.borrow().read(self.ram_address + i);
            self.ppu.borrow_mut().transfer_sprite(i as u8, data);
        }
        self.is_processing = false;
    }
    pub fn write(&mut self, data: Byte) {
        self.ram_address = (data as Word) << 8;
        self.is_processing = true;
    }
    pub fn is_processing(&self) -> bool {
        self.is_processing
    }
}
