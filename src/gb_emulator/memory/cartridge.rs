#[derive(Debug)]
enum MBCType {
    MBC0,
    MBC1,
    MBC2,
    MBC3,
    MBC5,
}

pub struct Cartridge {
    cartridge_buffer: Vec<u8>,
    swap_rom_offset: usize,
    swap_ram: [u8; 0x1FFF],
    ram_active: bool,
    memory_bank_type: MBCType,
}

impl Cartridge {
    pub fn new(cartridge_buffer: Vec<u8>) -> Cartridge {
        let memory_bank_type = match cartridge_buffer[0x0147] {
            0x1 | 0x2 | 0x3 => MBCType::MBC1,
            0x5 | 0x6 => MBCType::MBC2,
            0xF | 0x10 | 0x11 | 0x12 | 0x13 => MBCType::MBC3,
            0x19 | 0x1A | 0x1C | 0x1D | 0x1B | 0x1E => MBCType::MBC5,
            _ => MBCType::MBC0,
        };

        Cartridge {
            cartridge_buffer: cartridge_buffer,
            swap_rom_offset: 0x4000,
            swap_ram: [0; 0x1FFF],
            ram_active: false,
            memory_bank_type: memory_bank_type,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ..= 0x3FFF => self.cartridge_buffer[addr as usize],
            0x4000 ..= 0x7FFF => self.cartridge_buffer[((addr - 0x4000) as usize) + self.swap_rom_offset],
            0xA000 ..= 0xBFFF => self.swap_ram[(addr - 0xA000) as usize],
            _ => panic!("access to cartridge in non mapped memory space: {:X}", addr),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match self.memory_bank_type {
            MBCType::MBC2 => match addr {
                0x0000 ..= 0x1FFF => self.ram_active = (addr & 0x100) == 0,
                0x2000 ..= 0x3FFF if (addr & 0x100) != 0 => self.select_rom_bank((val & 0xF) as u16),
                _ => panic!("write to cartridge at read only address: {:X}", addr),
            },
            _ => panic!("write to cartridge with unknown MBC: {:?}", self.memory_bank_type),
        }
    }

    fn select_rom_bank(&mut self, bank_id: u16) {
        self.swap_rom_offset = (bank_id as usize) * 0x4000;

        assert!(
            self.cartridge_buffer.len() >= self.swap_rom_offset + 0x4000,
            "Tried to swap in a bank not available on rom: {:X}", self.swap_rom_offset
        );
    }
}
