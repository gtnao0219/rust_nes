use std::{cell::RefCell, rc::Rc};

use crate::{interrupt::Interrupt, log, ram::RAM, Byte, Cycle, Word};

mod bus;
mod decoder;
mod executor;
mod opcode;
mod register;

pub use bus::CPUBus;

const WRAM_SIZE: usize = 2048;
pub type WRAM = RAM<WRAM_SIZE>;

#[derive(Debug)]
pub struct CPU {
    bus: CPUBus,
    register: register::CPURegister,
    interrupt: Rc<RefCell<Interrupt>>,
}

impl CPU {
    pub fn new(bus: CPUBus, interrupt: Rc<RefCell<Interrupt>>) -> Self {
        CPU {
            bus,
            register: register::CPURegister::default(),
            interrupt,
        }
    }

    pub fn reset(&mut self) {
        log("CPU reset...");
        let pc = self.read_word(0xFFFC);
        self.register.set_pc(if pc == 0 { 0x8000 } else { pc });
        log(&format!("PC: {:04X}", self.register.get_pc()));
    }
    pub fn run(&mut self) -> Cycle {
        if self.interrupt.borrow().is_nmi() {
            self.process_nmi();
        }
        if self.interrupt.borrow().is_irq() {
            self.process_irq();
        }
        let opcode_byte = self.fetch_byte();
        let opcode = opcode::get_opcode(opcode_byte);
        let decode_result = decoder::decode(self, &opcode);
        executor::execute(self, &opcode, &decode_result.operand);
        opcode.cycle + if decode_result.page_crossed { 1 } else { 0 }
    }

    fn fetch_byte(&mut self) -> Byte {
        let data = self.read_byte(self.register.get_pc());
        self.register.increment_pc_byte();
        data
    }
    fn fetch_word(&mut self) -> Word {
        let data = self.read_word(self.register.get_pc());
        self.register.increment_pc_word();
        data
    }
    fn read_byte(&self, address: Word) -> Byte {
        self.bus.read(address)
    }
    fn read_word(&self, address: Word) -> Word {
        self.bus.read(address) as Word | ((self.bus.read(address + 1) as Word) << 8)
    }
    fn write(&mut self, address: Word, data: Byte) {
        self.bus.write(address, data);
    }

    fn push(&mut self, data: Byte) {
        self.write(self.register.stack_address(), data);
        self.register.decrement_s();
    }
    fn pop(&mut self) -> Byte {
        self.register.increment_s();
        self.read_byte(self.register.stack_address())
    }
    fn push_status(&mut self) {
        self.push(self.register.get_p());
    }
    fn pop_status(&mut self) {
        let p = self.pop();
        self.register.set_p(p);
    }
    fn push_pc(&mut self) {
        let pc = self.register.get_pc();
        self.push((pc >> 8) as Byte);
        self.push(pc as Byte);
    }
    fn pop_pc(&mut self) {
        let lo = self.pop() as Word;
        let hi = self.pop() as Word;
        self.register.set_pc((hi << 8) | lo);
    }
    fn branch(&mut self, address: Word) {
        self.register.set_pc(address);
    }
    fn process_irq(&mut self) {
        if self.register.get_i() {
            return;
        }
        self.interrupt.borrow_mut().clear_irq();
        self.register.clear_b();
        self.push_pc();
        self.push_status();
        self.register.set_i();
        self.set_pc_by_irq();
    }
    fn set_pc_by_irq(&mut self) {
        self.register.set_pc(self.read_word(0xFFFE));
    }
    fn process_nmi(&mut self) {
        self.interrupt.borrow_mut().clear_nmi();
        self.register.clear_b();
        self.push_pc();
        self.push_status();
        self.register.set_i();
        self.set_pc_by_nmi();
    }
    fn set_pc_by_nmi(&mut self) {
        self.register.set_pc(self.read_word(0xFFFA));
    }

    fn get_register(&mut self) -> &mut register::CPURegister {
        &mut self.register
    }
}
