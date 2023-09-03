use crate::{ppu::PPU, Byte, Word};

use super::{
    opcode::{Addressing, Opcode, OpcodeBaseName},
    CPU,
};

pub fn execute<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    match opcode.base_name {
        OpcodeBaseName::LDA => execute_lda(cpu, opcode, operand),
        OpcodeBaseName::LDX => execute_ldx(cpu, opcode, operand),
        OpcodeBaseName::LDY => execute_ldy(cpu, opcode, operand),
        OpcodeBaseName::STA => execute_sta(cpu, opcode, operand),
        OpcodeBaseName::STX => execute_stx(cpu, opcode, operand),
        OpcodeBaseName::STY => execute_sty(cpu, opcode, operand),
        OpcodeBaseName::TAX => execute_tax(cpu, opcode, operand),
        OpcodeBaseName::TAY => execute_tay(cpu, opcode, operand),
        OpcodeBaseName::TSX => execute_tsx(cpu, opcode, operand),
        OpcodeBaseName::TXA => execute_txa(cpu, opcode, operand),
        OpcodeBaseName::TXS => execute_txs(cpu, opcode, operand),
        OpcodeBaseName::TYA => execute_tya(cpu, opcode, operand),
        OpcodeBaseName::ADC => execute_adc(cpu, opcode, operand),
        OpcodeBaseName::AND => execute_and(cpu, opcode, operand),
        OpcodeBaseName::ASL => execute_asl(cpu, opcode, operand),
        OpcodeBaseName::BIT => execute_bit(cpu, opcode, operand),
        OpcodeBaseName::CMP => execute_cmp(cpu, opcode, operand),
        OpcodeBaseName::CPX => execute_cpx(cpu, opcode, operand),
        OpcodeBaseName::CPY => execute_cpy(cpu, opcode, operand),
        OpcodeBaseName::DEC => execute_dec(cpu, opcode, operand),
        OpcodeBaseName::DEX => execute_dex(cpu, opcode, operand),
        OpcodeBaseName::DEY => execute_dey(cpu, opcode, operand),
        OpcodeBaseName::EOR => execute_eor(cpu, opcode, operand),
        OpcodeBaseName::INC => execute_inc(cpu, opcode, operand),
        OpcodeBaseName::INX => execute_inx(cpu, opcode, operand),
        OpcodeBaseName::INY => execute_iny(cpu, opcode, operand),
        OpcodeBaseName::LSR => execute_lsr(cpu, opcode, operand),
        OpcodeBaseName::ORA => execute_ora(cpu, opcode, operand),
        OpcodeBaseName::ROL => execute_rol(cpu, opcode, operand),
        OpcodeBaseName::ROR => execute_ror(cpu, opcode, operand),
        OpcodeBaseName::SBC => execute_sbc(cpu, opcode, operand),
        OpcodeBaseName::PHA => execute_pha(cpu, opcode, operand),
        OpcodeBaseName::PHP => execute_php(cpu, opcode, operand),
        OpcodeBaseName::PLA => execute_pla(cpu, opcode, operand),
        OpcodeBaseName::PLP => execute_plp(cpu, opcode, operand),
        OpcodeBaseName::JMP => execute_jmp(cpu, opcode, operand),
        OpcodeBaseName::JSR => execute_jsr(cpu, opcode, operand),
        OpcodeBaseName::RTS => execute_rts(cpu, opcode, operand),
        OpcodeBaseName::RTI => execute_rti(cpu, opcode, operand),
        OpcodeBaseName::BCC => execute_bcc(cpu, opcode, operand),
        OpcodeBaseName::BCS => execute_bcs(cpu, opcode, operand),
        OpcodeBaseName::BEQ => execute_beq(cpu, opcode, operand),
        OpcodeBaseName::BMI => execute_bmi(cpu, opcode, operand),
        OpcodeBaseName::BNE => execute_bne(cpu, opcode, operand),
        OpcodeBaseName::BPL => execute_bpl(cpu, opcode, operand),
        OpcodeBaseName::BVC => execute_bvc(cpu, opcode, operand),
        OpcodeBaseName::BVS => execute_bvs(cpu, opcode, operand),
        OpcodeBaseName::CLC => execute_clc(cpu, opcode, operand),
        OpcodeBaseName::CLD => execute_cld(cpu, opcode, operand),
        OpcodeBaseName::CLI => execute_cli(cpu, opcode, operand),
        OpcodeBaseName::CLV => execute_clv(cpu, opcode, operand),
        OpcodeBaseName::SEC => execute_sec(cpu, opcode, operand),
        OpcodeBaseName::SED => execute_sed(cpu, opcode, operand),
        OpcodeBaseName::SEI => execute_sei(cpu, opcode, operand),
        OpcodeBaseName::BRK => execute_brk(cpu, opcode, operand),
        OpcodeBaseName::NOP => {}

        OpcodeBaseName::NOPD => {}
        OpcodeBaseName::NOPI => {}
        OpcodeBaseName::LAX => execute_lax(cpu, opcode, operand),
        OpcodeBaseName::SAX => execute_sax(cpu, opcode, operand),
        OpcodeBaseName::DCP => execute_dcp(cpu, opcode, operand),
        OpcodeBaseName::ISB => execute_isb(cpu, opcode, operand),
        OpcodeBaseName::SLO => execute_slo(cpu, opcode, operand),
        OpcodeBaseName::RLA => execute_rla(cpu, opcode, operand),
        OpcodeBaseName::SRE => execute_sre(cpu, opcode, operand),
        OpcodeBaseName::RRA => execute_rra(cpu, opcode, operand),
    }
}

