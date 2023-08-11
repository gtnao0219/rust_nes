use std::{cell::RefCell, rc::Rc};

use crate::{ppu::PPU, rom::ROM, Byte, Word, console_log};

use super::WRAM;

#[derive(Debug)]
pub struct CPUBus {
    program_rom: ROM,
    wram: WRAM,
    ppu: Rc<RefCell<PPU>>,
}

impl CPUBus {
    pub fn new(program_rom: ROM, ppu: Rc<RefCell<PPU>>) -> Self {
        CPUBus {
            program_rom,
            wram: WRAM::default(),
            ppu,
        }
    }
    pub fn read(&self, address: Word) -> Byte {
        match address {
            0x0000..=0x07FF => self.wram.read(address),
            0x0800..=0x1FFF => self.wram.read(address % 0x0800),
            0x2000..=0x2007 => self.ppu.borrow().read_register(address),
            0x2008..=0x3FFF => self.ppu.borrow().read_register((address - 0x2000) % 0x0008 + 0x2000),
            0x4000..=0x401F => panic!("APU registers are not implemented yet"),
            0x4020..=0x5FFF => panic!("Expansion ROM is not implemented yet"),
            0x6000..=0x7FFF => {
                console_log(&format!("SRAM is not implemented yet: {:04X}", address));
                0x00
            }
            0x8000..=0xFFFF => self.program_rom.read(address - 0x8000),
        }
    }
    pub fn write(&mut self, address: Word, data: Byte) {
        match address {
            0x0000..=0x07FF => self.wram.write(address, data),
            0x0800..=0x1FFF => self.wram.write(address % 0x0800, data),
            0x2000..=0x2007 => self.ppu.borrow_mut().write_register(address, data),
            0x2008..=0x3FFF => self.ppu.borrow_mut().write_register((address - 0x2000) % 0x0008 + 0x2000, data),
            0x4000..=0x401F => panic!("APU registers are not implemented yet"),
            _ => panic!("Write to ROM is not allowed"),
        }
    }
}
