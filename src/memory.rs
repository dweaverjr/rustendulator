pub(crate) struct Ram {
    address: [u8; 0x0800],
}

impl Ram {
    pub(crate) fn new() -> Self {
        Self {
            address: [0; 0x0800],
        }
    }

    pub(crate) fn read(&self, address: u16) -> u8 {
        let index: usize = (address & 0x07ff) as usize;
        self.address[index]
    }

    pub(crate) fn write(&mut self, address: u16, value: u8) {
        let index = (address & 0x07ff) as usize;
        self.address[index] = value;
    }
}