fn execute_lda<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    let value = if opcode.addressing == Addressing::Immediate {
        operand as Byte
    } else {
        cpu.read_byte(operand)
    };
    let register = cpu.get_register();
    register.set_a(value);
    register.set_zn_by(value);
}

fn execute_ldx<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    let value = if opcode.addressing == Addressing::Immediate {
        operand as Byte
    } else {
        cpu.read_byte(operand)
    };
    let register = cpu.get_register();
    register.set_x(value);
    register.set_zn_by(value);
}
fn execute_ldy<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    let value = if opcode.addressing == Addressing::Immediate {
        operand as Byte
    } else {
        cpu.read_byte(operand)
    };
    let register = cpu.get_register();
    register.set_y(value);
    register.set_zn_by(value);
}
fn execute_sta<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    let value = cpu.get_register().get_a();
    cpu.write(operand, value);
}
fn execute_stx<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    let value = cpu.get_register().get_x();
    cpu.write(operand, value);
}
fn execute_sty<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    let value = cpu.get_register().get_y();
    cpu.write(operand, value);
}
fn execute_tax<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    let value = cpu.get_register().get_a();
    let register = cpu.get_register();
    register.set_x(value);
    register.set_zn_by(value);
}
fn execute_tay<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    let value = cpu.get_register().get_a();
    let register = cpu.get_register();
    register.set_y(value);
    register.set_zn_by(value);
}
fn execute_tsx<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    let value = cpu.get_register().get_s();
    let register = cpu.get_register();
    register.set_x(value);
    register.set_zn_by(value);
}
fn execute_txa<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    let value = cpu.get_register().get_x();
    let register = cpu.get_register();
    register.set_a(value);
    register.set_zn_by(value);
}
fn execute_txs<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    let value = cpu.get_register().get_x();
    let register = cpu.get_register();
    register.set_s(value);
}
fn execute_tya<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    let value = cpu.get_register().get_y();
    let register = cpu.get_register();
    register.set_a(value);
    register.set_zn_by(value);
}

