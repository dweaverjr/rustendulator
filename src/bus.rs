use crate::memory::Ram;

pub(crate) struct Bus {
    ram: Ram,
    last_read: u8
}

impl Bus {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram.read(address), // RAM
            0x2000..=0x3FFF => unimplemented!(), // PPU Registers
            0x4000..=0x4013 | 0x4015 => unimplemented!(), // APU
            0x4016 => unimplemented!(), // Controller 1
            0x4017 => unimplemented!(), // Controller 2
            0x4020..=0xFFFF => unimplemented!(), // Cartridge
            _ => self.last_read, // Open Bus
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram.write(address, value), // RAM
            0x2000..=0x3FFF => unimplemented!(), // PPU Registers
            0x4000..=0x4013 | 0x4015 | 0x4017 => unimplemented!(), // APU
            0x4014 => unimplemented!(), // OAM DMA
            0x4016 => unimplemented!(), // Controller Strobe
            0x4020..=0xFFFF => unimplemented!(), // Cartridge
            _ => {} // Open Bus
         }
    }
}