mod registers;
mod addressing;
mod opcodes;
use self::registers::CpuRegisters;
use crate::bus::Bus;


struct Cpu {
    registers: CpuRegisters,
    bus: *mut Bus,
}
