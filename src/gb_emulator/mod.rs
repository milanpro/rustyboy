mod cpu;
mod memory;
mod registers;

pub fn start_emulation() {
  cpu::Z80CPU::new();
}
