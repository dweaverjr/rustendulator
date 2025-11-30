mod instructions;
mod opcodes;
mod registers;
use self::registers::CpuRegisters;
use self::opcodes::AddressingMode;
use crate::bus::Bus;

pub(crate) struct Cpu {
    registers: CpuRegisters,
    bus: *mut Bus,
    cycle_counter: u8,
    current_handler: Option<fn(&mut Cpu)>,
    current_addressing_mode: AddressingMode,
    current_page_cross_penalty: bool,
}

impl Cpu {
    pub fn new(bus: *mut Bus) -> Self {
        Self {
            registers: CpuRegisters::new(),
            bus: bus,
            cycle_counter: 0,
            current_handler: None,
            current_addressing_mode: AddressingMode::Implicit,
            current_page_cross_penalty: false,
        }
    }

    pub fn tick(&mut self) {
        // Approach is, exhaust cycles until the last, then execute. Mid instruction quirks are easier to deal with
        if self.cycle_counter > 0 {
            // TODO: track interrupts for BRK etc.
            self.cycle_counter -= 1;
            if self.cycle_counter == 0 {
                // Even if the instruction incurs cycle penalties to then burn down, the instruction will still only execute once via Option
                if let Some(handler) = self.current_handler.take() {
                    handler(self); // Execute the intruction handler
                }
            }
            return; 
        }

        // TODO: DMA handling later

        let opcode = self.fetch_byte();
        let opcode_record = &opcodes::OPCODE_TABLE[opcode as usize];
        // Burn one cycle for the fetch and decode
        self.cycle_counter = opcode_record.cycles - 1;
        // Store the handler for later when cycle counter is exhausted
        self.current_handler = Some(opcode_record.handler);
        self.current_addressing_mode = opcode_record.addressing_mode;
        self.current_page_cross_penalty = opcode_record.page_cross_penalty;

    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.read_bus(self.registers.program_counter);
        self.registers.increment_pc();
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let low = self.fetch_byte();
        let high = self.fetch_byte();
        u16::from_le_bytes([low, high])
    }

    fn read_bus(&self, address: u16) -> u8 {
        unsafe { (*self.bus).read(address) }
    }

    fn write_bus(&self, address: u16, value: u8) {
        unsafe { (*self.bus).write(address, value);}
    }
}
