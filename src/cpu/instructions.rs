use super::{Cpu, opcodes::AddressingMode, registers};

impl Cpu {
    // Helpers for instruction handlers
    // Stack helpers
    fn push_byte(&mut self, value: u8) {
        let stack_address = registers::STACK_PAGE | self.registers.stack_pointer as u16;
        self.write_bus(stack_address, value);
        self.registers.decrement_sp();
    }

    fn push_word(&mut self, value: u16) {
        let [low, high] = value.to_le_bytes();
        self.push_byte(high);
        self.push_byte(low);
    }

    fn pop_byte(&mut self) -> u8 {
        self.registers.increment_sp();
        let stack_address = registers::STACK_PAGE | (self.registers.stack_pointer as u16);
        self.read_bus(stack_address)
    }

    fn pop_word(&mut self) -> u16 {
        let low = self.pop_byte();
        let high = self.pop_byte();
        u16::from_le_bytes([low, high])
    }

    // Address helper
    fn get_operand_address(&mut self, mode: AddressingMode, is_store: bool) -> u16 {
        match mode {
            AddressingMode::Immediate => {
                let address = self.registers.program_counter;
                self.registers.increment_pc();
                address
            }

            AddressingMode::ZeroPage => self.fetch_byte() as u16,

            AddressingMode::ZeroPageX => {
                let base = self.fetch_byte();
                base.wrapping_add(self.registers.index_x) as u16
            }

            AddressingMode::ZeroPageY => {
                let base = self.fetch_byte();
                base.wrapping_add(self.registers.index_y) as u16
            }

            AddressingMode::Absolute => self.fetch_word(),

            AddressingMode::AbsoluteX => {
                let base = self.fetch_word();
                let final_address = base.wrapping_add(self.registers.index_x as u16);

                // Reads add a cycle on page cross; stores don't (handled in opcode table)
                if !is_store
                    && (registers::PAGE_MASK & base) != (registers::PAGE_MASK & final_address)
                {
                    let wrong_address =
                        (registers::PAGE_MASK & base) | (registers::OFFSET_MASK & final_address);
                    self.read_bus(wrong_address);
                    self.cycle_counter += 1;
                }

                final_address
            }

            AddressingMode::AbsoluteY => {
                let base = self.fetch_word();
                let final_address = base.wrapping_add(self.registers.index_y as u16);

                // Reads add a cycle on page cross; stores don't (handled in opcode table)
                if !is_store
                    && (registers::PAGE_MASK & base) != (registers::PAGE_MASK & final_address)
                {
                    let wrong_address =
                        (registers::PAGE_MASK & base) | (registers::OFFSET_MASK & final_address);
                    self.read_bus(wrong_address);
                    self.cycle_counter += 1;
                }

                final_address
            }

            AddressingMode::Indirect => {
                let pointer_address = self.fetch_word();
                let low = self.read_bus(pointer_address);

                // 6502 bug, wraps within page boundary
                let high_address = (pointer_address & registers::PAGE_MASK)
                    | ((pointer_address.wrapping_add(1)) & registers::OFFSET_MASK);
                let high = self.read_bus(high_address);

                u16::from_le_bytes([low, high])
            }

            AddressingMode::IndirectX => {
                let base = self.fetch_byte();
                let pointer = base.wrapping_add(self.registers.index_x);
                // reads at zero page hence casts to u16
                let low = self.read_bus(pointer as u16);
                let high = self.read_bus(pointer.wrapping_add(1) as u16);
                u16::from_le_bytes([low, high])
            }

            AddressingMode::IndirectY => {
                let pointer = self.fetch_byte();
                let low = self.read_bus(pointer as u16);
                let high = self.read_bus(pointer.wrapping_add(1) as u16);
                let base = u16::from_le_bytes([low, high]);
                let final_address = base.wrapping_add(self.registers.index_y as u16);

                // Reads add a cycle on page cross; stores don't (handled in opcode table)
                if !is_store
                    && (registers::PAGE_MASK & base) != (registers::PAGE_MASK & final_address)
                {
                    let wrong_address =
                        (registers::PAGE_MASK & base) | (registers::OFFSET_MASK & final_address);
                    self.read_bus(wrong_address);
                    self.cycle_counter += 1;
                }

                final_address
            }

            AddressingMode::Relative => {
                // Branch helper handles offset
                self.registers.program_counter
            }

            AddressingMode::Accumulator => {
                // Operates on accumulator, nothing to use here
                0
            }

            AddressingMode::Implicit => {
                // No operand, nothing to use here
                0
            }
        }
    }