fn execute_adc<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    let value = if opcode.addressing == Addressing::Immediate {
        operand as Byte
    } else {
        cpu.read_byte(operand)
    };
    cpu.get_register().add_a(value);
}
fn execute_and<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    let value = if opcode.addressing == Addressing::Immediate {
        operand as Byte
    } else {
        cpu.read_byte(operand)
    };
    let a = cpu.get_register().get_a();
    let result = a & value;
    let register = cpu.get_register();
    register.set_a(result);
    register.set_zn_by(result);
}
fn execute_asl<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    if opcode.addressing == Addressing::Accumulator {
        cpu.get_register().left_shift_a()
    } else {
        let value = cpu.read_byte(operand);
        let result = cpu.get_register().set_flag_by_left_shift(value);
        cpu.write(operand, result);
    };
}
fn execute_bit<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    let value = cpu.read_byte(operand);
    let register = cpu.get_register();
    let result = register.get_a() & value;
    if result == 0 {
        register.set_z()
    } else {
        register.clear_z();
    }
    if value & 0b0100_0000 != 0 {
        register.set_v();
    } else {
        register.clear_v();
    }
    if value & 0b1000_0000 != 0 {
        register.set_n();
    } else {
        register.clear_n();
    }
}
fn execute_cmp<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    let value = if opcode.addressing == Addressing::Immediate {
        operand as Byte
    } else {
        cpu.read_byte(operand)
    };
    let register = cpu.get_register();
    let a = register.get_a();
    register.cmp(a, value);
}
fn execute_cpx<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    let value = if opcode.addressing == Addressing::Immediate {
        operand as Byte
    } else {
        cpu.read_byte(operand)
    };
    let register = cpu.get_register();
    let x = register.get_x();
    register.cmp(x, value);
}
fn execute_cpy<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    let value = if opcode.addressing == Addressing::Immediate {
        operand as Byte
    } else {
        cpu.read_byte(operand)
    };
    let register = cpu.get_register();
    let y = register.get_y();
    register.cmp(y, value);
}
fn execute_dec<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    let value = cpu.read_byte(operand);
    let result = value.wrapping_sub(1);
    cpu.write(operand, result);
    cpu.get_register().set_zn_by(result);
}
fn execute_dex<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    let register = cpu.get_register();
    let value = register.get_x();
    let result = value.wrapping_sub(1);
    register.set_x(result);
    register.set_zn_by(result);
}
fn execute_dey<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    let register = cpu.get_register();
    let value = register.get_y();
    let result = value.wrapping_sub(1);
    register.set_y(result);
    register.set_zn_by(result);
}
fn execute_eor<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    let value = if opcode.addressing == Addressing::Immediate {
        operand as Byte
    } else {
        cpu.read_byte(operand)
    };
    let a = cpu.get_register().get_a();
    let result = a ^ value;
    let register = cpu.get_register();
    register.set_a(result);
    register.set_zn_by(result);
}
fn execute_inc<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    let value = cpu.read_byte(operand);
    let result = value.wrapping_add(1);
    cpu.write(operand, result);
    cpu.get_register().set_zn_by(result);
}
fn execute_inx<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    let register = cpu.get_register();
    let value = register.get_x();
    let result = value.wrapping_add(1);
    register.set_x(result);
    register.set_zn_by(result);
}
fn execute_iny<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    let register = cpu.get_register();
    let value = register.get_y();
    let result = value.wrapping_add(1);
    register.set_y(result);
    register.set_zn_by(result);
}
fn execute_lsr<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    if opcode.addressing == Addressing::Accumulator {
        cpu.get_register().right_shift_a()
    } else {
        let value = cpu.read_byte(operand);
        let result = cpu.get_register().set_flag_by_right_shift(value);
        cpu.write(operand, result);
    };
}
fn execute_ora<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    let value = if opcode.addressing == Addressing::Immediate {
        operand as Byte
    } else {
        cpu.read_byte(operand)
    };
    let a = cpu.get_register().get_a();
    let result = a | value;
    let register = cpu.get_register();
    register.set_a(result);
    register.set_zn_by(result);
}
fn execute_rol<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    if opcode.addressing == Addressing::Accumulator {
        cpu.get_register().left_rotate_a()
    } else {
        let value = cpu.read_byte(operand);
        let result = cpu.get_register().set_flag_by_left_rotate(value);
        cpu.write(operand, result);
    };
}
fn execute_ror<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    if opcode.addressing == Addressing::Accumulator {
        cpu.get_register().right_rotate_a()
    } else {
        let value = cpu.read_byte(operand);
        let result = cpu.get_register().set_flag_by_right_rotate(value);
        cpu.write(operand, result);
    };
}
fn execute_sbc<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    let value = if opcode.addressing == Addressing::Immediate {
        operand as Byte
    } else {
        cpu.read_byte(operand)
    };
    cpu.get_register().sub_a(value);
}

fn execute_pha<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    let a = cpu.get_register().get_a();
    cpu.push(a);
}
fn execute_php<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    cpu.get_register().set_b();
    cpu.get_register().set_r();
    cpu.push_status();
}
fn execute_pla<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    let value = cpu.pop();
    let register = cpu.get_register();
    register.set_a(value);
    register.set_zn_by(value);
}
fn execute_plp<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    cpu.get_register().set_r();
    cpu.pop_status();
}

