mod registers;

use self::registers::PpuRegisters;
use crate::memory::{Oam, Palette, Vram};

pub(crate) struct Ppu {
    registers: PpuRegisters,
    pub(super) oam: Oam,
    palette: Palette,
    vram: Vram,
}

impl Ppu {
    pub(crate) fn new() -> Self {
        Self {
            registers: PpuRegisters::new(),
            oam: Oam::new(),
            palette: Palette::new(),
            vram: Vram::new(),
        }
    }

    pub(super) fn get_oam_address(&self) -> u8 {
        self.registers.oam_address
    }
}
