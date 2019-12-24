mod cartridge;
use cartridge::Cartridge;

pub struct MemoryBus {
    cartridge: Cartridge,
    tile_ram: [u8; 0x17FF],
    working_ram: [u8; 0x1FFF],
    object_attribute_memory: [u8; 0x9F],
    io_registers: [u8; 0x7F],
    high_ram: [u8; 0x7E],
    interrupt_enable_register: u8
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        //ROM dummy of bigges possible size (8Mb)
        let dummy_rom = vec![0; 0x7a1200];

        MemoryBus {
            cartridge: Cartridge::new(dummy_rom),
            tile_ram: [0; 0x17FF],
            working_ram: [0; 0x1FFF],
            object_attribute_memory: [0; 0x9F],
            io_registers: [0; 0x7F],
            high_ram: [0; 0x7E],
            interrupt_enable_register: 0
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ..= 0x7FFF => self.cartridge.read_byte(addr),
            0x8000 ..= 0x9FFF => self.tile_ram[(addr - 0x8000) as usize],
            0xA000 ..= 0xBFFF => self.cartridge.read_byte(addr),
            0xC000 ..= 0xDFFF => self.working_ram[(addr - 0xC000) as usize],
            0xE000 ..= 0xFDFF => self.working_ram[(addr - 0xE000) as usize],
            0xFE00 ..= 0xFE9F => self.object_attribute_memory[(addr - 0xFE00) as usize],
            0xFEA0 ..= 0xFEFF => 0,
            0xFF00 ..= 0xFF7F => self.io_registers[(addr - 0xFF00) as usize],
            0xFF80 ..= 0xFFFE => self.high_ram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable_register
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000 ..= 0x7FFF => self.cartridge.write_byte(addr, val),
            0x8000 ..= 0x9FFF => self.tile_ram[(addr - 0x8000) as usize] = val,
            0xA000 ..= 0xBFFF => self.cartridge.write_byte(addr, val),
            0xC000 ..= 0xDFFF => self.working_ram[(addr - 0xC000) as usize] = val,
            0xE000 ..= 0xFDFF => self.working_ram[(addr - 0xE000) as usize] = val,
            0xFE00 ..= 0xFE9F => self.object_attribute_memory[(addr - 0xFE00) as usize] = val,
            0xFEA0 ..= 0xFEFF => {},
            0xFF00 ..= 0xFF7F => self.io_registers[(addr - 0xFF00) as usize] = val,
            0xFF80 ..= 0xFFFE => self.high_ram[(addr - 0xFF80) as usize] = val,
            0xFFFF => self.interrupt_enable_register = val
        }
    }
}