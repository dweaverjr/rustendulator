use super::{Cpu, opcodes::AddressingMode};

impl Cpu {
    // Helpers for instruction handlers
    // Stack helpers
    fn push_byte(&mut self, value: u8) {}

    fn push_word(&mut self, value: u16) {}

    fn pop_byte(&mut self) {}

    fn pop_word(&mut self) {}
    // Address helper
    fn get_operand_address(&mut self, mode: AddressingMode) {}
    // Status helpers
    fn update_zero_and_negative(&mut self, value: u8) {}

    fn update_carry_zero_negative(&mut self, value: u8) {}
    // Branch helper
    fn branch(&mut self, condition: bool) {}
    // Page cross helper
    fn check_page_cross(&mut self, base: u16, offset: u8) {}

    // Instruction handlers
    pub(super) fn brk(&mut self) {
        todo!("BRK not implemented yet")
    }

    pub(super) fn ora(&mut self) {
        todo!("ORA not implemented yet")
    }

    pub(super) fn kil(&mut self) {
        todo!("KIL not implemented yet")
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
