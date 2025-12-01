mod instructions;
mod opcodes;
mod registers;
use self::opcodes::AddressingMode;
use self::registers::CpuRegisters;
use crate::bus::Bus;

pub(crate) struct Cpu {
    registers: CpuRegisters,
    cycle_counter: u16,
    bus: *mut Bus,
    total_cycles: u64,
    current_handler: Option<fn(&mut Cpu)>,
    current_addressing_mode: AddressingMode,
    current_page_cross_penalty: bool,
}

impl Cpu {
    // Hardcoded vectors
    const NMI_VECTOR: u16 = 0xFFFA;
    const RESET_VECTOR: u16 = 0xFFFC;
    const IRQ_VECTOR: u16 = 0xFFFE;

    pub fn new(bus: *mut Bus) -> Self {
        Self {
            registers: CpuRegisters::new(),
            cycle_counter: 0,
            bus,
            total_cycles: 0,
            current_handler: None,
            current_addressing_mode: AddressingMode::Implicit,
            current_page_cross_penalty: false,
        }
    }

    pub fn tick(&mut self) {
        self.total_cycles += 1;

        // Approach is, exhaust cycles until the last, then execute
        // Mid instruction quirks are easier to deal with
        // This also allows halting for DMA transfer using the same counter
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

    fn write_bus(&mut self, address: u16, value: u8) {
        // Single exception trap for OAMDMA write, never reaches the bus but keeps everything else simple
        if address == 0x4014 {
            self.perform_oamdma_write(value);
            return;
        }
        unsafe {
            (*self.bus).write(address, value);
        }
    }

    fn perform_oamdma_write(&mut self, page: u8) {
        // might make this a per cycle operation later
        let odd_cycle = (self.total_cycles & 1) as u16;

        // Read 256 bytes from CPU memory
        let page_start = (page as u16) << 8;
        let mut buffer = [0u8; 0x100];
        for i in 0..256 {
            buffer[i] = self.read_bus(page_start + i as u16);
        }

        // Write to OAM

        unsafe {
            (*self.bus).load_oam_data(&buffer);
        }

        // Suspend: 513 transfer cycles + 1 if odd cycle
        self.cycle_counter = 513 + odd_cycle;
    }
}
