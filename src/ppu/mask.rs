#[derive(Default, Clone)]
pub struct PpuMask {
    pub mask_bg: bool,
    pub mask_sprites: bool,
    pub render_bg: bool,
    pub render_sprites: bool,
}

impl PpuMask {
    pub fn to_byte(&self) ->u8 {
        self.mask_bg as u8 |
            (self.mask_sprites as u8) << 1 |
            (self.render_bg as u8) << 2 |
            (self.render_sprites as u8) << 3
    }

    pub fn from_byte(byte: u8) -> Self {
        Self {
            mask_bg: byte & 0x1 != 0,
            mask_sprites: byte & 0x2 != 0,
            render_bg: byte & 0x4 != 0,
            render_sprites: byte & 0x8 != 0,
        }
    }

    pub fn read_and_clear_vblank(&mut self) -> u8 {
        let v = (self.render_sprites as u8) << 7;
        self.render_sprites = false;
        v
    }
}
