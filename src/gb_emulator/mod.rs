mod cpu;
mod registers;

pub fn startEmulation() {
  cpu::Z80CPU::new();
}