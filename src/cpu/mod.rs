mod instructions;
mod opcodes;
mod registers;
use self::registers::CpuRegisters;
use crate::bus::Bus;

pub(crate) struct Cpu {
    registers: CpuRegisters,
    bus: *mut Bus,
}

impl Cpu {
    pub fn new(bus: *mut Bus) -> Self {
        Self {
            registers: CpuRegisters::new(),
            bus: bus,
        }
    }
}