fn execute_jmp<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    cpu.get_register().set_pc(operand);
}
fn execute_jsr<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    cpu.get_register().decrement_pc_byte();
    cpu.push_pc();
    cpu.get_register().set_pc(operand);
}
fn execute_rts<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    cpu.pop_pc();
    cpu.get_register().increment_pc_byte();
}
fn execute_rti<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    cpu.pop_status();
    cpu.pop_pc();
    cpu.get_register().set_r();
}

fn execute_bcc<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    if !cpu.get_register().get_c() {
        cpu.branch(operand);
    }
}
fn execute_bcs<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    if cpu.get_register().get_c() {
        cpu.branch(operand);
    }
}
fn execute_beq<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    if cpu.get_register().get_z() {
        cpu.branch(operand);
    }
}
fn execute_bmi<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    if cpu.get_register().get_n() {
        cpu.branch(operand);
    }
}
fn execute_bne<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    if !cpu.get_register().get_z() {
        cpu.branch(operand);
    }
}
fn execute_bpl<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    if !cpu.get_register().get_n() {
        cpu.branch(operand);
    }
}
fn execute_bvc<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    if !cpu.get_register().get_v() {
        cpu.branch(operand);
    }
}
fn execute_bvs<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    if cpu.get_register().get_v() {
        cpu.branch(operand);
    }
}

fn execute_clc<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    cpu.get_register().clear_c();
}
fn execute_cld<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    cpu.get_register().clear_d();
}
fn execute_cli<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    cpu.get_register().clear_i();
}
fn execute_clv<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    cpu.get_register().clear_v();
}
fn execute_sec<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    cpu.get_register().set_c();
}
fn execute_sed<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    cpu.get_register().set_d();
}
fn execute_sei<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    cpu.get_register().set_i();
}

fn execute_brk<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, _operand: Word) -> () {
    let interrupt = cpu.get_register().get_i();
    cpu.get_register().increment_pc_byte();
    cpu.push_pc();
    cpu.get_register().set_b();
    cpu.get_register().set_r();
    cpu.push_status();
    cpu.get_register().set_i();
    if interrupt {
        cpu.set_pc_by_irq();
    }
    cpu.get_register().decrement_pc_byte();
}

