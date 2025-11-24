use super::Cpu;
pub(super) enum AddressingMode {
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Implicit,
    Accumulator,
    Immediate,
    Relative,
}

// Records for the opcode table
pub(super) struct OpcodeRecord {
    mnemonic: &'static str,
    function: fn(&mut Cpu),
    cycles: u8,
    addressing_mode: AddressingMode,
    page_cross_penalty: bool,
}

// Helper function to represent record more cleanly
const fn opcode(
    mnemonic: &'static str,
    function: fn(&mut Cpu),
    cycles: u8,
    addressing_mode: AddressingMode,
    page_cross_penalty: bool,
) -> OpcodeRecord {
    OpcodeRecord {
        mnemonic,
        function,
        cycles,
        addressing_mode,
        page_cross_penalty,
    }
}

pub(super) const OPCODE_TABLE: [OpcodeRecord; 14] =
    [
        /* 0x00 */ opcode("BRK", Cpu::brk, 7, AddressingMode::Implicit, false),
        /* 0x01 */ opcode("ORA", Cpu::ora, 6, AddressingMode::IndirectX, false),
        /* 0x02 */ opcode("KIL", Cpu::kil, 0, AddressingMode::Implicit, false),
        /* 0x03 */ opcode("SLO", Cpu::slo, 8, AddressingMode::IndirectX, false),
        /* 0x04 */ opcode("NOP", Cpu::nop, 3, AddressingMode::ZeroPage, false),
        /* 0x05 */ opcode("ORA", Cpu::ora, 3, AddressingMode::ZeroPage, false),
        /* 0x06 */ opcode("ASL", Cpu::asl, 5, AddressingMode::ZeroPage, false),
        /* 0x07 */ opcode("SLO", Cpu::slo, 5, AddressingMode::ZeroPage, false),
        /* 0x08 */ opcode("PHP", Cpu::php, 3, AddressingMode::Implicit, false),
        /* 0x09 */ opcode("ORA", Cpu::ora, 2, AddressingMode::Immediate, false),
        /* 0x0A */ opcode("ASL", Cpu::asl, 2, AddressingMode::Accumulator, false),
        /* 0x0B */ opcode("ANC", Cpu::anc, 2, AddressingMode::Immediate, false),
        /* 0x0C */ opcode("NOP", Cpu::nop, 4, AddressingMode::Absolute, false),
        /* 0x0D */ opcode("ORA", Cpu::ora, 4, AddressingMode::Absolute, false),
    ];
