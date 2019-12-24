#[derive(Debug)]
enum MBCType {
    NO,
    MBC1,
    MBC2,
    MBC3,
    MBC5(u16),
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
            0x19 | 0x1A | 0x1C | 0x1D | 0x1B | 0x1E => MBCType::MBC5(0),
            _ => MBCType::NO,
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
            0x0000..=0x3FFF => self.cartridge_buffer[addr as usize],
            0x4000..=0x7FFF => {
                self.cartridge_buffer[((addr - 0x4000) as usize) + self.swap_rom_offset]
            }
            0xA000..=0xBFFF => self.swap_ram[(addr - 0xA000) as usize],
            _ => panic!("access to cartridge in non mapped memory space: {:X}", addr),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match self.memory_bank_type {
            MBCType::NO => panic!(
                "write attempt to cartridge without MBC: {:X} => {:X}",
                addr, val
            ),
            MBCType::MBC2 => match addr {
                0x0000..=0x1FFF => self.ram_active = (addr & 0x100) == 0,
                0x2000..=0x3FFF if (addr & 0x100) != 0 => self.select_rom_bank((val & 0xF) as u16),
                _ => panic!("write to cartridge at read only address: {:X}", addr),
            },
            MBCType::MBC3 => match addr {
                0x0000..=0x1FFF => self.ram_active = (val & 0xF) == 0xA,
                0x2000..=0x3FFF => {
                    if val == 0 {
                        self.select_rom_bank(1)
                    } else {
                        self.select_rom_bank((val & 0x7F) as u16)
                    }
                }
                0x4000..=0x7FFF => self.select_ram_bank(addr),
                _ => panic!("write to cartridge at read only address: {:X}", addr),
            },
            MBCType::MBC5(ref mut addr_cache) => match addr {
                0x0000..=0x1FFF => self.ram_active = (val & 0xF) == 0xA,
                0x2000..=0x2FFF => {
                    *addr_cache &= 0xFF00;
                    *addr_cache |= val as u16;
                    let rom_bank = *addr_cache;
                    self.select_rom_bank(rom_bank);
                }
                0x3000..=0x3FFF => {
                    *addr_cache &= 0xFF;
                    *addr_cache |= ((val & 1) as u16) << 8;
                    let rom_bank = *addr_cache;
                    self.select_rom_bank(rom_bank);
                }
                0x4000..=0x5FFF => self.select_ram_bank(val as u16),
                _ => panic!("write to cartridge at read only address: {:X}", addr),
            },
            _ => panic!(
                "write to cartridge with unknown MBC: {:?}",
                self.memory_bank_type
            ),
        }
    }

    fn select_rom_bank(&mut self, bank_id: u16) {
        self.swap_rom_offset = (bank_id as usize) * 0x4000;

        assert!(
            self.cartridge_buffer.len() >= self.swap_rom_offset + 0x4000,
            "tried to swap in a bank not available on rom: {:X}",
            self.swap_rom_offset
        );
    }

    fn select_ram_bank(&mut self, bank_id: u16) {
        panic!("rom bank selection not implemented");
    }
}
