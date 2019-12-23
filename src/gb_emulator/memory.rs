pub struct MemoryBus {
    initial_rom: [u8; 0x3FFF],
    swap_rom: [u8; 0x3FFF],
    tile_ram: [u8; 0x17FF],
    swap_ram: [u8; 0x1FFF],
    working_ram: [u8; 0x1FFF],
    object_attribute_memory: [u8; 0x9F],
    io_registers: [u8; 0x7F],
    high_ram: [u8; 0x7E],
    interrupt_enable_register: u8
}

impl MemoryBus {
    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ..= 0x3FFF => self.initial_rom[addr as usize],
            0x4000 ..= 0x7FFF => self.swap_rom[(addr - 0x4000) as usize],
            0x8000 ..= 0x9FFF => self.tile_ram[(addr - 0x8000) as usize],
            0xA000 ..= 0xBFFF => self.swap_ram[(addr - 0xA000) as usize],
            0xC000 ..= 0xDFFF => self.working_ram[(addr - 0xC000) as usize],
            0xE000 ..= 0xFDFF => self.working_ram[(addr - 0xE000) as usize],
            0xFE00 ..= 0xFE9F => self.object_attribute_memory[(addr - 0xFE00) as usize],
            0xFEA0 ..= 0xFEFF => 0,
            0xFF00 ..= 0xFF7F => self.io_registers[(addr - 0xFF00) as usize],
            0xFF80 ..= 0xFFFE => self.high_ram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable_register
        }
    }
}