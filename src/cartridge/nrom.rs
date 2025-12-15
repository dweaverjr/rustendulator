use super::mapper::{Mapper, Mirroring};

pub(super) struct Nrom {
    prg_rom: Vec<u8>,
    chr: Vec<u8>,
    prg_ram: [u8; 8192],
    mirroring: Mirroring,
    prg_mask: usize,
}

impl Nrom {
    pub fn new(prg_rom: Vec<u8>, chr: Vec<u8>, mirroring: Mirroring) -> Self {
        let prg_mask = if prg_rom.len() <= 0x4000 {
            0x3FFF // Mirror
        } else {
            0x7FFF // Full
        };

        Self {
            prg_rom,
            chr,
            prg_ram: [0; 8192],
            mirroring,
            prg_mask,
        }
    }
}

impl Mapper for Nrom {
    fn cpu_read(&self, address: u16) -> u8 {
        match address {
            0x6000..=0x7FFF => self.prg_ram[(address - 0x6000) as usize],
            0x8000..=0xFFFF => {
                let index = (address - 0x8000) as usize & self.prg_mask;
                self.prg_rom[index]
            }
            _ => 0, // Open bus for unmapped regions
        }
    }

    fn cpu_write(&mut self, address: u16, value: u8) {
        match address {
            0x6000..=0x7FFF => self.prg_ram[(address - 0x6000) as usize] = value,
            _ => {} // PRG ROM is read-only, writes ignored
        }
    }

    fn ppu_read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.chr[address as usize],
            _ => 0,
        }
    }

    fn ppu_write(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            self.chr[address as usize] = value;
        }
    }

    fn mirroring(&self) -> Mirroring {
        self.mirroring
    }
}