fn execute_lax<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    execute_lda(cpu, opcode, operand);
    execute_ldx(cpu, opcode, operand);
}
fn execute_sax<P: PPU>(cpu: &mut CPU<P>, _opcode: &Opcode, operand: Word) -> () {
    let register = cpu.get_register();
    let a = register.get_a();
    let x = register.get_x();
    let value = a & x;
    cpu.write(operand, value);
}
fn execute_isb<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    execute_inc(cpu, opcode, operand);
    execute_sbc(cpu, opcode, operand);
}
fn execute_dcp<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    execute_dec(cpu, opcode, operand);
    execute_cmp(cpu, opcode, operand);
}
fn execute_slo<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    execute_asl(cpu, opcode, operand);
    execute_ora(cpu, opcode, operand);
}
fn execute_rla<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    execute_rol(cpu, opcode, operand);
    execute_and(cpu, opcode, operand);
}
fn execute_sre<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    execute_lsr(cpu, opcode, operand);
    execute_eor(cpu, opcode, operand);
}
fn execute_rra<P: PPU>(cpu: &mut CPU<P>, opcode: &Opcode, operand: Word) -> () {
    execute_ror(cpu, opcode, operand);
    execute_adc(cpu, opcode, operand);
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        controller::Controller,
        cpu::CPUBus,
        interrupt,
        ppu::{PPUBus, PPUImpl},
        rom::ROM,
    };

    use super::*;

    fn prepare_cpu() -> CPU<PPUImpl> {
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
            ROM::new(vec![]),
            wram.clone(),
            ppu.clone(),
            controller.clone(),
            dma.clone(),
        );
        CPU::new(cpu_bus, interrupt.clone())
    }

    #[test]
    fn test_lda() {
        let mut cpu = prepare_cpu();
        let opcode = Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::Immediate,
            cycle: 2,
        };
        execute(&mut cpu, &opcode, 0x00FF);
        let register = cpu.get_register();
        assert_eq!(register.get_a(), 0xFF);
        assert_eq!(register.get_z(), false);
        assert_eq!(register.get_n(), true);

        execute(&mut cpu, &opcode, 0x0000);
        let register = cpu.get_register();
        assert_eq!(register.get_a(), 0x00);
        assert_eq!(register.get_z(), true);
        assert_eq!(register.get_n(), false);

        let opcode = Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::Absolute,
            cycle: 4,
        };
        cpu.write(0x0001, 0x80);
        execute(&mut cpu, &opcode, 0x0001);
        let register = cpu.get_register();
        assert_eq!(register.get_a(), 0x80);
        assert_eq!(register.get_z(), false);
        assert_eq!(register.get_n(), true);
    }

    #[test]
    fn test_ldx() {
        let mut cpu = prepare_cpu();
        let opcode = Opcode {
            base_name: OpcodeBaseName::LDX,
            addressing: Addressing::Immediate,
            cycle: 2,
        };
        execute(&mut cpu, &opcode, 0x00FF);
        let register = cpu.get_register();
        assert_eq!(register.get_x(), 0xFF);
        assert_eq!(register.get_z(), false);
        assert_eq!(register.get_n(), true);

        execute(&mut cpu, &opcode, 0x0000);
        let register = cpu.get_register();
        assert_eq!(register.get_x(), 0x00);
        assert_eq!(register.get_z(), true);
        assert_eq!(register.get_n(), false);

        let opcode = Opcode {
            base_name: OpcodeBaseName::LDX,
            addressing: Addressing::Absolute,
            cycle: 4,
        };
        cpu.write(0x0001, 0x80);
        execute(&mut cpu, &opcode, 0x0001);
        let register = cpu.get_register();
        assert_eq!(register.get_x(), 0x80);
        assert_eq!(register.get_z(), false);
        assert_eq!(register.get_n(), true);
    }

    #[test]
    fn test_ldy() {
        let mut cpu = prepare_cpu();
        let opcode = Opcode {
            base_name: OpcodeBaseName::LDY,
            addressing: Addressing::Immediate,
            cycle: 2,
        };
        execute(&mut cpu, &opcode, 0x00FF);
        let register = cpu.get_register();
        assert_eq!(register.get_y(), 0xFF);
        assert_eq!(register.get_z(), false);
        assert_eq!(register.get_n(), true);

        execute(&mut cpu, &opcode, 0x0000);
        let register = cpu.get_register();
        assert_eq!(register.get_y(), 0x00);
        assert_eq!(register.get_z(), true);
        assert_eq!(register.get_n(), false);

        let opcode = Opcode {
            base_name: OpcodeBaseName::LDY,
            addressing: Addressing::Absolute,
            cycle: 4,
        };
        cpu.write(0x0001, 0x80);
        execute(&mut cpu, &opcode, 0x0001);
        let register = cpu.get_register();
        assert_eq!(register.get_y(), 0x80);
        assert_eq!(register.get_z(), false);
        assert_eq!(register.get_n(), true);
    }

    #[test]
    fn test_sta() {
        let mut cpu = prepare_cpu();
        let opcode = Opcode {
            base_name: OpcodeBaseName::STA,
            addressing: Addressing::Absolute,
            cycle: 4,
        };
        let register = cpu.get_register();
        register.set_a(0xFF);
        execute(&mut cpu, &opcode, 0x0001);
        assert_eq!(cpu.read_byte(0x0001), 0xFF);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_n(), false);
        }
    }

    #[test]
    fn test_stx() {
        let mut cpu = prepare_cpu();
        let opcode = Opcode {
            base_name: OpcodeBaseName::STX,
            addressing: Addressing::Absolute,
            cycle: 4,
        };
        let register = cpu.get_register();
        register.set_x(0xFF);
        execute(&mut cpu, &opcode, 0x0001);
        assert_eq!(cpu.read_byte(0x0001), 0xFF);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_n(), false);
        }
    }

    #[test]
    fn test_sty() {
        let mut cpu = prepare_cpu();
        let opcode = Opcode {
            base_name: OpcodeBaseName::STY,
            addressing: Addressing::Absolute,
            cycle: 4,
        };
        let register = cpu.get_register();
        register.set_y(0xFF);
        execute(&mut cpu, &opcode, 0x0001);
        assert_eq!(cpu.read_byte(0x0001), 0xFF);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_n(), false);
        }
    }

    #[test]
    fn test_tax() {
        let mut cpu = prepare_cpu();
        {
            let register = cpu.get_register();
            register.set_a(0x00);
        }
        let opcode = Opcode {
            base_name: OpcodeBaseName::TAX,
            addressing: Addressing::Implied,
            cycle: 2,
        };
        execute(&mut cpu, &opcode, 0x0000);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_x(), 0x00);
            assert_eq!(register.get_z(), true);
            assert_eq!(register.get_n(), false);
        }

        {
            let register = cpu.get_register();
            register.set_a(0xFF);
        }
        execute(&mut cpu, &opcode, 0x0000);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_x(), 0xFF);
            assert_eq!(register.get_z(), false);
            assert_eq!(register.get_n(), true);
        }
    }

    #[test]
    fn test_tay() {
        let mut cpu = prepare_cpu();
        {
            let register = cpu.get_register();
            register.set_a(0x00);
        }
        let opcode = Opcode {
            base_name: OpcodeBaseName::TAY,
            addressing: Addressing::Implied,
            cycle: 2,
        };
        execute(&mut cpu, &opcode, 0x0000);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_y(), 0x00);
            assert_eq!(register.get_z(), true);
            assert_eq!(register.get_n(), false);
        }

        {
            let register = cpu.get_register();
            register.set_a(0xFF);
        }
        execute(&mut cpu, &opcode, 0x0000);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_y(), 0xFF);
            assert_eq!(register.get_z(), false);
            assert_eq!(register.get_n(), true);
        }
    }

    #[test]
    fn test_tsx() {
        let mut cpu = prepare_cpu();
        {
            let register = cpu.get_register();
            register.set_s(0x00);
        }
        let opcode = Opcode {
            base_name: OpcodeBaseName::TSX,
            addressing: Addressing::Implied,
            cycle: 2,
        };
        execute(&mut cpu, &opcode, 0x0000);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_x(), 0x00);
            assert_eq!(register.get_z(), true);
            assert_eq!(register.get_n(), false);
        }

        {
            let register = cpu.get_register();
            register.set_s(0xFF);
        }
        execute(&mut cpu, &opcode, 0x0000);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_x(), 0xFF);
            assert_eq!(register.get_z(), false);
            assert_eq!(register.get_n(), true);
        }
    }

    #[test]
    fn test_txa() {
        let mut cpu = prepare_cpu();
        {
            let register = cpu.get_register();
            register.set_x(0x00);
        }
        let opcode = Opcode {
            base_name: OpcodeBaseName::TXA,
            addressing: Addressing::Implied,
            cycle: 2,
        };
        execute(&mut cpu, &opcode, 0x0000);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_a(), 0x00);
            assert_eq!(register.get_z(), true);
            assert_eq!(register.get_n(), false);
        }

        {
            let register = cpu.get_register();
            register.set_x(0xFF);
        }
        execute(&mut cpu, &opcode, 0x0000);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_a(), 0xFF);
            assert_eq!(register.get_z(), false);
            assert_eq!(register.get_n(), true);
        }
    }

    #[test]
    fn test_txs() {
        let mut cpu = prepare_cpu();
        {
            let register = cpu.get_register();
            register.set_x(0xFF);
        }
        let opcode = Opcode {
            base_name: OpcodeBaseName::TXS,
            addressing: Addressing::Implied,
            cycle: 2,
        };
        execute(&mut cpu, &opcode, 0x0000);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_s(), 0xFF);
            assert_eq!(register.get_n(), false);
        }
    }

    #[test]
    fn test_tya() {
        let mut cpu = prepare_cpu();
        {
            let register = cpu.get_register();
            register.set_y(0x00);
        }
        let opcode = Opcode {
            base_name: OpcodeBaseName::TYA,
            addressing: Addressing::Implied,
            cycle: 2,
        };
        execute(&mut cpu, &opcode, 0x0000);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_a(), 0x00);
            assert_eq!(register.get_z(), true);
            assert_eq!(register.get_n(), false);
        }

        {
            let register = cpu.get_register();
            register.set_y(0xFF);
        }
        execute(&mut cpu, &opcode, 0x0000);
        {
            let register = cpu.get_register();
            assert_eq!(register.get_a(), 0xFF);
            assert_eq!(register.get_z(), false);
            assert_eq!(register.get_n(), true);
        }
    }
}
