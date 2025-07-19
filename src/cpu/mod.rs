use crate::js::consoleLog;
use status_flags::CpuFlags;

use crate::js::{addTracelog};

mod status_flags;
mod addressing;
mod instructions;
mod bus;
mod ram;

pub use bus::Bus;
pub use ram::Ram;

pub struct Cpu {
    pub cycles: usize,
    pub counter: u16,
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub stack: u8,
    pub flags: CpuFlags,
    pub bus: Bus,
    pub running: bool,
    last_read_instruction: u8,
    last_location: u16,
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        Self {
            cycles: 0,
            counter: 0,
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            stack: 0,
            bus,
            flags: CpuFlags::default(),
            running: false,
            last_read_instruction: 0,
            last_location: 0
        }
    }

    pub async fn reset(&mut self) {
        self.flags.interrupt_disable = true;
        self.running = true;

        let jmp_addr = {
            self.bus.address = 0xFFFC;
            self.bus.read();
            let second = self.bus.data;

            self.bus.address = 0xFFFD;
            self.bus.read();
            let first = self.bus.data;

            first as u16 * 0x100 + second as u16
        };

        self.counter = jmp_addr;
        self.stack = 0xFD;
    }

    pub fn clock(&mut self) -> usize {
        self.cycles = 0;
        self.last_location = self.counter;
        let byte = self.read_next();
        self.last_read_instruction = byte.clone();

        match byte {
            0x69 => self.run_op(Self::adc, Self::immediate),
            0x65 => self.run_op(Self::adc, Self::zeropage),
            0x75 => self.run_op(Self::adc, Self::zeropage_x),
            0x6D => self.run_op(Self::adc, Self::absolute),
            0x7D => self.run_op(Self::adc, Self::absolute_x),
            0x79 => self.run_op(Self::adc, Self::absolute_y),
            0x61 => self.run_op(Self::adc, Self::indirect_x),
            0x71 => self.run_op(Self::adc, Self::indirect_y),

            0x29 => self.run_op(Self::and, Self::immediate),
            0x25 => self.run_op(Self::and, Self::zeropage),
            0x35 => self.run_op(Self::and, Self::zeropage_x),
            0x2d => self.run_op(Self::and, Self::absolute),
            0x3d => self.run_op(Self::and, Self::absolute_x),
            0x39 => self.run_op(Self::and, Self::absolute_y),
            0x21 => self.run_op(Self::and, Self::indirect_x),
            0x31 => self.run_op(Self::and, Self::indirect_y),

            0x0A => self.asl_accumulator(),
            0x06 => self.run_op_addr(Self::asl, Self::zeropage_addr),
            0x16 => self.run_op_addr(Self::asl, Self::zeropage_x_addr),
            0x0E => self.run_op_addr(Self::asl, Self::absolute_addr),
            0x1E => self.run_op_addr(Self::asl, Self::absolute_x_addr),

            0x24 => self.run_op(Self::bit, Self::zeropage),
            0x2C => self.run_op(Self::bit, Self::absolute),

            0x10 => self.branch_flag(!self.flags.negative, "BPL"),
            0x30 => self.branch_flag(self.flags.negative, "BMI"),
            0x50 => self.branch_flag(!self.flags.overflow, "BVC"),
            0x70 => self.branch_flag(self.flags.overflow, "BVS"),
            0x90 => self.branch_flag(!self.flags.carry, "BCC"),
            0xB0 => self.branch_flag(self.flags.carry, "BCS"),
            0xD0 => self.branch_flag(!self.flags.zero, "BNE"),
            0xF0 => self.branch_flag(self.flags.zero, "BEQ"),

            0x18 => {
                self.flags.carry = false;
                self.add_tracelog(&[], String::from("CLC"));
            },
            0xD8 => {
                self.flags.decimal = false;
                self.add_tracelog(&[], String::from("CLD"));
            },
            0x58 => {
                self.flags.interrupt_disable = false;
                self.add_tracelog(&[], String::from("CLI"));
            },
            0xB8 => {
                self.flags.overflow = false;
                self.add_tracelog(&[], String::from("CLV"));
            },

            0x00 => self.brk(),

            0xC9 => self.run_op(Self::cmp, Self::immediate),
            0xC5 => self.run_op(Self::cmp, Self::zeropage),
            0xD5 => self.run_op(Self::cmp, Self::zeropage_x),
            0xCD => self.run_op(Self::cmp, Self::absolute),
            0xDD => self.run_op(Self::cmp, Self::absolute_x),
            0xD9 => self.run_op(Self::cmp, Self::absolute_y),
            0xC1 => self.run_op(Self::cmp, Self::indirect_x),
            0xD1 => self.run_op(Self::cmp, Self::indirect_y),

            0xE0 => self.run_op(Self::cpx, Self::immediate),
            0xE4 => self.run_op(Self::cpx, Self::zeropage),
            0xEC => self.run_op(Self::cpx, Self::absolute),

            0xC0 => self.run_op(Self::cpy, Self::immediate),
            0xC4 => self.run_op(Self::cpy, Self::zeropage),
            0xCC => self.run_op(Self::cpy, Self::absolute),

            0xC6 => self.run_op_addr(Self::dec, Self::zeropage_addr),
            0xD6 => self.run_op_addr(Self::dec, Self::zeropage_x_addr),
            0xCE => self.run_op_addr(Self::dec, Self::absolute_addr),
            0xDE => self.run_op_addr(Self::dec, Self::absolute_x_addr),

            0xCA => self.dex(),
            0x88 => self.dey(),

            0x49 => self.run_op(Self::eor, Self::immediate),
            0x45 => self.run_op(Self::eor, Self::zeropage),
            0x55 => self.run_op(Self::eor, Self::zeropage_x),
            0x4D => self.run_op(Self::eor, Self::absolute),
            0x5D => self.run_op(Self::eor, Self::absolute_x),
            0x59 => self.run_op(Self::eor, Self::absolute_y),
            0x41 => self.run_op(Self::eor, Self::indirect_x),
            0x51 => self.run_op(Self::eor, Self::indirect_y),

            0xE6 => self.run_op_addr(Self::inc, Self::zeropage_addr),
            0xF6 => self.run_op_addr(Self::inc, Self::zeropage_x_addr),
            0xEE => self.run_op_addr(Self::inc, Self::absolute_addr),
            0xFE => self.run_op_addr(Self::inc, Self::absolute_x_addr),

            0xE8 => self.inx(),
            0xC8 => self.iny(),

            0x4C => self.jmp_absolute(),
            0x6C => self.jmp_indirect(),

            0x20 => self.jsr_absolute(),

            0xA9 => self.run_op(Self::lda, Self::immediate),
            0xA5 => self.run_op(Self::lda, Self::zeropage),
            0xB5 => self.run_op(Self::lda, Self::zeropage_x),
            0xAD => self.run_op(Self::lda, Self::absolute),
            0xBD => self.run_op(Self::lda, Self::absolute_x),
            0xB9 => self.run_op(Self::lda, Self::absolute_y),
            0xA1 => self.run_op(Self::lda, Self::indirect_x),
            0xB1 => self.run_op(Self::lda, Self::indirect_y),

            0xA2 => self.run_op(Self::ldx, Self::immediate),
            0xA6 => self.run_op(Self::ldx, Self::zeropage),
            0xB6 => self.run_op(Self::ldx, Self::zeropage_y),
            0xAE => self.run_op(Self::ldx, Self::absolute),
            0xBE => self.run_op(Self::ldx, Self::absolute_y),

            0xA0 => self.run_op(Self::ldy, Self::immediate),
            0xA4 => self.run_op(Self::ldy, Self::zeropage),
            0xB4 => self.run_op(Self::ldy, Self::zeropage_x),
            0xAC => self.run_op(Self::ldy, Self::absolute),
            0xBC => self.run_op(Self::ldy, Self::absolute_x),

            0x4A => self.lsr_accumulator(),
            0x46 => self.run_op_addr(Self::lsr, Self::zeropage_addr),
            0x56 => self.run_op_addr(Self::lsr, Self::zeropage_x_addr),
            0x4E => self.run_op_addr(Self::lsr, Self::absolute_addr),
            0x5E => self.run_op_addr(Self::lsr, Self::absolute_x_addr),

            0xEA => self.nop(),

            0x09 => self.run_op(Self::ora, Self::immediate),
            0x05 => self.run_op(Self::ora, Self::zeropage),
            0x15 => self.run_op(Self::ora, Self::zeropage_x),
            0x0D => self.run_op(Self::ora, Self::absolute),
            0x1D => self.run_op(Self::ora, Self::absolute_x),
            0x19 => self.run_op(Self::ora, Self::absolute_y),
            0x01 => self.run_op(Self::ora, Self::indirect_x),
            0x11 => self.run_op(Self::ora, Self::indirect_y),

            0x48 => self.pha(),
            0x08 => self.php(),
            0x68 => self.pla(),
            0x28 => self.plp(),

            0x2A => self.rol_accumulator(),
            0x26 => self.run_op_addr(Self::rol, Self::zeropage_addr),
            0x36 => self.run_op_addr(Self::rol, Self::zeropage_x_addr),
            0x2E => self.run_op_addr(Self::rol, Self::absolute_addr),
            0x3E => self.run_op_addr(Self::rol, Self::absolute_x_addr),

            0x6A => self.ror_accumulator(),
            0x66 => self.run_op_addr(Self::ror, Self::zeropage_addr),
            0x76 => self.run_op_addr(Self::ror, Self::zeropage_x_addr),
            0x6E => self.run_op_addr(Self::ror, Self::absolute_addr),
            0x7E => self.run_op_addr(Self::ror, Self::absolute_x_addr),

            0x40 => self.rti(),
            0x60 => self.rts(),

            0xE9 => self.run_op(Self::sbc, Self::immediate),
            0xE5 => self.run_op(Self::sbc, Self::zeropage),
            0xF5 => self.run_op(Self::sbc, Self::zeropage_x),
            0xED => self.run_op(Self::sbc, Self::absolute),
            0xFD => self.run_op(Self::sbc, Self::absolute_x),
            0xF9 => self.run_op(Self::sbc, Self::absolute_y),
            0xE1 => self.run_op(Self::sbc, Self::indirect_x),
            0xF1 => self.run_op(Self::sbc, Self::indirect_y),

            0x38 => {
                self.flags.carry = true;
                self.add_tracelog(&[], String::from("SEC"));
            },
            0xF8 => {
                self.flags.decimal = true;
                self.add_tracelog(&[], String::from("SED"));
            },
            0x78 => {
                self.flags.interrupt_disable = true;
                self.add_tracelog(&[], String::from("SEI"));
            },

            0x85 => self.run_op_addr(Self::sta, Self::zeropage_addr),
            0x95 => self.run_op_addr(Self::sta, Self::zeropage_x_addr),
            0x8D => self.run_op_addr(Self::sta, Self::absolute_addr),
            0x9D => self.run_op_addr(Self::sta, Self::absolute_x_addr),
            0x99 => self.run_op_addr(Self::sta, Self::absolute_y_addr),
            0x81 => self.run_op_addr(Self::sta, Self::indirect_x_addr),
            0x91 => self.run_op_addr(Self::sta, Self::indirect_y_addr),

            0x86 => self.run_op_addr(Self::stx, Self::zeropage_addr),
            0x96 => self.run_op_addr(Self::stx, Self::zeropage_y_addr),
            0x8E => self.run_op_addr(Self::stx, Self::absolute_addr),

            0x84 => self.run_op_addr(Self::sty, Self::zeropage_addr),
            0x94 => self.run_op_addr(Self::sty, Self::zeropage_x_addr),
            0x8C => self.run_op_addr(Self::sty, Self::absolute_addr),

            0xAA => self.tax(),
            0xA8 => self.tay(),
            0xBA => self.tsx(),
            0x8A => self.txa(),
            0x9A => self.txs(),
            0x98 => self.tya(),

            0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2 => self.hlt(),

            x => {
                consoleLog(format!("Unimplemented instruction! {x:#x}").as_str());
                return self.cycles
            }
        };

        self.cycles
    }

    pub fn read_next(&mut self) -> u8 {
        self.bus.address = self.counter;
        self.bus.read();
        self.counter = self.counter.wrapping_add(1);
        self.cycle();
        self.bus.data
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        self.bus.address = addr;
        self.bus.read();
        self.cycle();
        self.bus.data
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.bus.address = addr;
        self.bus.data = val;
        self.bus.write();
        self.cycle();
    }

    pub fn cycle(&mut self) {
        self.cycles = self.cycles.wrapping_add(1);
    }

    pub fn push_stack(&mut self, val: u8) {
        self.write(0x100 + self.stack as u16, val);
        self.stack = self.stack.wrapping_sub(1);
    }

    pub fn pull_stack(&mut self) -> u8 {
        self.cycle();
        self.stack = self.stack.wrapping_add(1);
        self.read(0x100 + self.stack as u16)
    }

    pub fn add_tracelog(&self, by: &[u8], inst: String) {
        let f = format!(
            "{}{}--{}{}{}{}",
            if self.flags.negative { "N" } else { "n" },
            if self.flags.overflow { "V" } else { "v" },
            if self.flags.decimal { "D" } else { "d" },
            if self.flags.interrupt_disable { "I" } else { "i" },
            if self.flags.zero { "Z" } else { "z" },
            if self.flags.carry { "C" } else { "c" },
        );

        addTracelog(
            format!("{:04X}", self.last_location).as_str(),
            format!("{:02X} {}", self.last_read_instruction, by.into_iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ").as_str()).as_str(),
            inst.as_str(),
            format!("{:02X}", self.reg_a).as_str(),
            format!("{:02X}", self.reg_x).as_str(),
            format!("{:02X}", self.reg_y).as_str(),
            format!("{:02X}", self.stack).as_str(),
            format!("{:02X}", self.flags.to_byte()).as_str(),
            f.as_str(),
            self.cycles.to_string().as_str()
        );
    }

    fn run_op(
        &mut self,
        instr: fn(&mut Self, (Vec<u8>, u8, String)),
        addr_mode: fn(&mut Self) -> (Vec<u8>, u8, String)
    ) {
        let operand = addr_mode(self);
        instr(self, operand);
    }

    fn run_op_addr(
        &mut self,
        instr: fn(&mut Self, (Vec<u8>, u16, String)),
        addr_mode: fn(&mut Self) -> (Vec<u8>, u16, String)
    ) {
        let operand = addr_mode(self);
        instr(self, operand);
    }
}
