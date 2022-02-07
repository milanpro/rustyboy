pub struct Registers {
  pub a: u8,
  // Flag register
  pub f: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
  pub pc: u16,
  pub sp: u16,
}

#[derive(Copy, Clone)]
pub enum Flag
{
    // Carry flag
    C = 0b00010000,
    // Half-carry flag
    H = 0b00100000,
    // Subtraction flag
    N = 0b01000000,
    // Zero flag
    Z = 0b10000000,
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      a: 0x00,
      f: 0x00,
      b: 0x00,
      c: 0x00,
      d: 0x00,
      e: 0x00,
      h: 0x00,
      l: 0x00,
      pc: 0x0000,
      sp: 0x0000,
    }
  }

  pub fn get_hl(&self) -> u16 {
    (self.h as u16) << 8 | self.l as u16
  }

  pub fn get_bc(&self) -> u16 {
    (self.b as u16) << 8 | self.c as u16
  }

  pub fn get_af(&self) -> u16 {
    (self.a as u16) << 8 | self.f as u16
  }

  pub fn get_de(&self) -> u16 {
    (self.d as u16) << 8 | self.e as u16
  }

  pub fn set_hl(&mut self, word: u16) {
    self.h = (word >> 8) as u8;
    self.l = word as u8;
  }

  pub fn set_bc(&mut self, word: u16) {
    self.b = (word >> 8) as u8;
    self.c = word as u8;
  }

  pub fn set_af(&mut self, word: u16) {
    self.a = (word >> 8) as u8;
    self.f = word as u8;
  }

  pub fn set_de(&mut self, word: u16) {
    self.d = (word >> 8) as u8;
    self.e = word as u8;
  }

  pub fn set_flag(&mut self, flag: Flag, value: bool) {
    let mask = flag as u8; match value {
      true => self.f |= mask,
      false => self.f &= !mask,
    }
    self.f &= 0xF0
  }

  pub fn get_flag(&self, flag: Flag) -> bool {
    let mask = flag as u8;
    self.f & mask != 0
  }

  fn set_flags(&mut self, flags: u8) {
    self.f = flags
  }

  fn reset_flag(&mut self) {
    self.f = 0;
  }

  pub fn rl_op(&mut self, byte: u8) -> u8 {
    let carry: u8 = if byte>>7 == 1 {Flag::C as u8} else {0};
    let value = byte<<1 | (self.get_flag(Flag::C) as u8);
    let zero: u8 = if value == 0 {Flag::Z as u8} else {0};
    self.set_flags(carry | zero);
    value
  }
}
