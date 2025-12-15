#[derive(Clone, Copy)]
pub(super) enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

pub(super) trait Mapper {
    fn cpu_read(&self, address: u16) -> u8;
    fn cpu_write(&mut self, address: u16, value: u8);
    fn ppu_read(&self, address: u16) -> u8;
    fn ppu_write(&mut self, address: u16, value: u8);
    fn mirroring(&self) -> Mirroring;
}