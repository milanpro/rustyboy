mod opcodes;

use opcodes::Opcodes;

use super::memory::MemoryBus;
use super::registers::Flag;
use super::registers::Registers;

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

    pub fn run(&mut self) {
        loop {
            let ticks = self.cycle();
            dbg!(ticks);
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
                1
            }
            0x03 => {
                self.r.set_bc(self.r.get_bc().wrapping_add(1));
                1
            }
            0x04 => {
                self.r.b = self.inc(self.r.b);
                1
            }
            0x05 => {
                self.r.b = self.dec(self.r.b);
                1
            }
            0x06 => {
                self.r.b = self.fetch_byte();
                2
            }
            0x07 => {
                self.r.a = self.rlc(self.r.a);
                1
            }
            0x08 => {
                let addr = self.fetch_word();
                self.m.write_word(addr, self.r.sp);
                3
            }
            0x09 => {
                let result = self.add_16(self.r.get_hl(), self.r.get_bc());
                self.r.set_hl(result);
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
                self.r.c = self.inc(self.r.c);
                1
            }
            0x0D => {
                self.r.c = self.dec(self.r.c);
                1
            }
            0x0E => {
                self.r.c = self.fetch_byte();
                2
            }
            0x0F => {
                self.r.a = self.rrc(self.r.a);
                1
            }
            0x10 => 2, //STOP
            0x11 => {
                let v = self.fetch_word();
                self.r.set_de(v);
                3
            }
            0x12 => {
                self.m.write_byte(self.r.get_de(), self.r.a);
                1
            }
            0x13 => {
                self.r.set_de(self.r.get_de().wrapping_add(1));
                1
            }
            0x14 => {
                self.r.d = self.inc(self.r.d);
                1
            }
            0x15 => {
                self.r.d = self.dec(self.r.d);
                1
            }
            0x16 => {
                self.r.d = self.fetch_byte();
                2
            }
            0x17 => {
                self.r.a = self.rl(self.r.a);
                1
            }
            0x18 => {
                self.jr();
                2
            }
            0x19 => {
                let result = self.add_16(self.r.get_hl(), self.r.get_de());
                self.r.set_hl(result);
                1
            }
            0x1A => {
                self.r.a = self.m.read_byte(self.r.get_de());
                2
            }
            0x1B => {
                self.r.set_de(self.r.get_de().wrapping_rem(1));
                1
            }
            0x1C => {
                self.r.e = self.inc(self.r.e);
                1
            }
            0x1D => {
                self.r.e = self.dec(self.r.e);
                1
            }
            0x1E => {
                self.r.e = self.fetch_byte();
                2
            }
            0x1F => {
                self.r.a = self.rr(self.r.a);
                1
            }
            0x20 => {
                if !self.r.get_flag(Flag::Z) {
                    self.jr();
                    3
                } else {
                    self.r.pc +=1;
                    2
                }
            }
            0x21 => {
                let v = self.fetch_word();
                self.r.set_hl(v);
                3
            }
            0x22 => {
                self.m.write_byte(self.r.get_hl(), self.r.a); // TODO: correct?
                2
            }
            0x23 => {
                self.r.set_hl(self.r.get_hl().wrapping_add(1));
                1
            }
            0x24 => {
                self.r.h = self.inc(self.r.h);
                1
            }
            0x25 => {
                self.r.h = self.dec(self.r.h);
                1
            }
            0x26 => {
                self.r.h = self.fetch_byte();
                2
            }
            0x27 => {
                self.daa();
                1
            }
            0x28 => {
                if self.r.get_flag(Flag::Z) {
                    self.jr();
                    3
                } else {
                    self.r.pc +=1;
                    2
                }
            }
            0x29 => {
                let result = self.add_16(self.r.get_hl(), self.r.get_hl());
                self.r.set_hl(result);
                2
            }
            0x2A => {
                self.r.a = self.m.read_byte(self.r.get_hl()); // TODO: correct?
                2
            }
            0x2B => {
                self.r.set_hl(self.r.get_hl().wrapping_rem(1));
                1
            }
            0x2C => {
                self.r.l = self.inc(self.r.l);
                1
            }
            0x2D => {
                self.r.l = self.dec(self.r.l);
                1
            }
            0x2E => {
                self.r.l = self.fetch_byte();
                2
            }
            0x2F => {
                unimplemented!();
                1
            }
            0x30 => {
                if !self.r.get_flag(Flag::C) {
                    self.jr();
                    3
                } else {
                    self.r.pc +=1;
                    2
                }
            }
            0x31 => {
                self.r.sp = self.fetch_word();
                3
            }
            0x32 => {
                self.m.write_byte(self.r.get_hl(), self.r.a); // TODO: correct?
                2
            }
            0x33 => {
                self.r.sp = self.r.sp.wrapping_add(1);
                1
            }
            0x34 => {
                let addr = self.r.get_hl();
                let res = self.inc(self.m.read_byte(addr));
                self.m.write_byte(addr, res);
                1
            }
            0x35 => {
                let addr = self.r.get_hl();
                let res = self.dec(self.m.read_byte(addr));
                self.m.write_byte(addr, res);
                1
            }
            0x36 => {
                let v = self.fetch_byte();
                self.m.write_byte(self.r.get_hl(), v);
                3
            }
            0x37 => {
                self.r.set_flag(Flag::C, true);
                self.r.set_flag(Flag::H, false);
                self.r.set_flag(Flag::N, false);
                1
            }
            0x38 => {
                if self.r.get_flag(Flag::C) {
                    self.jr();
                    3
                } else {
                    self.r.pc +=1;
                    2
                }
            }
            0x39 => {
                unimplemented!()
            }
            0x3A => {
                self.r.a = self.m.read_byte(self.r.get_hl()); // TODO: correct?
                2
            }
            0x3B => {
                self.r.sp = self.r.sp.wrapping_rem(1);
                1
            }
            0x3C => {
                self.r.a = self.inc(self.r.a);
                1
            }
            0x3D => {
                self.r.a = self.dec(self.r.a);
                1
            }
            0x3E => {
                self.r.a = self.fetch_byte();
                2
            }
            0x3F => {
                unimplemented!();
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
            0x56 => {
                self.r.d = self.m.read_byte(self.r.get_hl());
                2
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
            0x5E => {
                self.r.e = self.m.read_byte(self.r.get_hl());
                2
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
            0x66 => {
                self.r.h = self.m.read_byte(self.r.get_hl());
                2
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
            0x6E => {
                self.r.l = self.m.read_byte(self.r.get_hl());
                2
            }
            0x6F => {
                self.r.l = self.r.a;
                1
            }
            0x70 => {
                self.m.write_byte(self.r.get_hl(), self.r.b);
                2
            }
            0x71 => {
                self.m.write_byte(self.r.get_hl(), self.r.c);
                2
            }
            0x72 => {
                self.m.write_byte(self.r.get_hl(), self.r.d);
                2
            }
            0x73 => {
                self.m.write_byte(self.r.get_hl(), self.r.e);
                2
            }
            0x74 => {
                self.m.write_byte(self.r.get_hl(), self.r.h);
                2
            }
            0x75 => {
                self.m.write_byte(self.r.get_hl(), self.r.l);
                2
            }
            0x76 => {
                self.halted = true;
                1
            }
            0x77 => {
                self.m.write_byte(self.r.get_hl(), self.r.a);
                2
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
            0x7E => {
                self.r.a = self.m.read_byte(self.r.get_hl());
                2
            }
            0x7F => 1,
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
            0xA8 => {
                self.xor(self.r.b);
                1
            }
            0xA9 => {
                self.xor(self.r.c);
                1
            }
            0xAA => {
                self.xor(self.r.d);
                1
            }
            0xAB => {
                self.xor(self.r.e);
                1
            }
            0xAC => {
                self.xor(self.r.h);
                1
            }
            0xAD => {
                self.xor(self.r.l);
                1
            }
            0xAE => {
                let val = self.m.read_byte(self.r.get_hl());
                self.xor(val);
                1
            }
            0xAF => {
                self.xor(self.r.a);
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
