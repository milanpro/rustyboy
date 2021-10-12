use crate::gb_emulator::registers::Flag;

use super::memory::MemoryBus;
use super::registers::Registers;
use super::utils::U16Ext;

pub struct Z80CPU {
    r: Registers,
    m: MemoryBus,
    halted: bool,
    ime: bool,
}

impl Z80CPU {
    pub fn new() -> Z80CPU {
        let r = Registers::new();
        let m = MemoryBus::new();
        Z80CPU {
            r,
            m,
            halted: false,
            ime: false,
        }
    }

    fn cycle(&mut self) -> u32 {
        if self.halted {
            1
        } else {
            self.interpret()
        }
    }

    fn reset(&mut self) {
        self.r = Registers::new();
        self.halted = false;
        self.ime = false;
    }

    fn fetch_byte(&mut self) -> u8 {
        let b = self.m.read_byte(self.r.pc);
        self.r.pc += 1;
        b
    }

    fn fetch_word(&mut self) -> u16 {
        let b = self.m.read_byte(self.r.pc);
        self.r.pc += 1;
        let w = (b as u16) << 8 + self.m.read_byte(self.r.pc);
        self.r.pc += 1;
        w
    }

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
        self.r
            .set_flag(Flag::H, ((self.r.a & 0xF) as i8 - (val & 0xF) as i8) < 0);
        self.r
            .set_flag(Flag::C, ((self.r.a as i16) - (val as i16) - 1) < 0);
        self.r.set_flag(Flag::N, true);
        self.r.a -= val;
        self.r.a -= 1;
        self.r.set_flag(Flag::Z, self.r.a == 0);
    }

    fn and(&mut self, val: u8) {
        let res = self.r.a & val;

        self.r.set_flag(Flag::Z, self.r.a == 0);
        self.r.set_flag(Flag::H, true);
        self.r.set_flag(Flag::N, false);
        self.r.set_flag(Flag::C, false);

        self.r.a = res;
    }

    fn or(&mut self, val: u8) {
        let res = self.r.a | val;

        self.r.set_flag(Flag::Z, self.r.a == 0);
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

    // fetch and run the next instruction, returns the length of the ran instruction
    fn interpret(&mut self) -> u32 {
        match self.fetch_byte() {
            0x00 => 1,
            0x01 => {
                let d = self.fetch_word();
                self.r.set_bc(d);
                3
            }
            0x02 => {
                self.m.write_byte(self.r.get_bc(), self.r.a);
                2
            }
            0x03 => {
                self.r.set_bc(self.r.get_bc().wrapping_add(1));
                2
            }
            0x04 => {
                self.r.b = self.r.b.wrapping_add(1);
                1
            }
            0x05 => {
                self.r.b = self.r.b.wrapping_rem(1);
                1
            }
            0x06 => {
                self.r.b = self.fetch_byte();
                2
            }
            0x07 => {
                self.r.a = self.r.a.rotate_left(1);
                1
            }
            0x08 => {
                let addr = self.fetch_word();
                self.m.write_word(addr, self.r.sp);
                3
            }
            0x09 => {
                self.r.set_hl(self.r.get_hl() + self.r.get_bc());
                1
            }
            0x0A => {
                let addr = self.r.get_bc();
                self.r.a = self.m.read_byte(addr);
                1
            }
            0x0B => {
                self.r.set_bc(self.r.get_bc().wrapping_rem(1));
                1
            }
            0x0C => {
                self.r.c = self.r.c.wrapping_add(1);
                1
            }
            0x0D => {
                self.r.c = self.r.c.wrapping_rem(1);
                1
            }
            0x0E => {
                self.r.c = self.fetch_byte();
                2
            }
            0x0F => {
                self.r.a = self.r.a.rotate_right(1);
                1
            }
            0x40 => 1,
            0x41 => {
                self.r.b = self.r.c;
                1
            }
            0x42 => {
                self.r.b = self.r.d;
                1
            }
            0x43 => {
                self.r.b = self.r.e;
                1
            }
            0x44 => {
                self.r.b = self.r.h;
                1
            }
            0x45 => {
                self.r.b = self.r.l;
                1
            }
            0x47 => {
                self.r.b = self.r.a;
                1
            }
            0x48 => {
                self.r.c = self.r.b;
                1
            }
            0x49 => 1,
            0x4A => {
                self.r.c = self.r.d;
                1
            }
            0x4B => {
                self.r.c = self.r.e;
                1
            }
            0x4C => {
                self.r.c = self.r.h;
                1
            }
            0x4D => {
                self.r.c = self.r.l;
                1
            }
            0x4F => {
                self.r.c = self.r.a;
                1
            }
            0x50 => {
                self.r.d = self.r.b;
                1
            }
            0x51 => {
                self.r.d = self.r.c;
                1
            }
            0x52 => 1,
            0x53 => {
                self.r.d = self.r.e;
                1
            }
            0x54 => {
                self.r.d = self.r.h;
                1
            }
            0x55 => {
                self.r.d = self.r.l;
                1
            }
            0x57 => {
                self.r.d = self.r.a;
                1
            }
            0x58 => {
                self.r.e = self.r.b;
                1
            }
            0x59 => {
                self.r.e = self.r.c;
                1
            }
            0x5A => {
                self.r.e = self.r.d;
                1
            }
            0x5B => 1,
            0x5C => {
                self.r.e = self.r.h;
                1
            }
            0x5D => {
                self.r.e = self.r.l;
                1
            }
            0x5F => {
                self.r.e = self.r.a;
                1
            }
            0x60 => {
                self.r.h = self.r.b;
                1
            }
            0x61 => {
                self.r.h = self.r.c;
                1
            }
            0x62 => {
                self.r.h = self.r.d;
                1
            }
            0x63 => {
                self.r.h = self.r.e;
                1
            }
            0x64 => 1,
            0x65 => {
                self.r.h = self.r.l;
                1
            }
            0x67 => {
                self.r.h = self.r.a;
                1
            }
            0x68 => {
                self.r.l = self.r.b;
                1
            }
            0x69 => {
                self.r.l = self.r.c;
                1
            }
            0x6A => {
                self.r.l = self.r.d;
                1
            }
            0x6B => {
                self.r.l = self.r.e;
                1
            }
            0x6C => {
                self.r.l = self.r.h;
                1
            }
            0x6D => 1,
            0x6F => {
                self.r.l = self.r.a;
                1
            }
            0x76 => {
                self.halted = true;
                1
            }
            0x78 => {
                self.r.a = self.r.b;
                1
            }
            0x79 => {
                self.r.a = self.r.c;
                1
            }
            0x7A => {
                self.r.a = self.r.d;
                1
            }
            0x7B => {
                self.r.a = self.r.e;
                1
            }
            0x7C => {
                self.r.a = self.r.h;
                1
            }
            0x7D => {
                self.r.a = self.r.l;
                1
            }
            0x80 => {
                self.add(self.r.b);
                1
            }
            0x81 => {
                self.add(self.r.c);
                1
            }
            0x82 => {
                self.add(self.r.d);
                1
            }
            0x83 => {
                self.add(self.r.e);
                1
            }
            0x84 => {
                self.add(self.r.h);
                1
            }
            0x85 => {
                self.add(self.r.l);
                1
            }
            0x86 => {
                let val = self.m.read_byte(self.r.get_hl());
                self.add(val);
                1
            }
            0x87 => {
                self.add(self.r.a);
                1
            }
            0x88 => {
                self.adc(self.r.b);
                1
            }
            0x89 => {
                self.adc(self.r.c);
                1
            }
            0x8A => {
                self.adc(self.r.d);
                1
            }
            0x8B => {
                self.adc(self.r.e);
                1
            }
            0x8C => {
                self.adc(self.r.h);
                1
            }
            0x8D => {
                self.adc(self.r.l);
                1
            }
            0x8E => {
                let val = self.m.read_byte(self.r.get_hl());
                self.adc(val);
                1
            }
            0x8F => {
                self.adc(self.r.a);
                1
            }
            0x90 => {
                self.sub(self.r.b);
                1
            }
            0x91 => {
                self.sub(self.r.c);
                1
            }
            0x92 => {
                self.sub(self.r.d);
                1
            }
            0x93 => {
                self.sub(self.r.e);
                1
            }
            0x94 => {
                self.sub(self.r.h);
                1
            }
            0x95 => {
                self.sub(self.r.l);
                1
            }
            0x96 => {
                let val = self.m.read_byte(self.r.get_hl());
                self.sub(val);
                1
            }
            0x97 => {
                self.sub(self.r.a);
                1
            }
            0x98 => {
                self.sbc(self.r.b);
                1
            }
            0x99 => {
                self.sbc(self.r.c);
                1
            }
            0x9A => {
                self.sbc(self.r.d);
                1
            }
            0x9B => {
                self.sbc(self.r.e);
                1
            }
            0x9C => {
                self.sbc(self.r.h);
                1
            }
            0x9D => {
                self.sbc(self.r.l);
                1
            }
            0x9E => {
                let val = self.m.read_byte(self.r.get_hl());
                self.sbc(val);
                1
            }
            0x9F => {
                self.sbc(self.r.a);
                1
            }
            0xA0 => {
                self.and(self.r.b);
                1
            }
            0xA1 => {
                self.and(self.r.c);
                1
            }
            0xA2 => {
                self.and(self.r.d);
                1
            }
            0xA3 => {
                self.and(self.r.e);
                1
            }
            0xA4 => {
                self.and(self.r.h);
                1
            }
            0xA5 => {
                self.and(self.r.l);
                1
            }
            0xA6 => {
                let val = self.m.read_byte(self.r.get_hl());
                self.and(val);
                1
            }
            0xA7 => {
                self.and(self.r.a);
                1
            }
            0xB0 => {
                self.or(self.r.b);
                1
            }
            0xB1 => {
                self.or(self.r.c);
                1
            }
            0xB2 => {
                self.or(self.r.d);
                1
            }
            0xB3 => {
                self.or(self.r.e);
                1
            }
            0xB4 => {
                self.or(self.r.h);
                1
            }
            0xB5 => {
                self.or(self.r.l);
                1
            }
            0xB6 => {
                let val = self.m.read_byte(self.r.get_hl());
                self.or(val);
                1
            }
            0xB7 => {
                self.or(self.r.a);
                1
            }
            0xB8 => {
                self.cp(self.r.b);
                1
            }
            0xB9 => {
                self.cp(self.r.c);
                1
            }
            0xBA => {
                self.cp(self.r.d);
                1
            }
            0xBB => {
                self.cp(self.r.e);
                1
            }
            0xBC => {
                self.cp(self.r.h);
                1
            }
            0xBD => {
                self.cp(self.r.l);
                1
            }
            0xBE => {
                let val = self.m.read_byte(self.r.get_hl());
                self.cp(val);
                1
            }
            0xBF => {
                self.cp(self.r.a);
                1
            }
            notimpl => unimplemented!("Instruction {:2X} is not implemented", notimpl),
        }
    }
}
