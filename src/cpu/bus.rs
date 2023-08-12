use std::{cell::RefCell, rc::Rc};

use crate::{console_log, controller::Controller, ppu::PPU, rom::ROM, Byte, Word, dma::DMA};

use super::WRAM;

#[derive(Debug)]
pub struct CPUBus {
    program_rom: ROM,
    wram: Rc<RefCell<WRAM>>,
    ppu: Rc<RefCell<PPU>>,
    controller: Rc<RefCell<Controller>>,
    dma: Rc<RefCell<DMA>>,
}

impl CPUBus {
    pub fn new(
        program_rom: ROM,
        wram: Rc<RefCell<WRAM>>,
        ppu: Rc<RefCell<PPU>>,
        controller: Rc<RefCell<Controller>>,
        dma: Rc<RefCell<DMA>>,
    ) -> Self {
        CPUBus {
            program_rom,
            wram,
            ppu,
            controller,
            dma,
        }
    }
    pub fn read(&self, address: Word) -> Byte {
        match address {
            0x0000..=0x07FF => self.wram.borrow().read(address),
            0x0800..=0x1FFF => self.wram.borrow().read(address % 0x0800),
            0x2000..=0x2007 => self.ppu.borrow().read_register(address),
            0x2008..=0x3FFF => self
                .ppu
                .borrow()
                .read_register((address - 0x2000) % 0x0008 + 0x2000),
            0x4016 => {
                if self.controller.borrow_mut().read() {
                    0x01
                } else {
                    0x00
                }
            }
            0x4000..=0x401F => {
                console_log("APU registers are not implemented yet");
                0x00
            }
            0x4020..=0x5FFF => {
                // console_log(&format!(
                //     "Expansion ROM is not implemented yet: {:04X}",
                //     address
                // ));
                0x00
            }
            0x6000..=0x7FFF => {
                // console_log(&format!("SRAM is not implemented yet: {:04X}", address));
                0x00
            }
            0x8000..=0xBFFF => self.program_rom.read(address - 0x8000),
            0xC000..=0xFFFF => self.program_rom.read(address - 0xC000),
        }
    }
    pub fn write(&mut self, address: Word, data: Byte) {
        match address {
            0x0000..=0x07FF => self.wram.borrow_mut().write(address, data),
            0x0800..=0x1FFF => self.wram.borrow_mut().write(address % 0x0800, data),
            0x2000..=0x2007 => self.ppu.borrow_mut().write_register(address, data),
            0x2008..=0x3FFF => self
                .ppu
                .borrow_mut()
                .write_register((address - 0x2000) % 0x0008 + 0x2000, data),
            0x4014 => {
                self.dma.borrow_mut().write(data);
            }
            0x4016 => {
                self.controller.borrow_mut().write(data);
            }
            0x4000..=0x401F => {
                console_log("APU registers are not implemented yet");
            }
            _ => {
                // console_log(&format!("Write to {:04X} is not implemented yet", address));
            }
        }
    }
}