    // Status helpers
    fn update_zero_and_negative(&mut self, value: u8) {
        self.registers.set_zero(value == 0);
        self.registers
            .set_negative(value & registers::NEGATIVE_MASK != 0);
    }

    fn update_carry_zero_negative(&mut self, value: u8, carry: bool) {
        self.registers.set_carry(carry);
        self.update_zero_and_negative(value);
    }

    // Branch helper
    fn branch(&mut self, condition: bool) {
        let offset = self.fetch_byte() as i8;
        if condition {
            let previous_program_counter = self.registers.program_counter;
            self.registers.program_counter =
                // negative offsets wrap around to effectively subtract using two's complement
                previous_program_counter.wrapping_add(offset as i16 as u16);

            self.cycle_counter += 1;

            if (registers::PAGE_MASK & previous_program_counter)
                != (registers::PAGE_MASK & self.registers.program_counter)
            {
                self.cycle_counter += 1;
            }
        }
    }

    // Instruction handlers
    pub(super) fn brk(&mut self) {
        // To get PC + 2 including original BRK fetch
        self.registers.increment_pc();

        self.push_word(self.registers.program_counter);

        self.push_byte(self.registers.status_for_stack_push(true));

        self.registers.set_interrupt_disable(true);

        let vector_low = self.read_bus(Self::IRQ_VECTOR);
        let vector_high = self.read_bus(Self::IRQ_VECTOR + 1);
        self.registers.program_counter = u16::from_le_bytes([vector_low, vector_high]);
    }

    pub(super) fn ora(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);

