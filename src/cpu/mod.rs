mod instructions;
mod opcodes;
mod registers;
use self::opcodes::OpcodeRecord;
use self::registers::CpuRegisters;
use crate::bus::Bus;

pub(crate) struct Cpu {
    registers: CpuRegisters,
    cycle_counter: u16,
    bus: *mut Bus,
    total_cycles: u64,
    opcode_handler: Option<fn(&mut Cpu)>,
    opcode_record: &'static OpcodeRecord,
    halted: bool,
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
            opcode_handler: None,
            opcode_record: &opcodes::OPCODE_TABLE[0xEA],
            halted: false,
        }
    }

    fn load_reset_vector(&mut self) {
        let low = self.read_bus(Self::RESET_VECTOR);
        let high = self.read_bus(Self::RESET_VECTOR + 1);
        self.registers.program_counter = u16::from_le_bytes([low, high]);
        self.cycle_counter = 7;
    }

    fn load_irq_vector(&mut self) {
        let low = self.read_bus(Self::IRQ_VECTOR);
        let high = self.read_bus(Self::IRQ_VECTOR + 1);
        self.registers.program_counter = u16::from_le_bytes([low, high]);
    }

    fn load_nmi_vector(&mut self) {
        let low = self.read_bus(Self::NMI_VECTOR);
        let high = self.read_bus(Self::NMI_VECTOR + 1);
        self.registers.program_counter = u16::from_le_bytes([low, high]);
    }

    fn execute_nmi(&mut self) {
        self.read_bus(self.registers.program_counter); // Dummy read
        self.read_bus(self.registers.program_counter); // Dummy read
        self.push_word(self.registers.program_counter);
        self.push_byte(self.registers.status_for_stack_push(false));
        self.registers.set_interrupt_disable(true);
        self.load_nmi_vector();
        self.cycle_counter = 7;
    }

    fn execute_irq(&mut self) {
        self.read_bus(self.registers.program_counter); // Dummy read
        self.read_bus(self.registers.program_counter); // Dummy read
        self.push_word(self.registers.program_counter);
        self.push_byte(self.registers.status_for_stack_push(false));
        self.registers.set_interrupt_disable(true);
        self.load_irq_vector();
        self.cycle_counter = 7;
    }

    pub fn power_on(&mut self) {
        self.load_reset_vector();
    }

    pub fn reset(&mut self) {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(3);
        self.registers.set_interrupt_disable(true);
        self.halted = false;
        self.opcode_handler = None;
        self.load_reset_vector();
    }

    pub fn tick(&mut self) {
        self.total_cycles += 1;

        if self.halted {
            return;
        }

        // TODO: Add logic for handling IRQ delay for CLI and SEI

        // Approach is, exhaust cycles until the last, then execute
        // Mid instruction quirks are easier to deal with
        // This also allows halting for DMA transfer using the same counter
        if self.cycle_counter > 0 {
            // TODO: track interrupts for BRK etc.
            self.cycle_counter -= 1;
            if self.cycle_counter == 0 {
                // Even if the instruction incurs cycle penalties to then burn down, the instruction will still only execute once via Option
                if let Some(handler) = self.opcode_handler.take() {
                    handler(self); // Execute the intruction handler
                }
            }
            return;
        }

        if self.bus_mut().take_nmi_edge() {
            self.execute_nmi();
            return;
        }
        if self.bus_mut().irq_asserted() && !self.registers.interrupt_disable() {
            self.execute_irq();
            return;
        }

        let opcode = self.fetch_byte();
        self.opcode_record = &opcodes::OPCODE_TABLE[opcode as usize];
        // Burn one cycle for the fetch and decode
        self.cycle_counter = self.opcode_record.cycles - 1;
        // Store the handler for later when cycle counter is exhausted
        self.opcode_handler = Some(self.opcode_record.handler);
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

    #[inline]
    fn bus_mut(&mut self) -> &mut Bus {
        // Safety: This is the only pointer to bus for the CPU. The bus owns all other NES components
        unsafe { &mut *self.bus }
    }

    #[inline]
    fn read_bus(&mut self, address: u16) -> u8 {
        self.bus_mut().read(address)
    }

    #[inline]
    fn write_bus(&mut self, address: u16, value: u8) {
        // Single exception trap for OAMDMA write, never reaches the bus but keeps everything else simple
        if address == 0x4014 {
            self.perform_oamdma_write(value);
            return;
        }

        self.bus_mut().write(address, value);
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
