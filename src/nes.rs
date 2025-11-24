use crate::bus::Bus;
use crate::cpu::Cpu;

pub struct Nes {
    bus: Bus,
    cpu: Cpu,
}

impl Nes {
    pub fn new() -> Self {
        let mut bus = Bus::new();
        let bus_pointer = &mut bus as *mut Bus;
        let cpu = Cpu::new(bus_pointer);

        Self { bus, cpu }
    }
}
