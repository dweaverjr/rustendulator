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
struct OpcodeRecord {
    mnemonic: &'static str,
    instruction_call: fn(&mut Cpu),
    base_cycles: u8,
    addressing_mode: AddressingMode,
    page_cross_penalty: bool,
}

// Helper function to represent record more cleanly
const fn opcode(
    mnemonic: &'static str,
    instruction_call: fn(&mut Cpu),
    base_cycles: u8,
    addressing_mode: AddressingMode,
    page_cross_penalty: bool,
) -> OpcodeRecord {
    OpcodeRecord {
        mnemonic,
        instruction_call,
        base_cycles,
        addressing_mode,
        page_cross_penalty,
    }
}

pub(super) const OPCODE_TABLE: [OpcodeRecord; 1] =
    [
        /* 0x00 */ opcode("BRK", Cpu::brk, 7, AddressingMode::Implicit, false),
    ];
