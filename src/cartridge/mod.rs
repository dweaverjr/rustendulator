mod mapper;
mod nrom;

use mapper::{Mapper, Mirroring};
use nrom::Nrom;

pub struct Cartridge {
    mapper: Box<dyn Mapper>,
    // Currently only handling iNES
    // Metadata from header
    prg_rom_size: usize,
    chr_rom_size: usize,
    mapper_id: u8,
    mirroring: Mirroring,
    has_battery: bool,
    has_trainer: bool,


}

impl Cartridge {
    pub fn from_bytes(rom: &[u8]) -> Result<Self, &'static str> {
        // Validate header
        if rom.len() < 16 {
            return Err("ROM too small for iNES header");
        }
        if &rom[0..4] != b"NES\x1A" {
            return Err("Invalid iNES magic number");
        }

        // Parse header
        let prg_banks = rom[4] as usize;
        let chr_banks = rom[5] as usize;
        let flags6 = rom[6];
        let flags7 = rom[7];

        let prg_rom_size = prg_banks * 16384;
        let chr_rom_size = chr_banks * 8192;
        let mapper_id = (flags6 >> 4) | (flags7 & 0xF0);

        let mirroring = if flags6 & 0x08 != 0 {
            Mirroring::FourScreen
        } else if flags6 & 0x01 != 0 {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        };

        let has_battery = flags6 & 0x02 != 0;
        let has_trainer = flags6 & 0x04 != 0;

        // Calculate ROM data offsets
        let trainer_size = if has_trainer { 512 } else { 0 };
        let prg_start = 16 + trainer_size;
        let prg_end = prg_start + prg_rom_size;
        let chr_start = prg_end;
        let chr_end = chr_start + chr_rom_size;

        // Validate ROM size
        if rom.len() < chr_end {
            return Err("ROM file truncated");
        }

        // Extract ROM data
        let prg_rom = rom[prg_start..prg_end].to_vec();
        let chr_rom = if chr_rom_size > 0 {
            rom[chr_start..chr_end].to_vec()
        } else {
            vec![0; 8192] // CHR-RAM
        };

        // Create mapper
        let mapper: Box<dyn Mapper> = match mapper_id {
            0 => Box::new(Nrom::new(prg_rom, chr_rom, mirroring)),
            _ => return Err("Unsupported mapper"),
        };

        Ok(Cartridge {
            mapper,
            prg_rom_size,
            chr_rom_size,
            mapper_id,
            mirroring,
            has_battery,
            has_trainer,
        })
    }

    pub fn cpu_read(&self, address: u16) -> u8 {
        self.mapper.cpu_read(address)
    }

    pub fn cpu_write(&mut self, address: u16, value: u8) {
        self.mapper.cpu_write(address, value);
    }

    pub fn ppu_read(&self, address: u16) -> u8 {
        self.mapper.ppu_read(address)
    }

    pub fn ppu_write(&mut self, address: u16, value: u8) {
        self.mapper.ppu_write(address, value);
    }
}