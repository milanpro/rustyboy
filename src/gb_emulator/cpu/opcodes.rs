use crate::gb_emulator::cpu::Z80CPU;
use crate::gb_emulator::registers::Flag;
use crate::gb_emulator::utils::U16Ext;

pub trait Opcodes {
    fn add(&mut self, val: u8);
    fn add_16(&mut self, lhs: u16, rhs: u16) -> u16;
    fn adc(&mut self, val: u8);

    fn sub(&mut self, val: u8);
    fn sbc(&mut self, val: u8);

    fn and(&mut self, val: u8);
    fn xor(&mut self, val: u8);
    fn or(&mut self, val: u8);

    fn cp(&mut self, val: u8);

    fn inc(&mut self, val: u8) -> u8;
    fn dec(&mut self, val: u8) -> u8;

    fn rl(&mut self, val: u8) -> u8;
    fn rlc(&mut self, val: u8) -> u8;
    fn rr(&mut self, val: u8) -> u8;
    fn rrc(&mut self, val: u8) -> u8;

    fn daa(&mut self);
    fn jr(&mut self);
}

impl Opcodes for Z80CPU {
    fn add(&mut self, val: u8) {
        let res = (self.r.a as u16) + (val as u16);

        self.r.set_flag(Flag::Z, res.lo() == 0);
        self.r
            .set_flag(Flag::H, ((self.r.a & 0x0F) + (val & 0x0F)) & 0x10 == 0x10);
        self.r.set_flag(Flag::C, res > 0xFF);
        self.r.set_flag(Flag::N, false);

        self.r.a = res.lo();
    }

    fn adc(&mut self, val: u8) {
        let mut res = (self.r.a as u16) + (val as u16);

        if self.r.get_flag(Flag::C) {
            res += 1;
        }

        self.r.set_flag(Flag::Z, res.lo() == 0);
        self.r
            .set_flag(Flag::H, ((self.r.a & 0x0F) + (val & 0x0F)) & 0x10 == 0x10);
        self.r.set_flag(Flag::C, res > 0xFF);
        self.r.set_flag(Flag::N, false);

        self.r.a = res.lo();
    }

    fn sub(&mut self, val: u8) {
        self.cp(val);
        self.r.a -= val;
    }

    fn sbc(&mut self, val: u8) {
        let res = self.r.a - val - 1;

        self.r
            .set_flag(Flag::H, ((self.r.a & 0xF) as i8 - (val & 0xF) as i8) < 0);
        self.r
            .set_flag(Flag::C, ((self.r.a as i16) - (val as i16) - 1) < 0);
        self.r.set_flag(Flag::N, true);
        self.r.set_flag(Flag::Z, res == 0);

        self.r.a = res;
    }

    fn and(&mut self, val: u8) {
        let res = self.r.a & val;

        self.r.set_flag(Flag::Z, res == 0);
        self.r.set_flag(Flag::H, true);
        self.r.set_flag(Flag::N, false);
        self.r.set_flag(Flag::C, false);

        self.r.a = res;
    }

    fn xor(&mut self, val: u8) {
        let res = self.r.a ^ val;

        self.r.set_flag(Flag::Z, res == 0);
        self.r.set_flag(Flag::H, false);
        self.r.set_flag(Flag::N, false);
        self.r.set_flag(Flag::C, false);

        self.r.a = res;
    }

    fn or(&mut self, val: u8) {
        let res = self.r.a | val;

        self.r.set_flag(Flag::Z, res == 0);
        self.r.set_flag(Flag::H, false);
        self.r.set_flag(Flag::N, false);
        self.r.set_flag(Flag::C, false);

        self.r.a = res;
    }

    fn cp(&mut self, val: u8) {
        self.r.set_flag(Flag::Z, self.r.a == val);
        self.r.set_flag(Flag::H, (self.r.a & 0xF) < (val & 0xF));
        self.r.set_flag(Flag::N, true);
        self.r.set_flag(Flag::C, self.r.a < val);
    }

    fn inc(&mut self, val: u8) -> u8 {
        let res = val.wrapping_add(1);

        self.r.set_flag(Flag::Z, res == 0);
        self.r.set_flag(Flag::H, (val & 0xF) == 0xF);
        self.r.set_flag(Flag::N, false);

        res
    }

    fn dec(&mut self, val: u8) -> u8 {
        let res = val.wrapping_rem(1);

        self.r.set_flag(Flag::Z, res == 0);
        self.r.set_flag(Flag::H, (res & 0xF) == 0xF);
        self.r.set_flag(Flag::N, true);

        res
    }

    fn rl(&mut self, val: u8) -> u8 {
        let carry = self.r.get_flag(Flag::C) as u8;
        self.r.set_flag(Flag::C, (val & (1<<7)) > 0x0);
        let mut res = val << 1;
        res += carry;
        self.r.set_flag(Flag::H, false);
        self.r.set_flag(Flag::N, false);
        self.r.set_flag(Flag::Z, res == 0);
        res
    }

    fn rlc(&mut self, val: u8) -> u8 {
        let (result, carry) = val.overflowing_shl(1);
        self.r.set_flag(Flag::C, carry);
        self.r.set_flag(Flag::H, false);
        self.r.set_flag(Flag::N, false);
        self.r.set_flag(Flag::Z, result == 0);
        result
    }

    fn rr(&mut self, val: u8) -> u8 {
        let carry = self.r.get_flag(Flag::C) as u8;
        self.r.set_flag(Flag::C, (val & 0x01) > 0x0);
        let mut res = val >> 1;
        res |= carry << 7;
        self.r.set_flag(Flag::H, false);
        self.r.set_flag(Flag::N, false);
        self.r.set_flag(Flag::Z, res == 0);
        res
    }

    fn rrc(&mut self, val: u8) -> u8 {
        let (result, carry) = val.overflowing_shr(1);
        self.r.set_flag(Flag::C, carry);
        self.r.set_flag(Flag::H, false);
        self.r.set_flag(Flag::N, false);
        self.r.set_flag(Flag::Z, result == 0);
        result
    }

    fn add_16(&mut self, lhs: u16, rhs: u16) -> u16 {
        let (result, carry) = lhs.overflowing_add(rhs);
        self.r.set_flag(Flag::C, carry);
        self.r.set_flag(Flag::H, ((lhs & 0x0fff) + (rhs & 0x0fff)) > 0x0fff);
        self.r.set_flag(Flag::N, false);
        result
    }

    fn daa(&mut self) {
        let mut a = self.r.a;
        let mut adjust = if self.r.get_flag(Flag::C) { 0x60 } else { 0x00 };
        if self.r.get_flag(Flag::H) { adjust |= 0x06; };
        if !self.r.get_flag(Flag::N) {
            if a & 0x0F > 0x09 { adjust |= 0x06; };
            if a > 0x99 { adjust |= 0x60; };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }

        self.r.set_flag(Flag::C, adjust >= 0x60);
        self.r.set_flag(Flag::H, false);
        self.r.set_flag(Flag::Z, a == 0);
        self.r.a = a;
    }

    fn jr(&mut self) {
        let n = self.fetch_byte() as i8;
        self.r.pc = ((self.r.pc as u32 as i32) + (n as i32)) as u16;
    }
}
