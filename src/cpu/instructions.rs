use crate::{cpu::{Cpu, status_flags::CpuFlags}, js::consoleLog};

impl Cpu {
    pub fn adc(&mut self, addressing: (Vec<u8>, u8, String)) {
        let old_a = self.reg_a.clone();

        if self.flags.decimal && false { // disable the decimal flag... implemented before I realised that the NES doesn't support this...
            let mut low = (self.reg_a & 0x0F) + (addressing.1 & 0x0F) + (self.flags.carry as u8);
            let mut high = (self.reg_a >> 4) + (addressing.1 >> 4);

            if low > 9 {
                low = low.wrapping_add(6);
                high = high.wrapping_add(1);
            }

            if high > 9 {
                self.flags.carry = true;
                high = high.wrapping_add(6);
            } else {
                self.flags.carry = false;
            }

            self.reg_a = (high << 4) | (low & 0x0F);

            self.flags.overflow = ((!(old_a ^ addressing.1) & (old_a ^ self.reg_a)) & 0x80) != 0;
        } else {
            let res = self.reg_a as u16 + addressing.1 as u16 + self.flags.carry as u16;
            self.reg_a = res as u8;
            self.flags.carry = res > 0xFF;
            self.flags.overflow = ((!(old_a ^ addressing.1) & (old_a ^ self.reg_a)) & 0x80) != 0;
        }

        self.flags.zero = self.reg_a == 0;
        self.flags.negative = self.reg_a > 127;
        self.add_tracelog(&addressing.0, format!("ADC {}", addressing.2));
    }

    pub fn and(&mut self, addressing: (Vec<u8>, u8, String)) {
        self.reg_a &= addressing.1;
        self.flags.zero = self.reg_a == 0;
        self.flags.negative = self.reg_a > 127;
        self.add_tracelog(&addressing.0, format!("AND {}", addressing.2));
    }

    pub fn asl_accumulator(&mut self) {
        self.flags.carry = (self.reg_a & 0x80) != 0;
        self.reg_a <<= 1;
        self.cycle();
        self.flags.zero = self.reg_a == 0;
        self.flags.negative = self.reg_a > 127;
        self.add_tracelog(&[], format!("ASL"));
    }

    pub fn asl(&mut self, addressing: (Vec<u8>, u16, String)) {
        let target = self.read(addressing.1);
        self.flags.carry = (target & 0x80) != 0;
        self.cycle();
        self.write(addressing.1, target << 1);
        self.flags.zero = target == 0;
        self.flags.negative = target > 127;
        self.add_tracelog(&addressing.0, format!("ASL {}", addressing.2));
    }

    pub fn bit(&mut self, addressing: (Vec<u8>, u8, String)) {
        let op = addressing.1;
        self.flags.zero = self.reg_a & op == 0;
        self.flags.negative = (op & 0x80) != 0;
        self.flags.overflow = (op & 0x40) != 0;
        self.add_tracelog(&addressing.0, format!("BIT {}", addressing.2));
    }

    // this + set/clear flag used for BPL, BMI, BVC, BVS, BCC, BCS, BNE and BEQ
    pub fn branch_flag(&mut self, flag: bool, name: &str) {
        let byte = self.read_next();
        let target = self.counter.wrapping_add(byte as i8 as i16 as u16);

        if flag {
            let old = self.counter.clone();
            self.counter = target;
            self.cycle();

            if old & 0xFF00 != self.counter & 0xFF00 {
                self.cycle();
            }

            self.add_tracelog(&[byte], format!("{name} ${target:04X} -> ${target:04X}"));
        } else {
            self.add_tracelog(&[byte], format!("{name} ${target:04X} -> ${:04X}", self.counter.wrapping_add(1)));
        }
    }

    pub fn brk(&mut self) {
        let next = self.counter.wrapping_add(1);
        self.push_stack((next >> 8) as u8);
        self.push_stack((next & 0xFF) as u8);

        let mut status = self.flags.clone();
        status.brk = true;
        self.push_stack(status.to_byte());
        self.flags.interrupt_disable = true;

        let low = self.read(0xFFFE) as u16;
        let high = self.read(0xFFFF) as u16;
        self.counter = (high << 8) | low;
        self.cycle();
        self.add_tracelog(&[], String::from("BRK"));
    }

