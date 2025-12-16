use crate::cartridge::Cartridge;
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

    cartridge: Option<Cartridge>,
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
            cartridge: None,
        }
    }

    pub(crate) fn load_cartridge(&mut self, data: &[u8]) -> Result<(), &'static str> {
        if self.cartridge.is_none() {
            let cartridge = Cartridge::from_bytes(data)?;

            self.cartridge = Some(cartridge);

            Ok(())
        } else {
            Err("Cartridge already loaded")
        }
    }

    pub(crate) fn unload_cartridge(&mut self) {
        self.cartridge = None;
    }

    fn set_nmi(&mut self, level: bool) {
        if level && !self.nmi_line {
            self.nmi_edge_detected = true;
        }
        self.nmi_line = level;
    }

    pub(crate) fn take_nmi_edge(&mut self) -> bool {
        let edge = self.nmi_edge_detected;
        self.nmi_edge_detected = false;
        edge
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

    pub(crate) fn irq_asserted(&self) -> bool {
        self.irq_apu_frame || self.irq_apu_dmc || self.irq_mapper
    }

    pub(crate) fn ppu_stub_tick(&self) {
        // TODO: Remove when PPU implemented
    }

    pub(crate) fn cpu_read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram.read(address), // RAM
            0x2000..=0x3FFF => todo!(),                // PPU Registers
            0x4000..=0x4013 | 0x4015 => todo!(),       // APU
            0x4016 => todo!(),                         // Controller 1
            0x4017 => todo!(),                         // Controller 2
            0x4020..=0xFFFF => match self.cartridge.as_ref() {
                Some(cartridge) => cartridge.cpu_read(address),
                None => self.last_read,
            }, // Cartridge
            _ => self.last_read,                       // Open Bus
        }
    }

    pub(crate) fn cpu_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram.write(address, value), // RAM
            0x2000..=0x3FFF => todo!(),                        // PPU Registers
            0x4000..=0x4013 | 0x4015 | 0x4017 => todo!(),      // APU
            0x4014 => todo!(),                                 // OAM DMA
            0x4016 => todo!(),                                 // Controller Strobe
            0x4020..=0xFFFF => match self.cartridge.as_mut() {
                Some(cartridge) => cartridge.cpu_write(address, value),
                None => (),
            }, // Cartridge
            _ => (),                                           // Open Bus
        }
    }

    pub(crate) fn load_oam_data(&mut self, buffer: &[u8; 0x100]) {
        let oam_address = self.ppu.get_oam_address();
        self.ppu.oam.dma_write(oam_address, &buffer);
    }
}