        self.registers.accumulator |= value;
        self.update_zero_and_negative(self.registers.accumulator);
    }

    pub(super) fn kil(&mut self) {
        self.halted = true;
    }

    pub(super) fn slo(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        // Dummy write
        self.write_bus(address, value);

        let carry = value & 0x80 != 0;
        let shifted = value << 1;
        self.write_bus(address, shifted);

        self.registers.accumulator |= shifted;

        self.update_carry_zero_negative(self.registers.accumulator, carry);
    }

    pub(super) fn nop(&mut self) {
        let _ = self.get_operand_address(self.current_addressing_mode, false);
    }

    pub(super) fn asl(&mut self) {
        if self.current_addressing_mode == AddressingMode::Accumulator {
            let value = self.registers.accumulator;
            let carry = value & 0x80 != 0;
            self.registers.accumulator = value << 1;
            self.update_carry_zero_negative(self.registers.accumulator, carry);
        } else {
            let address = self.get_operand_address(self.current_addressing_mode, false);
            let value = self.read_bus(address);
            // Dummy write
            self.write_bus(address, value);
            let carry = value & 0x80 != 0;
            let shifted = value << 1;
            self.write_bus(address, shifted);
            self.update_carry_zero_negative(shifted, carry);
        }
    }

    pub(super) fn php(&mut self) {
        self.push_byte(self.registers.status_for_stack_push(true));
    }

    pub(super) fn anc(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);

        self.registers.accumulator &= value;

        let carry = self.registers.accumulator & 0x80 != 0;

        self.update_carry_zero_negative(self.registers.accumulator, carry);
    }

    pub(super) fn bpl(&mut self) {
        self.branch(!self.registers.negative());
    }

    pub(super) fn clc(&mut self) {
        self.registers.set_carry(false);
    }

    pub(super) fn jsr(&mut self) {
        let target_low = self.fetch_byte();

        // Get PC one before actual next instruction to which to return
        // RTS will pop and then increment the PC
        self.push_word(self.registers.program_counter);

        let target_high = self.fetch_byte();

        self.registers.program_counter = u16::from_le_bytes([target_low, target_high]);
    }

    pub(super) fn and(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);

        self.registers.accumulator &= value;
        self.update_zero_and_negative(self.registers.accumulator);
    }

    pub(super) fn rla(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        // Dummy write
        self.write_bus(address, value);

        let old_carry = self.registers.carry() as u8;
        let new_carry = value & 0x80 != 0;
        let rotated = (value << 1) | old_carry;
        self.write_bus(address, rotated);

        self.registers.accumulator &= rotated;

        self.update_carry_zero_negative(self.registers.accumulator, new_carry);
    }

    pub(super) fn bit(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);

        self.registers.set_negative(value & 0x80 != 0);
        self.registers.set_overflow(value & 0x40 != 0);
        self.registers
            .set_zero(self.registers.accumulator & value == 0);
    }

    pub(super) fn rol(&mut self) {
        if self.current_addressing_mode == AddressingMode::Accumulator {
            let value = self.registers.accumulator;
            let new_carry = value & 0x80 != 0;
            self.registers.accumulator = (value << 1) | (self.registers.carry() as u8);
            self.update_carry_zero_negative(self.registers.accumulator, new_carry);
        } else {
            let address = self.get_operand_address(self.current_addressing_mode, false);
            let value = self.read_bus(address);

            // Dummy write
            self.write_bus(address, value);

            let new_carry = value & 0x80 != 0;
            let rolled = (value << 1) | (self.registers.carry() as u8);
            self.write_bus(address, rolled);
            self.update_carry_zero_negative(rolled, new_carry);
        }
    }

    pub(super) fn plp(&mut self) {
        let status_value = self.pop_byte();
        self.registers.set_status_from_stack_pop(status_value);
    }

    pub(super) fn bmi(&mut self) {
        self.branch(self.registers.negative());
    }

    pub(super) fn sec(&mut self) {
        self.registers.set_carry(true);
    }

    pub(super) fn rti(&mut self) {
        let status_value = self.pop_byte();
        self.registers.set_status_from_stack_pop(status_value);

        let program_counter_value = self.pop_word();
        self.registers.program_counter = program_counter_value;
    }

    pub(super) fn eor(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);

        self.registers.accumulator ^= value;
        self.update_zero_and_negative(self.registers.accumulator);
    }

    pub(super) fn sre(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);

        // Dummy write
        self.write_bus(address, value);

        let carry = value & 0x01 != 0;
        let shifted = value >> 1;
        self.write_bus(address, shifted);

        self.registers.accumulator ^= shifted;
        self.update_carry_zero_negative(self.registers.accumulator, carry);
    }

    pub(super) fn lsr(&mut self) {
        if self.current_addressing_mode == AddressingMode::Accumulator {
            let value = self.registers.accumulator;
            let carry = value & 0x01 != 0;
            self.registers.accumulator = value >> 1;
            self.update_carry_zero_negative(self.registers.accumulator, carry);
        } else {
            let address = self.get_operand_address(self.current_addressing_mode, false);
            let value = self.read_bus(address);

            // Dummy write
            self.write_bus(address, value);

            let carry = value & 0x01 != 0;
            let shifted = value >> 1;
            self.write_bus(address, shifted);
            self.update_carry_zero_negative(shifted, carry);
        }
    }

    pub(super) fn pha(&mut self) {
        self.push_byte(self.registers.accumulator);
    }

    pub(super) fn alr(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);

        let anded = self.registers.accumulator & value;
        let carry = anded & 0x01 != 0;
        self.registers.accumulator = anded >> 1;

        self.update_carry_zero_negative(self.registers.accumulator, carry);
    }

    pub(super) fn jmp(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        self.registers.program_counter = address;
    }

    pub(super) fn bvc(&mut self) {
        self.branch(!self.registers.overflow());
    }

    pub(super) fn cli(&mut self) {
        self.registers.set_interrupt_disable(false);
    }

    pub(super) fn rts(&mut self) {
        let address = self.pop_word();
        self.registers.program_counter = address;
        self.registers.increment_pc();
    }

    pub(super) fn adc(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);

        let carry_in = self.registers.carry() as u16;
        let accumulator = self.registers.accumulator as u16;
        let sum = accumulator + value as u16 + carry_in;

        let result = sum as u8;

        let overflow = (self.registers.accumulator ^ result) & (value ^ result) & 0x80 != 0;
        let carry = sum > 0xFF;

        self.registers.accumulator = result;

        self.registers.set_overflow(overflow);
        self.registers.set_carry(carry);
        self.update_zero_and_negative(self.registers.accumulator);
    }

    pub(super) fn rra(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);

        // Dummy write
        self.write_bus(address, value);

        // ROR: old carry into bit 7, old bit 0 into carry
        let old_carry = self.registers.carry() as u8;
        let new_carry = value & 0x01 != 0;
        let rotated = (value >> 1) | (old_carry << 7);
        self.write_bus(address, rotated);

        // Set carry from ROR before ADC uses it
        self.registers.set_carry(new_carry);

        // ADC with rotated value
        let carry_in = self.registers.carry() as u16;
        let accumulator = self.registers.accumulator as u16;
        let sum = accumulator + rotated as u16 + carry_in;

        let result = sum as u8;

        let overflow = (self.registers.accumulator ^ result) & (rotated ^ result) & 0x80 != 0;
        let carry = sum > 0xFF;

        self.registers.accumulator = result;

        self.registers.set_overflow(overflow);
        self.registers.set_carry(carry);
        self.update_zero_and_negative(self.registers.accumulator);
    }

    pub(super) fn ror(&mut self) {
        if self.current_addressing_mode == AddressingMode::Accumulator {
            let value = self.registers.accumulator;
            let new_carry = value & 0x01 != 0;
            self.registers.accumulator = (value >> 1) | ((self.registers.carry() as u8) << 7);
            self.update_carry_zero_negative(self.registers.accumulator, new_carry);
        } else {
            let address = self.get_operand_address(self.current_addressing_mode, false);
            let value = self.read_bus(address);

            // Dummy write
            self.write_bus(address, value);

            let new_carry = value & 0x01 != 0;
            let rotated = (value >> 1) | ((self.registers.carry() as u8) << 7);
            self.write_bus(address, rotated);
            self.update_carry_zero_negative(rotated, new_carry);
        }
    }

    pub(super) fn pla(&mut self) {
        self.registers.accumulator = self.pop_byte();
        self.update_zero_and_negative(self.registers.accumulator);
    }

    pub(super) fn arr(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);

        // AND
        let anded = self.registers.accumulator & value;

        // ROR
        let result = (anded >> 1) | ((self.registers.carry() as u8) << 7);
        self.registers.accumulator = result;

        // Bizarre flag behavior
        let bit_6 = result & 0x40 != 0;
        let bit_5 = result & 0x20 != 0;

        self.registers.set_carry(bit_6);
        self.registers.set_overflow(bit_6 ^ bit_5);
        self.update_zero_and_negative(result);
    }

    pub(super) fn bvs(&mut self) {
        self.branch(self.registers.overflow());
    }

    pub(super) fn sei(&mut self) {
        self.registers.set_interrupt_disable(true);
    }

    pub(super) fn sta(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, true);
        self.write_bus(address, self.registers.accumulator);
    }

    pub(super) fn sax(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, true);
        self.write_bus(address, self.registers.accumulator & self.registers.index_x);
    }

    pub(super) fn sty(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, true);
        self.write_bus(address, self.registers.index_y);
    }

    pub(super) fn stx(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, true);
        self.write_bus(address, self.registers.index_x);
    }

    pub(super) fn dey(&mut self) {
        self.registers.index_y = self.registers.index_y.wrapping_sub(1);
        self.update_zero_and_negative(self.registers.index_y);
    }

    pub(super) fn txa(&mut self) {
        self.registers.accumulator = self.registers.index_x;
        self.update_zero_and_negative(self.registers.accumulator);
    }

    pub(super) fn xaa(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        self.registers.accumulator =
            (self.registers.accumulator | 0xEE) & self.registers.index_x & value;
        self.update_zero_and_negative(self.registers.accumulator);
    }

    pub(super) fn bcc(&mut self) {
        self.branch(!self.registers.carry());
    }

    pub(super) fn ahx(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, true);
        let high_byte = ((address >> 8) as u8).wrapping_add(1);
        let value = self.registers.accumulator & self.registers.index_x & high_byte;
        self.write_bus(address, value);
    }

    pub(super) fn tya(&mut self) {
        self.registers.accumulator = self.registers.index_y;
        self.update_zero_and_negative(self.registers.accumulator);
    }

    pub(super) fn txs(&mut self) {
        self.registers.stack_pointer = self.registers.index_x;
    }

    pub(super) fn tas(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, true);
        self.registers.stack_pointer = self.registers.accumulator & self.registers.index_x;
        let high_byte = ((address >> 8) as u8).wrapping_add(1);
        let value = self.registers.stack_pointer & high_byte;
        self.write_bus(address, value);
    }

    pub(super) fn shy(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, true);
        let high_byte = ((address >> 8) as u8).wrapping_add(1);
        let value = self.registers.index_y & high_byte;
        self.write_bus(address, value);
    }

    pub(super) fn shx(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, true);
        let high_byte = ((address >> 8) as u8).wrapping_add(1);
        let value = self.registers.index_x & high_byte;
        self.write_bus(address, value);
    }

    pub(super) fn ldy(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        self.registers.index_y = value;
        self.update_zero_and_negative(value);
    }

    pub(super) fn lda(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        self.registers.accumulator = value;
        self.update_zero_and_negative(value);
    }

    pub(super) fn ldx(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        self.registers.index_x = value;
        self.update_zero_and_negative(value);
    }

    pub(super) fn lax(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        self.registers.accumulator = value;
        self.registers.index_x = value;
        self.update_zero_and_negative(value);
    }

    pub(super) fn tay(&mut self) {
        self.registers.index_y = self.registers.accumulator;
        self.update_zero_and_negative(self.registers.index_y);
    }

    pub(super) fn tax(&mut self) {
        self.registers.index_x = self.registers.accumulator;
        self.update_zero_and_negative(self.registers.index_x);
    }

    pub(super) fn bcs(&mut self) {
        self.branch(self.registers.carry());
    }

    pub(super) fn clv(&mut self) {
        self.registers.set_overflow(false);
    }

    pub(super) fn tsx(&mut self) {
        self.registers.index_x = self.registers.stack_pointer;
        self.update_zero_and_negative(self.registers.index_x);
    }

    pub(super) fn las(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        let result = value & self.registers.stack_pointer;
        self.registers.accumulator = result;
        self.registers.index_x = result;
        self.registers.stack_pointer = result;
        self.update_zero_and_negative(result);
    }

    pub(super) fn cpy(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        let result = self.registers.index_y.wrapping_sub(value);
        self.registers.set_carry(self.registers.index_y >= value);
        self.update_zero_and_negative(result);
    }

    pub(super) fn cmp(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        let result = self.registers.accumulator.wrapping_sub(value);
        self.registers
            .set_carry(self.registers.accumulator >= value);
        self.update_zero_and_negative(result);
    }

    pub(super) fn dcp(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        // Dummy write
        self.write_bus(address, value);

        let decremented = value.wrapping_sub(1);
        self.write_bus(address, decremented);

        let result = self.registers.accumulator.wrapping_sub(decremented);
        self.registers
            .set_carry(self.registers.accumulator >= decremented);
        self.update_zero_and_negative(result);
    }

    pub(super) fn dec(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);

        // Dummy write
        self.write_bus(address, value);

        let decremented = value.wrapping_sub(1);
        self.write_bus(address, decremented);
        self.update_zero_and_negative(decremented);
    }

    pub(super) fn iny(&mut self) {
        self.registers.index_y = self.registers.index_y.wrapping_add(1);
        self.update_zero_and_negative(self.registers.index_y);
    }

    pub(super) fn dex(&mut self) {
        self.registers.index_x = self.registers.index_x.wrapping_sub(1);
        self.update_zero_and_negative(self.registers.index_x);
    }

    pub(super) fn axs(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        let anded = self.registers.accumulator & self.registers.index_x;
        let result = anded.wrapping_sub(value);
        self.registers.index_x = result;
        self.registers.set_carry(anded >= value);
        self.update_zero_and_negative(result);
    }

    pub(super) fn bne(&mut self) {
        self.branch(!self.registers.zero());
    }

    pub(super) fn cld(&mut self) {
        self.registers.set_decimal(false);
    }

    pub(super) fn cpx(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        let result = self.registers.index_x.wrapping_sub(value);
        self.registers.set_carry(self.registers.index_x >= value);
        self.update_zero_and_negative(result);
    }

    pub(super) fn sbc(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);

        let inverted = value ^ 0xFF;

        let carry_in = self.registers.carry() as u16;
        let accumulator = self.registers.accumulator as u16;
        let sum = accumulator + inverted as u16 + carry_in;

        let result = sum as u8;

        let overflow = (self.registers.accumulator ^ result) & (inverted ^ result) & 0x80 != 0;
        let carry = sum > 0xFF;

        self.registers.accumulator = result;

        self.registers.set_overflow(overflow);
        self.registers.set_carry(carry);
        self.update_zero_and_negative(self.registers.accumulator);
    }

    pub(super) fn isc(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);
        // Dummy write
        self.write_bus(address, value);
        let incremented = value.wrapping_add(1);
        self.write_bus(address, incremented);

        // SBC logic with incremented value
        let inverted = incremented ^ 0xFF;
        let carry_in = self.registers.carry() as u16;
        let accumulator = self.registers.accumulator as u16;
        let sum = accumulator + inverted as u16 + carry_in;
        let result = sum as u8;

        let overflow = (self.registers.accumulator ^ result) & (inverted ^ result) & 0x80 != 0;
        self.registers.accumulator = result;
        self.registers.set_overflow(overflow);
        self.registers.set_carry(sum > 0xFF);
        self.update_zero_and_negative(result);
    }

    pub(super) fn inc(&mut self) {
        let address = self.get_operand_address(self.current_addressing_mode, false);
        let value = self.read_bus(address);

        // Dummy write
        self.write_bus(address, value);

        let incremented = value.wrapping_add(1);
        self.write_bus(address, incremented);

        self.update_zero_and_negative(incremented);
    }

    pub(super) fn inx(&mut self) {
        self.registers.index_x = self.registers.index_x.wrapping_add(1);
        self.update_zero_and_negative(self.registers.index_x);
    }

    pub(super) fn beq(&mut self) {
        self.branch(self.registers.zero());
    }

    pub(super) fn sed(&mut self) {
        self.registers.set_decimal(true);
    }
}
