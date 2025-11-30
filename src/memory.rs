pub(crate) struct Ram {
    data: [u8; 0x0800],
}

impl Ram {
    pub(crate) fn new() -> Self {
        Self { data: [0; 0x0800] }
    }

    pub(crate) fn read(&self, address: u16) -> u8 {
        let index = (address & 0x07ff) as usize;
        self.data[index]
    }

    pub(crate) fn write(&mut self, address: u16, value: u8) {
        let index = (address & 0x07ff) as usize;
        self.data[index] = value;
    }
}

pub(crate) struct Vram {
    data: [u8; 0x0800],
}

impl Vram {
    // TODO: Implement interaction with mirroring
    pub(crate) fn new() -> Self {
        Self { data: [0; 0x0800] }
    }

    pub(crate) fn read(&self, address: u16) -> u8 {
        let index = (address & 0x07ff) as usize;
        self.data[index]
    }

    pub(crate) fn write(&mut self, address: u16, value: u8) {
        let index = (address & 0x07ff) as usize;
        self.data[index] = value;
    }
}

pub(crate) struct Palette {
    data: [u8; 0x20],
}

impl Palette {
    pub(crate) fn new() -> Self {
        Self { data: [0; 0x20] }
    }

    fn normalize(address: u16) -> usize {
        // Normalize ppu addresses to palette addresses
        let mut index = (address & 0x1F) as usize;
        // Sprite transparent entries ($3F10/$14/$18/$1C) mirror background entries ($3F00/$04/$08/$0C)
        if index >= 0x10 && index % 4 == 0 {
            index &= !0x10; // Clear bit 4 to map to background slot
        }
        index
    }

    pub(crate) fn read(&self, address: u16) -> u8 {
        self.data[Self::normalize(address)]
    }

    pub(crate) fn write(&mut self, address: u16, value: u8) {
        self.data[Self::normalize(address)] = value;
    }
}

pub(crate) struct Oam {
    data: [u8; 0x100],
}

impl Oam {
    pub(crate) fn new() -> Self {
        Self {
            data: [0xFF; 0x100],
        }
    }

    pub(crate) fn dma_write(&mut self, start_address: u8, source: &[u8; 0x100]) {
        let start = start_address as usize;
        let split = 256 - start;

        // Copy starting at start_address until split boundary
        self.data[start..].copy_from_slice(&source[..split]);
        // Wrap around to beginning and finish
        self.data[..start].copy_from_slice(&source[split..]);
    }
}
