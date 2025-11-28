mod instructions;
mod opcodes;
mod registers;
use self::registers::CpuRegisters;
use crate::bus::Bus;

pub(crate) struct Cpu {
    registers: CpuRegisters,
    bus: *mut Bus,
    cycle_counter: u8,
    current_handler: Option<fn(&mut Cpu)>
}

impl Cpu {
    pub fn new(bus: *mut Bus) -> Self {
        Self {
            registers: CpuRegisters::new(),
            bus: bus,
            cycle_counter: 0,
            current_handler: None
        }
    }

    pub fn tick(&mut self) {
        // Approach is, exhaust cycles until the last, then execute. Mid instruction quirks are easier to deal with
        if self.cycle_counter > 0 {
            // todo: track interrupts for BRK etc.
            self.cycle_counter -= 1;
            if self.cycle_counter == 0 {
                if let Some(handler) = self.current_handler.take() {
                    handler(self); // Execute the intruction handler
                }
            }
            return; 
        }

        // todo: DMA handling later

        let opcode = self.fetch_byte();
        let opcode_record = &opcodes::OPCODE_TABLE[opcode as usize];
        // Burn one cycle for the fetch
        self.cycle_counter = opcode_record.cycles - 1;
        // Store the handler for later when cycle counter is exhausted
        self.current_handler = Some(opcode_record.handler);

    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.read_bus(self.registers.program_counter);
        self.registers.increment_pc();
        byte
    }

    fn read_bus(&self, address: u16) -> u8 {
        unsafe { (*self.bus).read(address) }
    }

    fn write_bus(&self, address: u16, value: u8) {
        unsafe { (*self.bus).write(address, value);}
    }
}
