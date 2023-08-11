use std::{cell::RefCell, rc::Rc};

use crate::{
    cartridge::Cartridge,
    cpu::{CPUBus, CPU, WRAM},
    interrupt,
    ppu::{PPUBus, Palette, PPU, VRAM}, console_log, renderer::Renderer,
};

pub struct NES {
    cpu: CPU,
    ppu: Rc<RefCell<PPU>>,
}

impl NES {
    pub fn new(rom_data: &[u8]) -> Self {
        let cartridge = Cartridge::new(rom_data);

        let interrupt = Rc::new(RefCell::new(interrupt::Interrupt::default()));
        let ppu_bus = PPUBus::new(cartridge.character_rom);
        let ppu = PPU::new(ppu_bus, interrupt.clone());
        let ppu_rc = Rc::new(RefCell::new(ppu));
        let cpu_bus = CPUBus::new(cartridge.program_rom, ppu_rc.clone());
        let cpu = CPU::new(cpu_bus, interrupt.clone());

        NES { cpu, ppu: ppu_rc }
    }

    pub fn frame(&mut self) {
        loop {
            let mut cycle = 0;
            cycle += self.cpu.run();
            let rendering_data = self.ppu.borrow_mut().run(cycle * 3);
            if let Some(rendering_data) = rendering_data {
                let mut renderer = Renderer::new();
                renderer.render(rendering_data);
                break;
            }
        }
    }
}
