use super::Cpu;
use super::addressing::AddressingMode;

pub(self) struct OpcodeRecord {
    mnemonic: &'static str,
    instruction_call: fn(&mut Cpu),
    base_cycles: u8,
    addressing_mode: AddressingMode,
    page_cross_penaly: bool,
}
