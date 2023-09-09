use crate::{ppu::PPU, Word};

use super::{
    opcode::{Addressing, Opcode},
    CPU,
};

#[derive(Debug)]
pub struct DecodeResult {
    pub operand: Word,
    pub page_crossed: bool,
}

pub fn decode<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode) -> DecodeResult {
    match opcode.addressing {
        Addressing::Implied | Addressing::Accumulator => DecodeResult {
            // dummy value
            operand: 0x0000,
            page_crossed: false,
        },
        Addressing::Immediate => {
            let value = cpu.fetch_byte() as Word;
            DecodeResult {
                operand: value,
                page_crossed: false,
            }
        }
        Addressing::ZeroPage => {
            let address = cpu.fetch_byte() as Word;
            DecodeResult {
                operand: address,
                page_crossed: false,
            }
        }
        Addressing::ZeroPageX => {
            let base = cpu.fetch_byte() as Word;
            let offset = cpu.get_register().get_x() as Word;
            let address = (base + offset) & 0x00FF;
            DecodeResult {
                operand: address,
                page_crossed: false,
            }
        }
        Addressing::ZeroPageY => {
            let base = cpu.fetch_byte() as Word;
            let offset = cpu.get_register().get_y() as Word;
            let address = (base + offset) & 0x00FF;
            DecodeResult {
                operand: address,
                page_crossed: false,
            }
        }
        Addressing::Relative => {
            let offset = cpu.fetch_byte() as i8;
            let base = cpu.get_register().get_pc();
            let address = (base as i32 + offset as i32) as Word;
            DecodeResult {
                operand: address,
                page_crossed: page_crossed(base, address),
            }
        }
        Addressing::Absolute => {
            let address = cpu.fetch_word();
            DecodeResult {
                operand: address,
                page_crossed: false,
            }
        }
        Addressing::AbsoluteX => {
            let base = cpu.fetch_word();
            let offset = cpu.get_register().get_x();
            let address = (base as u32 + offset as u32) as Word;
            DecodeResult {
                operand: address,
                page_crossed: page_crossed(base, address),
            }
        }
        Addressing::AbsoluteY => {
            let base = cpu.fetch_word();
            let offset = cpu.get_register().get_y();
            let address = (base as u32 + offset as u32) as Word;
            DecodeResult {
                operand: address,
                page_crossed: page_crossed(base, address),
            }
        }
        Addressing::Indirect => {
            let address = cpu.fetch_word();

            // bug
            let indirect_address_low = cpu.read_byte(address) as Word;
            let indirect_address_high =
                cpu.read_byte((address & 0xFF00) | ((address + 1) & 0x00FF)) as Word;
            let indirect_address = (indirect_address_high << 8) | indirect_address_low;

            DecodeResult {
                operand: indirect_address,
                page_crossed: page_crossed(address, indirect_address),
            }
        }
        Addressing::IndirectX => {
            let base = cpu.fetch_byte() as Word;
            let offset = cpu.get_register().get_x() as Word;
            let address = (base + offset) & 0x00FF;

            let indirect_address_low = cpu.read_byte(address) as Word;
            let indirect_address_high = cpu.read_byte((address + 1) & 0x00FF) as Word;
            let indirect_address = (indirect_address_high << 8) | indirect_address_low;

            DecodeResult {
                operand: indirect_address,
                page_crossed: page_crossed(address, indirect_address),
            }
        }
        Addressing::IndirectY => {
            let base = cpu.fetch_byte() as Word;
            let address_low = cpu.read_byte(base) as Word;
            let address_high = cpu.read_byte((base + 1) & 0x00FF) as Word;
            let address = (address_high << 8) | address_low;
            let offset = cpu.get_register().get_y() as Word;
            let indirect_address = address + offset;
            DecodeResult {
                operand: indirect_address,
                page_crossed: page_crossed(indirect_address, address),
            }
        }
    }
}

