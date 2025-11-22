pub struct Ram {
    pub address: [u8; 0x0800],
}

impl Ram {
    pub fn read(&self, address: u16) -> u8 {
        let index: usize = (address & 0x07ff) as usize;
        self.address[index]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let index: usize = (address & 0x07ff) as usize;
        self.address[index] = value;
    }
}
