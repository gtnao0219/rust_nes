use std::{cell::RefCell, rc::Rc};

use crate::{controller::Controller, dma::DMA, log, ppu::PPU, rom::ROM, Byte, Word};

use super::WRAM;

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
            0x0000..=0x1FFF => self.wram.borrow().read(address % 0x0800),
            0x2000..=0x3FFF => self
                .ppu
                .borrow_mut()
                .read_register((address - 0x2000) % 0x0008 + 0x2000),
            0x4016 => {
                if self.controller.borrow_mut().read() {
                    0x01
                } else {
                    0x00
                }
            }
            0x4017 => {
                // log("Read from controller 2 is not implemented yet");
                0x00
            }
            0x4000..=0x401F => {
                log(&format!(
                    "APU registers are not implemented yet: {:04X}",
                    address
                ));
                0x00
            }
            0x4020..=0x5FFF => {
                log(&format!(
                    "Expansion ROM is not implemented yet: {:04X}",
                    address
                ));
                0x00
            }
            0x6000..=0x7FFF => {
                log(&format!("SRAM is not implemented yet: {:04X}", address));
                0x00
            }
            0x8000..=0xBFFF => self.program_rom.read(address - 0x8000),
            0xC000..=0xFFFF if self.program_rom.size() <= 0x4000 => {
                self.program_rom.read(address - 0xC000)
            }
            0xC000..=0xFFFF => self.program_rom.read(address - 0x8000),
        }
    }
    pub fn write(&mut self, address: Word, data: Byte) -> () {
        match address {
            0x0000..=0x1FFF => self.wram.borrow_mut().write(address % 0x0800, data),
            0x2000..=0x3FFF => self
                .ppu
                .borrow_mut()
                .write_register((address - 0x2000) % 0x0008 + 0x2000, data),
            0x4014 => self.dma.borrow_mut().write(data),
            0x4016 => self.controller.borrow_mut().write(data),
            0x4017 => {
                // log("Write to controller 2 is not implemented yet");
            }
            0x4000..=0x401F => {
                log(&format!(
                    "APU registers are not implemented yet: {:04X}",
                    address
                ));
            }
            0x4020..=0x5FFF => {
                log(&format!(
                    "Expansion ROM is not implemented yet: {:04X}",
                    address
                ));
            }
            0x6000..=0x7FFF => {
                log(&format!("SRAM is not implemented yet: {:04X}", address));
            }
            0x8000..=0xFFFF => {
                log(&format!(
                    "Write to ROM is not implemented yet: {:04X}",
                    address
                ));
            }
        }
    }
}
