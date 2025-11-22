pub struct Registers {
    accumulator: u8,
    index_x: u8,
    index_y: u8,
    program_counter: u16,
    stack_pointer: u8,
    status_flags: u8,
}

// Status flag masks

// Bit 0 - C
const CARRY_MASK: u8 = 0b0000_0001;

// Bit 1 - Z
const ZERO_MASK: u8 = 0b0000_0010;

// Bit 2 - I
const INTERRUPT_DISABLE_MASK: u8 = 0b0000_0100;

// Bit 3 - D
const DECIMAL_MASK: u8 = 0b0000_1000;

// Bit 4 - B
const BREAK_MASK: u8 = 0b0001_0000;

// Bit 5 - 1
const UNUSED_MASK: u8 = 0b0010_0000;

// Bit 6 - V
const OVERFLOW_MASK: u8 = 0b0100_0000;

// Bit 7 - N
const NEGATIVE_MASK: u8 = 0b1000_0000;

impl Registers {
    fn new() -> Registers {
        Registers {
            accumulator: 0,
            index_x: 0,
            index_y: 0,
            program_counter: 0xFFFC,
            stack_pointer: 0xFD,
            status_flags: 0b0010_0100,
        }
    }

    fn get_carry_flag(&self) -> bool {
        self.status_flags & CARRY_MASK != 0
    }

    fn set_carry_flag(&mut self, value: bool) {
        if value {
            self.status_flags |= CARRY_MASK
        } else {
            self.status_flags &= !CARRY_MASK
        };
    }

    fn get_zero_flag(&self) -> bool {
        self.status_flags & ZERO_MASK != 0
    }

    fn set_zero_flag(&mut self, value: bool) {
        if value {
            self.status_flags |= ZERO_MASK
        } else {
            self.status_flags &= !ZERO_MASK
        };
    }
}
