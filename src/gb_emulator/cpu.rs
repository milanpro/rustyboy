use super::memory::MemoryBus;
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
            notimpl => unimplemented!("Instruction {:2X} is not implemented", notimpl),
        }
    }
}
