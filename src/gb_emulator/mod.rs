mod cpu;
mod memory;
mod registers;

pub fn startEmulation() {
  cpu::Z80CPU::new();
}
