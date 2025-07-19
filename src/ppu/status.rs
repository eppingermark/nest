#[derive(Default, Clone)]
pub struct PpuFlags {
    pub v_blank: bool,
    pub sprite0hit: bool,
    pub sprite_overflow: bool,
}

impl PpuFlags {
    pub fn to_byte(&self) -> u8 {
        (if self.v_blank { 0x80 } else { 0 }) |
        (if self.sprite0hit { 0x40 } else { 0 }) |
        (if self.sprite_overflow { 0x20 } else { 0 })
    }

    pub fn from_byte(byte: u8) -> Self {
        Self {
            v_blank: byte & 0x80 != 0,
            sprite0hit: byte & 0x40 != 0,
            sprite_overflow: byte & 0x20 != 0,
        }
    }

    pub fn read_and_clear_vblank(&mut self) -> u8 {
        let byte = self.to_byte();
        self.v_blank = false;
        byte
    }
}
