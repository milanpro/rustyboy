struct Registers {
  // 8-bit-registers
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  h: u8,
  l: u8,
  f: u8,

  // 16-bit-registers
  pc: u16,
  sp: u16,

  // Clock for last instruction
  m: u16,
  t: u16
} 

struct TimeClock {
  m: u16,
  t: u16
}

pub struct Z80CPU {
  registers: Registers,
  time_clock: TimeClock,
  halt: u8,
  stop: u8  
}

impl Z80CPU {

  pub fn new() -> Z80CPU {
    let registers = Registers {a:0, b:0, c:0, d:0, e:0, h:0, l: 0, f:0, m:0, pc:0, sp:0, t:0 };
    let time_clock = TimeClock {m:0, t:0};
    return Z80CPU {registers, time_clock, halt: 0, stop: 0};
  }

  fn reset(mut self) {
    self.registers = Registers {a:0, b:0, c:0, d:0, e:0, h:0, l: 0, f:0, m:0, pc:0, sp:0, t:0 };
    self.time_clock = TimeClock {m:0, t:0};
    self.halt = 0;
    self.stop = 0;
  }
}