    pub fn cmp(&mut self, addressing: (Vec<u8>, u8, String)) {
        let res = self.reg_a.wrapping_sub(addressing.1);

        self.flags.carry = self.reg_a >= addressing.1;
        self.flags.zero = res == 0;
        self.flags.negative = res > 127;

        self.add_tracelog(&addressing.0, format!("CMP {}", addressing.2));
    }

    pub fn cpx(&mut self, addressing: (Vec<u8>, u8, String)) {
        let res = self.reg_x.wrapping_sub(addressing.1);

        self.flags.carry = self.reg_x >= addressing.1;
        self.flags.zero = res == 0;
        self.flags.negative = res > 127;

        self.add_tracelog(&addressing.0, format!("CPX {}", addressing.2));
    }

    pub fn cpy(&mut self, addressing: (Vec<u8>, u8, String)) {
        let res = self.reg_y.wrapping_sub(addressing.1);

        self.flags.carry = self.reg_y >= addressing.1;
        self.flags.zero = res == 0;
        self.flags.negative = res > 127;

        self.add_tracelog(&addressing.0, format!("CPY {}", addressing.2));
    }

    pub fn dec(&mut self, addressing: (Vec<u8>, u16, String)) {
        let prev = self.read(addressing.1);
        let res = prev.wrapping_sub(1);
        self.cycle();
        self.write(addressing.1, res);
        self.flags.zero = res == 0;
        self.flags.negative = res > 127;
        self.add_tracelog(&addressing.0, format!("DEC {}", addressing.2));
    }

    pub fn dex(&mut self) {
        let res = self.reg_x.wrapping_sub(1);
        self.reg_x = res;
        self.cycle();
        self.flags.zero = res == 0;
        self.flags.negative = res > 127;
        self.add_tracelog(&[], String::from("DEX"));
    }

    pub fn dey(&mut self) {
        let res = self.reg_y.wrapping_sub(1);
        self.reg_y = res;
        self.cycle();
        self.flags.zero = res == 0;
        self.flags.negative = res > 127;
        self.add_tracelog(&[], String::from("DEY"));
    }

    pub fn eor(&mut self, addressing: (Vec<u8>, u8, String)) {
        let res = self.reg_a ^ addressing.1;
        self.reg_a = res;
        self.flags.zero = res == 0;
        self.flags.negative = res > 127;
        self.add_tracelog(&addressing.0, format!("{}", addressing.2));
    }

    pub fn inc(&mut self, addressing: (Vec<u8>, u16, String)) {
        let prev = self.read(addressing.1);
        let res = prev.wrapping_add(1);
        self.cycle();
        self.write(addressing.1, res);
        self.flags.zero = res == 0;
        self.flags.negative = res > 127;
        self.add_tracelog(&addressing.0, format!("INC {}", addressing.2));
    }

    pub fn inx(&mut self) {
        let res = self.reg_x.wrapping_add(1);
        self.reg_x = res;
        self.cycle();
        self.flags.zero = res == 0;
        self.flags.negative = res > 127;
        self.add_tracelog(&[], String::from("INX"));
    }

    pub fn iny(&mut self) {
        let res = self.reg_y.wrapping_add(1);
        self.reg_y = res;
        self.cycle();
        self.flags.zero = res == 0;
        self.flags.negative = res > 127;
        self.add_tracelog(&[], String::from("INX"));
    }

    pub fn jmp_absolute(&mut self) {
        let low = self.read_next();
        let high = self.read_next();
        let addr = (high as u16) << 8 | low as u16;
        self.counter = addr;
        self.add_tracelog(&[low, high], format!("JMP ${addr:04X}"));
    }

    pub fn jmp_indirect(&mut self) {
        let low = self.read_next();
        let high = self.read_next();
        let ptr = (high as u16) << 8 | low as u16;

        let target_low = self.read(ptr);
        let target_high = if low == 0xFF {
            self.read(ptr & 0xFF00)
        } else {
            self.read(ptr.wrapping_add(1))
        };

        let addr = (target_high as u16) << 8 | target_low as u16;
        self.counter = addr;
        self.add_tracelog(&[low, high], format!("JMP (${ptr:04X})"));
    }

