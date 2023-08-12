use crate::Word;

use super::{
    opcode::{Addressing, Opcode},
    CPU,
};

#[derive(Debug)]
pub struct DecodeResult {
    pub operand: Word,
    pub page_crossed: bool,
}

pub fn decode(cpu: &mut CPU, opcode: &Opcode) -> DecodeResult {
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
