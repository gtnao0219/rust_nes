use crate::{Byte, Word};

#[derive(Debug)]
pub struct CPURegister {
    a: Byte,
    x: Byte,
    y: Byte,
    s: Byte,
    p: CPUStatusRegister,
    pc: Word,
}

#[derive(Debug)]
pub struct CPUStatusRegister {
    c: bool,
    z: bool,
    i: bool,
    d: bool,
    b: bool,
    r: bool,
    v: bool,
    n: bool,
}

// TODO: default check
impl Default for CPURegister {
    fn default() -> Self {
        Self {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            s: 0xFD,
            p: CPUStatusRegister::default(),
            pc: 0x8000,
        }
    }
}

impl Default for CPUStatusRegister {
    fn default() -> Self {
        Self {
            c: false,
            z: false,
            i: true,
            d: false,
            b: false,
            r: true,
            v: false,
            n: false,
        }
    }
}

impl CPURegister {
    pub fn get_a(&self) -> Byte {
        self.a
    }
    pub fn set_a(&mut self, a: Byte) {
        self.a = a;
    }
    pub fn get_x(&self) -> Byte {
        self.x
    }
    pub fn set_x(&mut self, x: Byte) {
        self.x = x;
    }
    pub fn get_y(&self) -> Byte {
        self.y
    }
    pub fn set_y(&mut self, y: Byte) {
        self.y = y;
    }
    pub fn get_s(&self) -> Byte {
        self.s
    }
    pub fn set_s(&mut self, s: Byte) {
        self.s = s;
    }
    pub fn increment_s(&mut self) {
        self.s = self.s.wrapping_add(1);
    }
    pub fn decrement_s(&mut self) {
        self.s = self.s.wrapping_sub(1);
    }
    pub fn get_pc(&self) -> Word {
        self.pc
    }
    pub fn set_pc(&mut self, pc: Word) {
        self.pc = pc;
    }
    pub fn increment_pc_byte(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }
    pub fn increment_pc_word(&mut self) {
        self.pc = self.pc.wrapping_add(2);
    }
    pub fn decrement_pc_byte(&mut self) {
        self.pc = self.pc.wrapping_sub(1);
    }
    pub fn stack_address(&self) -> Word {
        0x0100 | self.s as Word
    }
    pub fn get_p(&self) -> Byte {
        let mut p = 0x00;
        if self.p.c {
            p |= 0x01;
        }
        if self.p.z {
            p |= 0x02;
        }
        if self.p.i {
            p |= 0x04;
        }
        if self.p.d {
            p |= 0x08;
        }
        if self.p.b {
            p |= 0x10;
        }
        if self.p.r {
            p |= 0x20;
        }
        if self.p.v {
            p |= 0x40;
        }
        if self.p.n {
            p |= 0x80;
        }
        p
    }
    pub fn set_p(&mut self, p: Byte) {
        self.p.c = (p & 0x01) != 0;
        self.p.z = (p & 0x02) != 0;
        self.p.i = (p & 0x04) != 0;
        self.p.d = (p & 0x08) != 0;
        self.p.b = (p & 0x10) != 0;
        self.p.r = (p & 0x20) != 0;
        self.p.v = (p & 0x40) != 0;
        self.p.n = (p & 0x80) != 0;
    }

    pub fn get_c(&self) -> bool {
        self.p.c
    }
    pub fn clear_c(&mut self) {
        self.p.c = false;
    }
    pub fn set_c(&mut self) {
        self.p.c = true;
    }
    pub fn get_z(&self) -> bool {
        self.p.z
    }
    pub fn clear_z(&mut self) {
        self.p.z = false;
    }
    pub fn set_z(&mut self) {
        self.p.z = true;
    }
    pub fn get_i(&self) -> bool {
        self.p.i
    }
    pub fn clear_i(&mut self) {
        self.p.i = false;
    }
    pub fn set_i(&mut self) {
        self.p.i = true;
    }
    pub fn get_d(&self) -> bool {
        self.p.d
    }
    pub fn clear_d(&mut self) {
        self.p.d = false;
    }
    pub fn set_d(&mut self) {
        self.p.d = true;
    }
    pub fn get_b(&self) -> bool {
        self.p.b
    }
    pub fn clear_b(&mut self) {
        self.p.b = false;
    }
    pub fn set_b(&mut self) {
        self.p.b = true;
    }
    pub fn get_r(&self) -> bool {
        self.p.r
    }
    pub fn clear_r(&mut self) {
        self.p.r = false;
    }
    pub fn set_r(&mut self) {
        self.p.r = true;
    }
    pub fn get_v(&self) -> bool {
        self.p.v
    }
    pub fn clear_v(&mut self) {
        self.p.v = false;
    }
    pub fn set_v(&mut self) {
        self.p.v = true;
    }
    pub fn get_n(&self) -> bool {
        self.p.n
    }
    pub fn clear_n(&mut self) {
        self.p.n = false;
    }
    pub fn set_n(&mut self) {
        self.p.n = true;
    }

    // TODO:
    pub fn add_a(&mut self, value: Byte) {
        let sum = self.a as u16 + value as u16 + self.p.c as u16;
        let overflow = (self.a ^ value) & 0x80 == 0 && (self.a ^ sum as u8) & 0x80 != 0;
        self.p.v = overflow;
        self.p.c = sum > 0xFF;
        self.a = sum as u8;
        self.set_zn_by(self.a);
    }
    pub fn sub_a(&mut self, value: Byte) {
        let diff = self.a as i16 - value as i16 - !self.p.c as i16;
        let overflow = (self.a ^ value) & 0x80 != 0 && (self.a ^ diff as u8) & 0x80 != 0;
        self.p.v = overflow;
        self.p.c = diff >= 0;
        self.a = diff as u8;
        self.set_zn_by(self.a);
    }
    pub fn left_shift_a(&mut self) {
        let result = self.set_flag_by_left_shift(self.a);
        self.a = result;
    }
    pub fn right_shift_a(&mut self) {
        let result = self.set_flag_by_right_shift(self.a);
        self.a = result;
    }
    pub fn set_flag_by_left_shift(&mut self, value: Byte) -> Byte {
        self.p.c = (value & 0x80) != 0;
        let result = value << 1;
        self.set_zn_by(result);
        result
    }
    pub fn set_flag_by_right_shift(&mut self, value: Byte) -> Byte {
        self.p.c = (value & 0x01) != 0;
        let result = value >> 1;
        self.set_zn_by(result);
        result
    }
    pub fn left_rotate_a(&mut self) {
        let result = self.set_flag_by_left_rotate(self.a);
        self.a = result;
    }
    pub fn right_rotate_a(&mut self) {
        let result = self.set_flag_by_right_rotate(self.a);
        self.a = result;
    }
    pub fn set_flag_by_left_rotate(&mut self, value: Byte) -> Byte {
        let c = self.p.c as u8;
        self.p.c = (value & 0x80) != 0;
        let result = (value << 1) | c;
        self.set_zn_by(result);
        result
    }
    pub fn set_flag_by_right_rotate(&mut self, value: Byte) -> Byte {
        let c = self.p.c as u8;
        self.p.c = (value & 0x01) != 0;
        let result = (value >> 1) | (c << 7);
        self.set_zn_by(result);
        result
    }
    pub fn cmp(&mut self, a: Byte, b: Byte) {
        let diff = a as i16 - b as i16;
        self.p.c = diff >= 0;
        self.set_zn_by(diff as u8);
    }
    pub fn set_zn_by(&mut self, value: Byte) {
        self.p.z = value == 0;
        self.p.n = (value & 0x80) != 0;
    }
}
