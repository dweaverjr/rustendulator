pub(super) struct CpuRegisters {
    pub(super) accumulator: u8,
    pub(super) index_x: u8,
    pub(super) index_y: u8,
    pub(super) program_counter: u16,
    pub(super) stack_pointer: u8,
    status_flags: u8,
}

// Status flag and stack push masks

// Bit 0 - C
const CARRY_MASK: u8 = 0b0000_0001;

// Bit 1 - Z
const ZERO_MASK: u8 = 0b0000_0010;

// Bit 2 - I
const INTERRUPT_DISABLE_MASK: u8 = 0b0000_0100;

// Bit 3 - D
const DECIMAL_MASK: u8 = 0b0000_1000;

// Bit 4 - B
const STACK_BREAK_MASK: u8 = 0b0001_0000;

// Bit 5 - 1
const UNUSED_MASK: u8 = 0b0010_0000;

// Bit 6 - V
const OVERFLOW_MASK: u8 = 0b0100_0000;

// Bit 7 - N
const NEGATIVE_MASK: u8 = 0b1000_0000;

impl CpuRegisters {
    // Initialized to post reset state
    pub(super) fn new() -> Self {
        Self {
            accumulator: 0,
            index_x: 0,
            index_y: 0,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_flags: 0b0010_0100,
        }
    }

    // Register getters for GUI

    fn accumulator(&self) -> u8 {
        self.accumulator
    }

    fn index_x(&self) -> u8 {
        self.index_x
    }

    fn index_y(&self) -> u8 {
        self.index_y
    }

    fn program_counter(&self) -> u16 {
        self.program_counter
    }

    fn stack_pointer(&self) -> u8 {
        self.stack_pointer
    }


    // Status flag getters and setters

    fn carry(&self) -> bool {
        self.status_flags & CARRY_MASK != 0
    }

    fn set_carry(&mut self, value: bool) {
        if value {
            self.status_flags |= CARRY_MASK
        } else {
            self.status_flags &= !CARRY_MASK
        }
    }

    fn zero(&self) -> bool {
        self.status_flags & ZERO_MASK != 0
    }

    fn set_zero(&mut self, value: bool) {
        if value {
            self.status_flags |= ZERO_MASK
        } else {
            self.status_flags &= !ZERO_MASK
        }
    }

    fn interrupt_disable(&self) -> bool {
        self.status_flags & INTERRUPT_DISABLE_MASK != 0
    }

    fn set_interrupt_disable(&mut self, value: bool) {
        if value {
            self.status_flags |= INTERRUPT_DISABLE_MASK
        } else {
            self.status_flags &= !INTERRUPT_DISABLE_MASK
        }
    }

    fn decimal(&self) -> bool {
        self.status_flags & DECIMAL_MASK != 0
    }

    fn set_decimal(&mut self, value: bool) {
        if value {
            self.status_flags |= DECIMAL_MASK
        } else {
            self.status_flags &= !DECIMAL_MASK
        }
    }

    fn overflow(&self) -> bool {
        self.status_flags & OVERFLOW_MASK != 0
    }

    fn set_overflow(&mut self, value: bool) {
        if value {
            self.status_flags |= OVERFLOW_MASK
        } else {
            self.status_flags &= !OVERFLOW_MASK
        }
    }

    fn negative(&self) -> bool {
        self.status_flags & NEGATIVE_MASK != 0
    }

    fn set_negative(&mut self, value: bool) {
        if value {
            self.status_flags |= NEGATIVE_MASK
        } else {
            self.status_flags &= !NEGATIVE_MASK
        }
    }

    // Stack operations
    fn status_for_stack_push(&self, is_break: bool) -> u8 {
        let mut result = self.status_flags | UNUSED_MASK;

        if is_break {
            result |= STACK_BREAK_MASK;
        } else {
            result &= !STACK_BREAK_MASK;
        }
        result
    }

    fn set_status_from_stack_pull(&mut self, value: u8) {
        self.status_flags = (value & 0b1100_1111) | UNUSED_MASK;
    }

    // Special flag clearing functions
    fn clear_carry(&mut self) {
        self.status_flags &= !CARRY_MASK;
    }

    fn clear_interrupt_disable(&mut self) {
        self.status_flags &= !INTERRUPT_DISABLE_MASK;
    }

    fn clear_decimal(&mut self) {
        self.status_flags &= !DECIMAL_MASK;
    }

    fn clear_overflow(&mut self) {
        self.status_flags &= !OVERFLOW_MASK;
    }

    pub(super) fn increment_pc(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1)
    }
}