fn page_crossed(address1: Word, address2: Word) -> bool {
    address1 & 0xFF00 != address2 & 0xFF00
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        controller::Controller,
        cpu::{opcode::OpcodeBaseName, CPUBus},
        interrupt,
        ppu::{PPUBus, PPUImpl},
        rom::ROM,
    };

    use super::*;

    fn prepare_cpu(program_rom: ROM) -> CPU<PPUImpl> {
        let interrupt = Rc::new(RefCell::new(interrupt::Interrupt::default()));
        let ppu_bus = PPUBus::new(ROM::new(vec![]), false);
        let ppu = Rc::new(RefCell::new(PPUImpl::new(ppu_bus, interrupt.clone())));
        let controller = Rc::new(RefCell::new(Controller::default()));
        let wram = Rc::new(RefCell::new(crate::cpu::WRAM::default()));
        let dma = Rc::new(RefCell::new(crate::dma::DMA::new(
            wram.clone(),
            ppu.clone(),
        )));
        let cpu_bus = CPUBus::new(
            program_rom,
            wram.clone(),
            ppu.clone(),
            controller.clone(),
            dma.clone(),
        );
        CPU::new(cpu_bus, interrupt.clone())
    }

    #[test]
    fn test_decode_implied() {
        let program_rom = ROM::new(Vec::new());
        let mut cpu = prepare_cpu(program_rom);
        let opcode = Opcode {
            base_name: OpcodeBaseName::TXA,
            addressing: Addressing::Implied,
            cycle: 2,
        };
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x0000);
        assert_eq!(result.page_crossed, false);
    }

    #[test]
    fn test_decode_accumulator() {
        let program_rom = ROM::new(Vec::new());
        let mut cpu = prepare_cpu(program_rom);
        let opcode = Opcode {
            base_name: OpcodeBaseName::ASL,
            addressing: Addressing::Accumulator,
            cycle: 2,
        };
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x0000);
        assert_eq!(result.page_crossed, false);
    }

    #[test]
    fn test_decode_immediate() {
        let program_rom_data = vec![0x01];
        let program_rom = ROM::new(program_rom_data);
        let mut cpu = prepare_cpu(program_rom);
        let opcode = Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::Immediate,
            cycle: 2,
        };
        // pc: 0x8000 -> 0x01
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x01);
        assert_eq!(result.page_crossed, false);
    }

    #[test]
    fn test_decode_zero_page() {
        let program_rom_data = vec![0x01];
        let program_rom = ROM::new(program_rom_data);
        let mut cpu = prepare_cpu(program_rom);
        let opcode = Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        };
        // pc: 0x8000 -> 0x01
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x01);
        assert_eq!(result.page_crossed, false);
    }

    #[test]
    fn test_decode_zero_page_x() {
        let program_rom_data = vec![0xF0, 0xF0];
        let program_rom = ROM::new(program_rom_data);
        let mut cpu = prepare_cpu(program_rom);
        let opcode = Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::ZeroPageX,
            cycle: 4,
        };
        // pc: 0x8000 -> 0xF0
        // x: 0x0F
        cpu.get_register().set_x(0x0F);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0xFF);
        assert_eq!(result.page_crossed, false);

        // pc: 0x8001 -> 0xF0
        // x: 0x11
        cpu.get_register().set_x(0x11);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x01);
        assert_eq!(result.page_crossed, false);
    }

    #[test]
    fn test_decode_zero_page_y() {
        let program_rom_data = vec![0xF0, 0xF0];
        let program_rom = ROM::new(program_rom_data);
        let mut cpu = prepare_cpu(program_rom);
        let opcode = Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::ZeroPageY,
            cycle: 4,
        };
        // pc: 0x8000 -> 0xF0
        // y: 0x0F
        cpu.get_register().set_y(0x0F);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0xFF);
        assert_eq!(result.page_crossed, false);

        // pc: 0x8001 -> 0xF0
        // y: 0x11
        cpu.get_register().set_y(0x11);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x01);
        assert_eq!(result.page_crossed, false);
    }

    #[test]
    fn test_decode_absolute() {
        let program_rom_data = vec![0x01, 0x02];
        let program_rom = ROM::new(program_rom_data);
        let mut cpu = prepare_cpu(program_rom);
        let opcode = Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::Absolute,
            cycle: 4,
        };
        // pc: 0x8000 -> 0x01, 0x8001 -> 0x02
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x0201);
        assert_eq!(result.page_crossed, false);
    }

    #[test]
    fn test_decode_absolute_x() {
        let program_rom_data = vec![0x01, 0x02, 0xFF, 0x00, 0x02, 0xFF];
        let program_rom = ROM::new(program_rom_data);
        let mut cpu = prepare_cpu(program_rom);
        let opcode = Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::AbsoluteX,
            cycle: 4,
        };
        // pc: 0x8000 -> 0x01, 0x8001 -> 0x02
        // x: 0x01
        cpu.get_register().set_x(0x01);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x0202);
        assert_eq!(result.page_crossed, false);

        // pc: 0x8002 -> 0xFF, 0x8003 -> 0x00
        // x: 0x02
        cpu.get_register().set_x(0x02);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x0101);
        assert_eq!(result.page_crossed, true);

        // pc: 0x8004 -> 0x02, 0x8005 -> 0xFF
        // x: 0xFF
        cpu.get_register().set_x(0xFF);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x0001);
        assert_eq!(result.page_crossed, true);
    }

    #[test]
    fn test_decode_absolute_y() {
        let program_rom_data = vec![0x01, 0x02, 0xFF, 0x00, 0x02, 0xFF];
        let program_rom = ROM::new(program_rom_data);
        let mut cpu = prepare_cpu(program_rom);
        let opcode = Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::AbsoluteY,
            cycle: 4,
        };
        // pc: 0x8000 -> 0x01, 0x8001 -> 0x02
        // y: 0x01
        cpu.get_register().set_y(0x01);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x0202);
        assert_eq!(result.page_crossed, false);

        // pc: 0x8002 -> 0xFF, 0x8003 -> 0x00
        // y: 0x02
        cpu.get_register().set_y(0x02);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x0101);
        assert_eq!(result.page_crossed, true);

        // pc: 0x8004 -> 0x02, 0x8005 -> 0xFF
        // y: 0xFF
        cpu.get_register().set_y(0xFF);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x0001);
        assert_eq!(result.page_crossed, true);
    }

    #[test]
    fn test_decode_relative() {
        let mut program_rom_data = vec![0x00; 0x8000];
        program_rom_data[0x0000] = 0x01;
        program_rom_data[0x0001] = 0xFF;
        program_rom_data[0x7FFE] = 0x02;
        let program_rom = ROM::new(program_rom_data);
        let mut cpu = prepare_cpu(program_rom);
        let opcode = Opcode {
            base_name: OpcodeBaseName::BPL,
            addressing: Addressing::Relative,
            cycle: 2,
        };
        // pc: 0x8000 -> 0x01
        // next_pc: 0x8001
        cpu.get_register().set_pc(0x8000);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x8002);
        assert_eq!(result.page_crossed, false);

        // pc: 0x8001 -> 0xFF(-1)
        // next_pc: 0x8002
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x8001);
        assert_eq!(result.page_crossed, false);

        // pc: 0xFFFE -> 0x02
        // next_pc: 0xFFFF
        cpu.get_register().set_pc(0xFFFE);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x0001);
        assert_eq!(result.page_crossed, true);

        // pc: 0x0000 -> 0xFD(-3)
        // next_pc: 0x0001
        cpu.get_register().set_pc(0x0000);
        cpu.write(0x00, 0xFD);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0xFFFE);
        assert_eq!(result.page_crossed, true);

        // pc: 0x00FE -> 0x01
        // next_pc: 0x00FF
        cpu.get_register().set_pc(0x00FE);
        cpu.write(0xFE, 0x01);
        let result = decode(&mut cpu, &opcode);
        assert_eq!(result.operand, 0x0100);
        assert_eq!(result.page_crossed, true);
    }
}
