#[derive(Default, Clone)]
pub struct CpuFlags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal: bool,
    pub brk: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl CpuFlags {
    pub fn to_byte(&self) -> u8 {
        (if self.negative { 0x80 } else { 0 }) |
        (if self.overflow { 0x40 } else { 0 }) |
        0x20 |
        (if self.brk { 0x10 } else { 0 }) |
        (if self.decimal { 0x08 } else { 0 }) |
        (if self.interrupt_disable { 0x04 } else { 0 }) |
        (if self.zero { 0x02 } else { 0 }) |
        (if self.carry { 0x01 } else { 0 })
    }

    pub fn from_byte(byte: u8) -> Self {
        CpuFlags {
            negative:          byte & 0x80 != 0,
            overflow:          byte & 0x40 != 0,
            brk:               false,
            decimal:           byte & 0x08 != 0,
            interrupt_disable: byte & 0x04 != 0,
            zero:              byte & 0x02 != 0,
            carry:             byte & 0x01 != 0,
        }
    }
}
