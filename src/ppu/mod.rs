mod registers;

use self::registers::PpuRegisters;
use crate::memory::{Oam, Palette, Vram};

pub(crate) struct Ppu {
    registers: PpuRegisters,
    oam: Oam,
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
}
