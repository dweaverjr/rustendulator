use crate::memory::Ram;

pub(crate) struct Bus {
    ram: Ram,
    last_read: u8,
}

impl Bus {
    pub(crate) fn new() -> Self {
        Self {
            ram: Ram::new(),
            last_read: 0,
        }
    }

    pub(crate) fn ppu_stub_tick(&self) {
        // TODO: Remove when PPU implemented
    }

    pub(crate) fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram.read(address), // RAM
            0x2000..=0x3FFF => todo!(),                // PPU Registers
            0x4000..=0x4013 | 0x4015 => todo!(),       // APU
            0x4016 => todo!(),                         // Controller 1
            0x4017 => todo!(),                         // Controller 2
            0x4020..=0xFFFF => todo!(),                // Cartridge
            _ => self.last_read,                       // Open Bus
        }
    }

    pub(crate) fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram.write(address, value), // RAM
            0x2000..=0x3FFF => todo!(),                        // PPU Registers
            0x4000..=0x4013 | 0x4015 | 0x4017 => todo!(),      // APU
            0x4014 => todo!(),                                 // OAM DMA
            0x4016 => todo!(),                                 // Controller Strobe
            0x4020..=0xFFFF => todo!(),                        // Cartridge
            _ => {}                                            // Open Bus
        }
    }
}
