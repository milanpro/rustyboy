mod cpu;
mod memory;
mod registers;
mod utils;

pub fn start_emulation() {
    cpu::Z80CPU::new();
}
