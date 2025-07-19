mod status;
mod ctrl;
mod mask;
mod vram;
mod vbus;

pub use vbus::VBus;
pub use status::PpuFlags;
pub use mask::PpuMask;
pub use ctrl::PpuCtrl;

use crate::js::{consoleLog, forceScreenDraw};

pub struct Ppu {
    pub status_flags: PpuFlags,
    pub mask_flags: PpuMask,
    pub ctrl_flags: PpuCtrl,
    pub vbus: VBus,
    pub write_latch: bool,
    pub transfer_address: u16,
    pub vram_address: u16,
    pub temp_vram_addr: u16,
    pub screen_buffer: [u8; 256 * 240 * 4],
    pub dot: usize,
    pub scanline: usize,
}

impl Ppu {
    pub fn new(vbus: VBus) -> Self {
        Self {
            status_flags: PpuFlags::default(),
            mask_flags: PpuMask::default(),
            ctrl_flags: PpuCtrl::default(),
            vbus,
            write_latch: false,
            transfer_address: 0,
            vram_address: 0,
            temp_vram_addr: 0,
            screen_buffer: [0u8; 256 * 240 * 4],
            dot: 0,
            scanline: 0,
        }
    }

    pub fn ppu_addr(&mut self, byte: u8) {
        if !self.write_latch {
            self.temp_vram_addr = (byte as u16 & 0x3FFF) << 8;
        } else {
            self.vram_address = self.temp_vram_addr | byte as u16;
            self.transfer_address = self.vram_address;
        }

        self.write_latch = !self.write_latch;
    }

    pub fn ppu_data_write(&mut self, data: u8) {
        let addr = self.vram_address;
        self.vbus.address = addr;
        self.vbus.data = data;
        self.vbus.write();
        self.vram_address = self.vram_address.wrapping_add(if self.ctrl_flags.vram_increment { 32 } else { 1 });
        self.vram_address &= 0x3FFF;
    }

    pub fn ppu_data_read(&mut self) -> u8 {
        self.vbus.address = self.vram_address;
        self.vbus.read();
        self.vbus.data
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        self.vbus.address = addr;
        self.vbus.read();
        self.vbus.data
    }

    pub fn clock(&mut self) {
        if self.dot == 1 && self.scanline == 241 {
            self.status_flags.v_blank = true;
        } else if self.dot == 1 && self.scanline == 261 {
            self.status_flags.v_blank = false;
        }

        let tile_index = self.read(0x2000 + self.dot as u16 + self.scanline as u16 * 32) as u16;

        for y in 0..8 {
            let low_byte = self.read(tile_index * 16 + y);
            let high_byte = self.read(tile_index * 16 + 8 + y);

            for x in 0..8 {
                let two_bit = ((low_byte >> (7 - x)) & 1) | (((high_byte >> (7 - x)) & 1) << 1);

                let buff_x = x + self.dot * 8;
                let buff_y = y as usize + self.scanline * 8;
                let buff_addr = (buff_y as usize * 256 + buff_x as usize) * 4;
                self.screen_buffer[buff_addr] = two_bit * 85;
                self.screen_buffer[buff_addr + 1] = two_bit * 85;
                self.screen_buffer[buff_addr + 2] = two_bit * 85;
                self.screen_buffer[buff_addr + 3] = 255;
            }
        }

        self.dot += 1;

        if self.dot > 341 {
            self.dot = 0;
            self.scanline += 1;

            if self.scanline > 261 {
                self.scanline = 0;
            }
        }

        forceScreenDraw();
    }
}
