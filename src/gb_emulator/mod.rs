mod cpu;
mod registers;
mod memory;

pub fn startEmulation() {
  cpu::Z80CPU::new();
}