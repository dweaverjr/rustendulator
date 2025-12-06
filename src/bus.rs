use crate::memory::Ram;
use crate::ppu::Ppu;

pub(crate) struct Bus {
    ram: Ram,
    ppu: Ppu,

    // For open bus behavior (for now)
    last_read: u8,

    // NMI line
    nmi_line: bool,
    nmi_edge_detected: bool,

    // IRQ sources
    irq_apu_frame: bool,
    irq_apu_dmc: bool,
    irq_mapper: bool,
}

impl Bus {
    pub(crate) fn new() -> Self {
        Self {
            ram: Ram::new(),
            ppu: Ppu::new(),
            last_read: 0,
            nmi_line: false,
            nmi_edge_detected: false,
            irq_apu_frame: false,
            irq_apu_dmc: false,
            irq_mapper: false,
        }
    }

    fn set_irq_apu_frame(&mut self, asserted: bool) {
        self.irq_apu_frame = asserted;
    }

    fn set_irq_apu_dmc(&mut self, asserted: bool) {
        self.irq_apu_dmc = asserted;
    }

    fn set_irq_mapper(&mut self, asserted: bool) {
        self.irq_mapper = asserted;
    }

    fn irq_asserted(&self) -> bool {
        self.irq_apu_frame || self.irq_apu_dmc || self.irq_mapper
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

    pub(crate) fn load_oam_data(&mut self, buffer: &[u8; 0x100]) {
        let oam_address = self.ppu.get_oam_address();
        self.ppu.oam.dma_write(oam_address, &buffer);
    }
}
