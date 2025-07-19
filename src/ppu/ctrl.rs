#[derive(Default, Clone)]
pub struct PpuCtrl{
    pub base_nametable: u8,
    pub vram_increment: bool,
    pub sprite_table: bool,
    pub bg_table: bool,
    pub sprite_size: bool,
    pub nmi: bool,
}

impl PpuCtrl{
    pub fn to_byte(&self)->u8{
        self.base_nametable & 3
        | (self.vram_increment as u8) << 2
        | (self.sprite_table as u8) << 3
        | (self.bg_table as u8) << 4
        | (self.sprite_size as u8) << 5
        | (self.nmi as u8) << 7
    }

    pub fn from_byte(byte:u8)->Self{
        Self{
            base_nametable: byte & 0x3,
            vram_increment: byte & 0x4 != 0,
            sprite_table: byte & 0x8 != 0,
            bg_table: byte & 0x10 != 0,
            sprite_size: byte & 0x20 != 0,
            nmi: byte & 0x80 != 0,
        }
    }
}
