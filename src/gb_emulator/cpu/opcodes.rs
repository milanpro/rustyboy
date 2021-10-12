use crate::gb_emulator::cpu::Z80CPU;
use crate::gb_emulator::registers::Flag;
use crate::gb_emulator::utils::U16Ext;

pub trait Opcodes {
    fn add(&mut self, val: u8);
    fn adc(&mut self, val: u8);

    fn sub(&mut self, val: u8);
    fn sbc(&mut self, val: u8);

    fn and(&mut self, val: u8);
    fn xor(&mut self, val: u8);
    fn or(&mut self, val: u8);

    fn cp(&mut self, val: u8);

    fn inc(&mut self, val: u8) -> u8;
    fn dec(&mut self, val: u8) -> u8;
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
}