    pub fn jsr_absolute(&mut self) {
        let low = self.read_next();
        let high = self.read_next();
        let addr = (high as u16) << 8 | low as u16;

        let return_addr = self.counter.wrapping_sub(1);

        let (prev_low, prev_high) = (
            (return_addr & 0x00FF) as u8,
            (return_addr >> 8) as u8
        );

        self.push_stack(prev_high);
        self.push_stack(prev_low);
        self.counter = (high as u16) << 8 | low as u16;
        self.cycle();
        self.add_tracelog(&[low, high], format!("JSR ${addr:04X}"));
    }

    pub fn lda(&mut self, addressing: (Vec<u8>, u8, String)) {
        self.reg_a = addressing.1;
        self.flags.zero = self.reg_a == 0;
        self.flags.negative = self.reg_a > 127;
        self.add_tracelog(&addressing.0, format!("LDA {}", addressing.2));
    }

    pub fn ldx(&mut self, addressing: (Vec<u8>, u8, String)) {
        self.reg_x = addressing.1;
        self.flags.zero = self.reg_x == 0;
        self.flags.negative = self.reg_x > 127;
        self.add_tracelog(&addressing.0, format!("LDA {}", addressing.2));
    }

    pub fn ldy(&mut self, addressing: (Vec<u8>, u8, String)) {
        self.reg_y = addressing.1;
        self.flags.zero = self.reg_y == 0;
        self.flags.negative = self.reg_y > 127;
        self.add_tracelog(&addressing.0, format!("LDA {}", addressing.2));
    }

    pub fn lsr_accumulator(&mut self) {
        self.flags.carry = (self.reg_a & 0x01) != 0;
        self.reg_a >>= 1;
        self.cycle();
        self.flags.zero = self.reg_a == 0;
        self.flags.negative = false;
        self.add_tracelog(&[], format!("LSR"));
    }

    pub fn lsr(&mut self, addressing: (Vec<u8>, u16, String)) {
        let value = self.read(addressing.1);
        self.flags.carry = (value & 0x01) != 0;
        self.cycle();
        let result = value >> 1;
        self.flags.zero = result == 0;
        self.flags.negative = false;
        self.write(addressing.1, result);
        self.add_tracelog(&addressing.0, format!("LSR {}", addressing.2));
    }

    pub fn nop(&mut self) {
        self.cycle();
        self.cycle();
        self.add_tracelog(&[], String::from("NOP"));
    }

    pub fn ora(&mut self, addressing: (Vec<u8>, u8, String)) {
        let res = self.reg_a | addressing.1;
        self.reg_a = res;
        self.flags.zero = res == 0;
        self.flags.negative = false;
        self.add_tracelog(&addressing.0, format!("ORA {}", addressing.2));
    }

    pub fn pha(&mut self) {
        self.cycle();
        self.push_stack(self.reg_a);
        self.add_tracelog(&[], String::from("PHA"));
    }

    pub fn php(&mut self) {
        self.cycle();
        let mut status = self.flags.clone();
        status.brk = true;
        self.push_stack(status.to_byte());
        self.add_tracelog(&[], String::from("PHP"));
    }

    pub fn pla(&mut self) {
        self.cycle();
        self.reg_a = self.pull_stack();
        self.flags.zero = self.reg_a == 0;
        self.flags.negative = self.reg_a > 127;
        self.add_tracelog(&[], String::from("PLA"));
    }

    pub fn plp(&mut self) {
        self.cycle();
        let value = self.pull_stack();
        self.flags = CpuFlags::from_byte(value);
        self.cycle();
        self.add_tracelog(&[value], String::from("PLP"));
    }

    pub fn rol_accumulator(&mut self) {
        let old_carry = self.flags.carry;
        self.flags.carry = (self.reg_a & 0x80) != 0;
        self.cycle();
        self.reg_a = (self.reg_a << 1) | (old_carry as u8);
        self.flags.zero = self.reg_a == 0;
        self.flags.negative = (self.reg_a & 0x80) != 0;
        self.add_tracelog(&[], String::from("ROL"));
    }

    pub fn rol(&mut self, addressing: (Vec<u8>, u16, String)) {
        let mut value = self.read(addressing.1);
        let old_carry = self.flags.carry;
        self.flags.carry = (value & 0x80) != 0;
        value = (value << 1) | (old_carry as u8);
        self.write(addressing.1, value);
        self.flags.zero = value == 0;
        self.flags.negative = (value & 0x80) != 0;
        self.add_tracelog(&addressing.0, format!("ROL {}", addressing.2));
    }

