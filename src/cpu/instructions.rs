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
    fn get_operand_address(&mut self, mode: AddressingMode) -> u16 {
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

                // Dummy read if page crosses
                if (registers::PAGE_MASK & base) != (registers::PAGE_MASK & final_address) {
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

                // Dummy read if page crosses
                if (registers::PAGE_MASK & base) != (registers::PAGE_MASK & final_address) {
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

                // Dummy read on page cross
                if (registers::PAGE_MASK & base) != (registers::PAGE_MASK & final_address) {
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

    // Page cross helper
    fn check_page_cross(&mut self, base_address: u16, offset: u8) {
        if self.current_page_cross_penalty {
            let offset_address = base_address.wrapping_add(offset as u16);
            if (registers::PAGE_MASK & base_address) != (registers::PAGE_MASK & offset_address) {
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
        let address = self.get_operand_address(self.current_addressing_mode);
        let value = self.read_bus(address);

        self.registers.accumulator |= value;
        self.update_zero_and_negative(self.registers.accumulator);
    }

    pub(super) fn kil(&mut self) {
        self.halted = true;
    }

    pub(super) fn slo(&mut self) {
        todo!("SLO not implemented yet")
    }

    pub(super) fn nop(&mut self) {
        todo!("NOP not implemented yet")
    }

    pub(super) fn asl(&mut self) {
        todo!("ASL not implemented yet")
    }

    pub(super) fn php(&mut self) {
        todo!("PHP not implemented yet")
    }

    pub(super) fn anc(&mut self) {
        todo!("ANC not implemented yet")
    }

    pub(super) fn bpl(&mut self) {
        todo!("BPL not implemented yet")
    }

    pub(super) fn clc(&mut self) {
        todo!("CLC not implemented yet")
    }

    pub(super) fn jsr(&mut self) {
        todo!("JSR not implemented yet")
    }

    pub(super) fn and(&mut self) {
        todo!("AND not implemented yet")
    }

    pub(super) fn rla(&mut self) {
        todo!("RLA not implemented yet")
    }

    pub(super) fn bit(&mut self) {
        todo!("BIT not implemented yet")
    }

    pub(super) fn rol(&mut self) {
        todo!("ROL not implemented yet")
    }

    pub(super) fn plp(&mut self) {
        todo!("PLP not implemented yet")
    }

    pub(super) fn bmi(&mut self) {
        todo!("BMI not implemented yet")
    }

    pub(super) fn sec(&mut self) {
        todo!("SEC not implemented yet")
    }

    pub(super) fn rti(&mut self) {
        todo!("RTI not implemented yet")
    }

    pub(super) fn eor(&mut self) {
        todo!("EOR not implemented yet")
    }

    pub(super) fn sre(&mut self) {
        todo!("SRE not implemented yet")
    }

    pub(super) fn lsr(&mut self) {
        todo!("LSR not implemented yet")
    }

    pub(super) fn pha(&mut self) {
        todo!("PHA not implemented yet")
    }

    pub(super) fn alr(&mut self) {
        todo!("ALR not implemented yet")
    }

    pub(super) fn jmp(&mut self) {
        todo!("JMP not implemented yet")
    }

    pub(super) fn bvc(&mut self) {
        todo!("BVC not implemented yet")
    }

    pub(super) fn cli(&mut self) {
        todo!("CLI not implemented yet")
    }

    pub(super) fn rts(&mut self) {
        todo!("RTS not implemented yet")
    }

    pub(super) fn adc(&mut self) {
        todo!("ADC not implemented yet")
    }

    pub(super) fn rra(&mut self) {
        todo!("RRA not implemented yet")
    }

    pub(super) fn ror(&mut self) {
        todo!("ROR not implemented yet")
    }

    pub(super) fn pla(&mut self) {
        todo!("PLA not implemented yet")
    }

    pub(super) fn arr(&mut self) {
        todo!("ARR not implemented yet")
    }

    pub(super) fn bvs(&mut self) {
        todo!("BVS not implemented yet")
    }

    pub(super) fn sei(&mut self) {
        todo!("SEI not implemented yet")
    }

    pub(super) fn sta(&mut self) {
        todo!("STA not implemented yet")
    }

    pub(super) fn sax(&mut self) {
        todo!("SAX not implemented yet")
    }

    pub(super) fn sty(&mut self) {
        todo!("STY not implemented yet")
    }

    pub(super) fn stx(&mut self) {
        todo!("STX not implemented yet")
    }

    pub(super) fn dey(&mut self) {
        todo!("DEY not implemented yet")
    }

    pub(super) fn txa(&mut self) {
        todo!("TXA not implemented yet")
    }

    pub(super) fn xaa(&mut self) {
        todo!("XAA not implemented yet")
    }

    pub(super) fn bcc(&mut self) {
        todo!("BCC not implemented yet")
    }

    pub(super) fn ahx(&mut self) {
        todo!("AHX not implemented yet")
    }

    pub(super) fn tya(&mut self) {
        todo!("TYA not implemented yet")
    }

    pub(super) fn txs(&mut self) {
        todo!("TXS not implemented yet")
    }

    pub(super) fn tas(&mut self) {
        todo!("TAS not implemented yet")
    }

    pub(super) fn shy(&mut self) {
        todo!("SHY not implemented yet")
    }

    pub(super) fn shx(&mut self) {
        todo!("SHX not implemented yet")
    }

    pub(super) fn ldy(&mut self) {
        todo!("LDY not implemented yet")
    }

    pub(super) fn lda(&mut self) {
        todo!("LDA not implemented yet")
    }

    pub(super) fn ldx(&mut self) {
        todo!("LDX not implemented yet")
    }

    pub(super) fn lax(&mut self) {
        todo!("LAX not implemented yet")
    }

    pub(super) fn tay(&mut self) {
        todo!("TAY not implemented yet")
    }

    pub(super) fn tax(&mut self) {
        todo!("TAX not implemented yet")
    }

    pub(super) fn bcs(&mut self) {
        todo!("BCS not implemented yet")
    }

    pub(super) fn clv(&mut self) {
        todo!("CLV not implemented yet")
    }

    pub(super) fn tsx(&mut self) {
        todo!("TSX not implemented yet")
    }

    pub(super) fn las(&mut self) {
        todo!("LAS not implemented yet")
    }

    pub(super) fn cpy(&mut self) {
        todo!("CPY not implemented yet")
    }

    pub(super) fn cmp(&mut self) {
        todo!("CMP not implemented yet")
    }

    pub(super) fn dcp(&mut self) {
        todo!("DCP not implemented yet")
    }

    pub(super) fn dec(&mut self) {
        todo!("DEC not implemented yet")
    }

    pub(super) fn iny(&mut self) {
        todo!("INY not implemented yet")
    }

    pub(super) fn dex(&mut self) {
        todo!("DEX not implemented yet")
    }

    pub(super) fn axs(&mut self) {
        todo!("AXS not implemented yet")
    }

    pub(super) fn bne(&mut self) {
        todo!("BNE not implemented yet")
    }

    pub(super) fn cld(&mut self) {
        todo!("CLD not implemented yet")
    }

    pub(super) fn cpx(&mut self) {
        todo!("CPX not implemented yet")
    }

    pub(super) fn sbc(&mut self) {
        todo!("SBC not implemented yet")
    }

    pub(super) fn isc(&mut self) {
        todo!("ISC not implemented yet")
    }

    pub(super) fn inc(&mut self) {
        todo!("INC not implemented yet")
    }

    pub(super) fn inx(&mut self) {
        todo!("INX not implemented yet")
    }

    pub(super) fn beq(&mut self) {
        todo!("BEQ not implemented yet")
    }

    pub(super) fn sed(&mut self) {
        todo!("SED not implemented yet")
    }
}
