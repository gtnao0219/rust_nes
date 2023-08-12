use std::{cell::RefCell, rc::Rc};

use crate::{
    cartridge::Cartridge,
    console_log,
    controller::Controller,
    cpu::{CPUBus, CPU},
    interrupt,
    ppu::{PPUBus, PPU},
    renderer::Renderer,
};

pub struct NES {
    cpu: CPU,
    ppu: Rc<RefCell<PPU>>,
    controller: Rc<RefCell<Controller>>,
    dma: Rc<RefCell<crate::dma::DMA>>,
}

impl NES {
    pub fn new(rom_data: &[u8]) -> Self {
        let cartridge = Cartridge::new(rom_data);

        let interrupt = Rc::new(RefCell::new(interrupt::Interrupt::default()));
        let ppu_bus = PPUBus::new(cartridge.character_rom);
        let ppu = Rc::new(RefCell::new(PPU::new(ppu_bus, interrupt.clone())));
        let controller = Rc::new(RefCell::new(Controller::default()));
        let wram = Rc::new(RefCell::new(crate::cpu::WRAM::default()));
        let dma = Rc::new(RefCell::new(crate::dma::DMA::new(wram.clone(), ppu.clone())));
        let cpu_bus = CPUBus::new(cartridge.program_rom, wram.clone(), ppu.clone(), controller.clone(), dma.clone());
        let mut cpu = CPU::new(cpu_bus, interrupt.clone());
        cpu.reset();

        NES {
            cpu,
            ppu,
            controller,
            dma,
        }
    }

    pub fn frame(&mut self) {
        loop {
            let mut cycle = 0;
            if self.dma.borrow().is_processing() {
                self.dma.borrow_mut().run();
                cycle += 513;
            }
            cycle += self.cpu.run();
            let rendering_data = self.ppu.borrow_mut().run(cycle * 3);
            if let Some(rendering_data) = rendering_data {
                let mut renderer = Renderer::new();
                renderer.render(rendering_data);
                break;
            }
        }
    }

    pub fn key_down(&mut self, key: u8) {
        self.controller.borrow_mut().key_down(key);
    }

    pub fn key_up(&mut self, key: u8) {
        self.controller.borrow_mut().key_up(key);
    }
}
