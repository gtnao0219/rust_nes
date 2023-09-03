use std::{cell::RefCell, rc::Rc};

use crate::{controller::Controller, dma::DMA, log, ppu::PPU, rom::ROM, Byte, Word};

use super::WRAM;

pub struct CPUBus<P: PPU> {
    program_rom: ROM,
    wram: Rc<RefCell<WRAM>>,
    ppu: Rc<RefCell<P>>,
    controller: Rc<RefCell<Controller>>,
    dma: Rc<RefCell<DMA<P>>>,
}

impl<P: PPU> CPUBus<P> {
    pub fn new(
        program_rom: ROM,
        wram: Rc<RefCell<WRAM>>,
        ppu: Rc<RefCell<P>>,
        controller: Rc<RefCell<Controller>>,
        dma: Rc<RefCell<DMA<P>>>,
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
                // log(&format!(
                //     "APU registers are not implemented yet: {:04X}",
                //     address
                // ));
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
                // log(&format!(
                //     "APU registers are not implemented yet: {:04X}",
                //     address
                // ));
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::cpu::WRAM;
    use crate::ppu::MockPPU;
    use crate::rom::ROM;
    use mockall::predicate;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_read_write() {
        let mut program_rom_data = vec![0; 0x8000];
        for i in 0..0x4000 {
            program_rom_data[i] = (i % 0x100) as u8;
        }
        for i in 0..0x4000 {
            program_rom_data[i + 0x4000] = (i + 1 % 0x100) as u8;
        }
        let program_rom = ROM::new(program_rom_data);
        let wram = Rc::new(RefCell::new(WRAM::default()));
        let ppu = Rc::new(RefCell::new(MockPPU::new()));
        let controller = Rc::new(RefCell::new(Controller::default()));
        let dma = Rc::new(RefCell::new(DMA::new(wram.clone(), ppu.clone())));

        let mut bus = CPUBus::new(
            program_rom,
            wram.clone(),
            ppu.clone(),
            controller.clone(),
            dma.clone(),
        );

        // WRAM r/w
        bus.write(0x0000, 0x01);
        bus.write(0x07FF, 0x02);
        assert_eq!(bus.read(0x0000), 0x01);
        assert_eq!(bus.read(0x07FF), 0x02);
        assert_eq!(bus.read(0x0800), 0x01);
        assert_eq!(bus.read(0x0FFF), 0x02);
        assert_eq!(bus.read(0x1000), 0x01);
        assert_eq!(bus.read(0x17FF), 0x02);
        assert_eq!(bus.read(0x1800), 0x01);
        assert_eq!(bus.read(0x1FFF), 0x02);
        bus.write(0x0800, 0x03);
        bus.write(0x0FFF, 0x04);
        assert_eq!(bus.read(0x0000), 0x03);
        assert_eq!(bus.read(0x07FF), 0x04);
        bus.write(0x1000, 0x05);
        bus.write(0x17FF, 0x06);
        assert_eq!(bus.read(0x0800), 0x05);
        assert_eq!(bus.read(0x0FFF), 0x06);
        bus.write(0x1800, 0x07);
        bus.write(0x1FFF, 0x08);
        assert_eq!(bus.read(0x1000), 0x07);
        assert_eq!(bus.read(0x17FF), 0x08);

        // PPU r/w
        ppu.borrow_mut()
            .expect_read_register()
            .with(predicate::eq(0x2000))
            .times(3)
            .return_const(0x01);
        bus.read(0x2000);
        bus.read(0x2008);
        bus.read(0x3FF8);
        ppu.borrow_mut()
            .expect_read_register()
            .with(predicate::eq(0x2007))
            .times(3)
            .return_const(0x01);
        bus.read(0x2007);
        bus.read(0x200F);
        bus.read(0x3FFF);
        ppu.borrow_mut()
            .expect_write_register()
            .with(predicate::eq(0x2000), predicate::eq(0x01))
            .times(3)
            .return_const(());
        bus.write(0x2000, 0x01);
        bus.write(0x2008, 0x01);
        bus.write(0x3FF8, 0x01);
        ppu.borrow_mut()
            .expect_write_register()
            .with(predicate::eq(0x2007), predicate::eq(0x01))
            .times(3)
            .return_const(());
        bus.write(0x2007, 0x01);
        bus.write(0x200F, 0x01);
        bus.write(0x3FFF, 0x01);

        // controller r/w
        controller.borrow_mut().key_down(0);
        controller.borrow_mut().key_down(7);
        controller.borrow_mut().write(0x01);
        controller.borrow_mut().write(0x00);
        assert_eq!(bus.read(0x4016), 0x01);
        assert_eq!(bus.read(0x4016), 0x00);
        assert_eq!(bus.read(0x4016), 0x00);
        assert_eq!(bus.read(0x4016), 0x00);
        assert_eq!(bus.read(0x4016), 0x00);
        assert_eq!(bus.read(0x4016), 0x00);
        assert_eq!(bus.read(0x4016), 0x00);
        assert_eq!(bus.read(0x4016), 0x01);

        // DMA
        for i in 0..=0xff {
            wram.borrow_mut().write(i + 0x100, i as u8);
        }
        bus.write(0x4014, 0x01);
        for i in 0..=0xff {
            ppu.borrow_mut()
                .expect_transfer_sprite()
                .with(predicate::eq(i), predicate::eq(i))
                .return_const(());
        }
        dma.borrow_mut().run();

        // ROM
        assert_eq!(bus.read(0x8000), 0x00);
        assert_eq!(bus.read(0xBFFF), 0xFF);
        assert_eq!(bus.read(0xC000), 0x01);
        assert_eq!(bus.read(0xFFFF), 0x00);
    }

    #[test]
    fn test_when_rom_block_is_one() {
        let mut program_rom_data = vec![0; 0x4000];
        for i in 0..0x4000 {
            program_rom_data[i] = (i % 0x100) as u8;
        }
        let program_rom = ROM::new(program_rom_data);
        let wram = Rc::new(RefCell::new(WRAM::default()));
        let ppu = Rc::new(RefCell::new(MockPPU::new()));
        let controller = Rc::new(RefCell::new(Controller::default()));
        let dma = Rc::new(RefCell::new(DMA::new(wram.clone(), ppu.clone())));

        let bus = CPUBus::new(
            program_rom,
            wram.clone(),
            ppu.clone(),
            controller.clone(),
            dma.clone(),
        );

        assert_eq!(bus.read(0x8000), 0x00);
        assert_eq!(bus.read(0xBFFF), 0xFF);
        assert_eq!(bus.read(0xC000), 0x00);
        assert_eq!(bus.read(0xFFFF), 0xFF);
    }
}