    pub fn ror_accumulator(&mut self) {
        let old_carry = self.flags.carry;
        self.flags.carry = (self.reg_a & 0x01) != 0;
        self.cycle();
        self.reg_a = (self.reg_a >> 1) | ((old_carry as u8) << 7);
        self.flags.zero = self.reg_a == 0;
        self.flags.negative = (self.reg_a & 0x80) != 0;
        self.add_tracelog(&[], String::from("ROL"));
    }

    pub fn ror(&mut self, addressing: (Vec<u8>, u16, String)) {
        let mut value = self.read(addressing.1);
        let old_carry = self.flags.carry;
        self.flags.carry = (value & 0x01) != 0;
        value = (value >> 1) | ((old_carry as u8) << 7);
        self.write(addressing.1, value);
        self.flags.zero = value == 0;
        self.flags.negative = (value & 0x80) != 0;
        self.add_tracelog(&addressing.0, format!("ROL {}", addressing.2));
    }

    pub fn rti(&mut self) {
        let status = self.pull_stack();
        self.flags = CpuFlags::from_byte(status);
        let low = self.pull_stack();
        let high = self.pull_stack();
        let addr = (high as u16) << 8 | low as u16;
        self.counter = addr;
        self.add_tracelog(&[], format!("RTI ${status:02X} -> ${addr:04X}"));
    }

    pub fn rts(&mut self) {
        let low = self.pull_stack();
        let high = self.pull_stack();
        let addr = (high as u16) << 8 | low as u16;
        self.counter = addr.wrapping_add(1);
        self.cycle();
        self.add_tracelog(&[], format!("RTS -> ${addr:04X}"));
    }

    pub fn sbc(&mut self, addressing: (Vec<u8>, u8, String)) {
        let operand = addressing.1;
        let old_a = self.reg_a;
        let carry = if self.flags.carry { 0 } else { 1 };
        let res = self.reg_a as i16 - operand as i16 - carry as i16;
        self.reg_a = res as u8;
        self.flags.carry = res >= 0;
        self.flags.zero = self.reg_a == 0;
        self.flags.negative = (self.reg_a & 0x80) != 0;
        self.flags.overflow = ((old_a ^ operand) & 0x80 != 0) && ((old_a ^ self.reg_a) & 0x80 != 0);
        self.add_tracelog(&addressing.0, format!("SBC {}", addressing.2));
    }

    pub fn sta(&mut self, addressing: (Vec<u8>, u16, String)) {
        self.write(addressing.1, self.reg_a);
        self.add_tracelog(&addressing.0, format!("STA {}", addressing.2));
    }

    pub fn stx(&mut self, addressing: (Vec<u8>, u16, String)) {
        self.write(addressing.1, self.reg_x);
        self.add_tracelog(&addressing.0, format!("STX {}", addressing.2));
    }

    pub fn sty(&mut self, addressing: (Vec<u8>, u16, String)) {
        self.write(addressing.1, self.reg_y);
        self.add_tracelog(&addressing.0, format!("STY {}", addressing.2));
    }

    pub fn tax(&mut self) {
        self.reg_x = self.reg_a;
        self.cycle();
        self.add_tracelog(&[], String::from("TAX"));
    }

    pub fn tay(&mut self) {
        self.reg_y = self.reg_a;
        self.cycle();
        self.add_tracelog(&[], String::from("TAY"));
    }

    pub fn tsx(&mut self) {
        self.reg_x = self.stack;
        self.cycle();
        self.add_tracelog(&[], String::from("TSX"));
    }

    pub fn txa(&mut self) {
        self.reg_a = self.reg_x;
        self.cycle();
        self.add_tracelog(&[], String::from("TXA"));
    }

    pub fn txs(&mut self) {
        self.stack = self.reg_x;
        self.cycle();
        self.add_tracelog(&[], String::from("TXS"));
    }

    pub fn tya(&mut self) {
        self.reg_a = self.reg_y;
        self.cycle();
        self.add_tracelog(&[], String::from("TYA"));
    }

    pub fn hlt(&mut self) {
        self.bus.data = 0xFF;
        self.running = false;
        self.add_tracelog(&[], String::from("HLT"));
    }
}
