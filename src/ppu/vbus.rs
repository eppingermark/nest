use std::{cell::RefCell, rc::Rc};

use crate::{card::Card, js::updateCRom, ppu::vram::VRam};

pub struct VBus {
    pub address: u16,
    pub data: u8,
    pub vram: VRam,
    pub card: Rc<RefCell<Card>>,
    pub chr_ram: Option<[u8; 0x2000]>,
    pub palette_ram: [u8; 0x20],
    pub vertical_mirror: bool
}

impl VBus {
    pub fn new(card: Rc<RefCell<Card>>, vertical_mirror: bool, allocate_chr_ram: bool) -> Self {
        let mut chr_ram = None;

        if allocate_chr_ram {
            chr_ram = Some([0u8; 0x2000]);
        }

        Self {
            address: 0,
            data: 0,
            vram: VRam::default(),
            card,
            chr_ram,
            vertical_mirror,
            palette_ram: [0u8; 0x20],
        }
    }

    pub fn read(&mut self) {
        match self.address {
            0x0000..=0x1FFF => {
                if let Some(chr_ram) = self.chr_ram.as_ref() {
                    self.data = chr_ram[self.address as usize];
                    return;
                }

                let mut card = self.card.borrow_mut();
                if let Some(byte) = card.ppu_read(self.address) {
                    self.data = byte;
                }
            }

            0x2000..=0x3EFF => {
                let mut addr = self.address & 0x0FFF;

                if self.vertical_mirror {
                    addr %= 0x800;
                } else {
                    addr = (addr & 0x3FF) | ((addr & 0x800) >> 1);
                }

                if let Some(byte) = self.vram.read(addr) {
                    self.data = byte;
                }
            }

            0x3F00..=0x3FFF => {
                let mut addr = (self.address & 0x1F) as usize;

                if addr == 0x10 { addr = 0x00; }
                if addr == 0x14 { addr = 0x04; }
                if addr == 0x18 { addr = 0x08; }
                if addr == 0x1C { addr = 0x0C; }

                self.data = self.palette_ram[addr];
            }

            _ => {}
        };
    }

    pub fn write(&mut self) {
        match self.address {
            0x0000..=0x1FFF => {
                if let Some(chr_ram) = self.chr_ram.as_mut() {
                    chr_ram[self.address as usize] = self.data;
                    return;
                }

                let mut card = self.card.borrow_mut();
                card.ppu_write(self.address, self.data);
            }

            0x2000..=0x3EFF => {
                let mut addr = self.address & 0x0FFF;

                if self.vertical_mirror {
                    addr %= 0x800;
                } else {
                    addr = (addr & 0x3FF) | ((addr & 0x800) >> 1);
                }

                self.vram.write(addr, self.data);
            }

            0x3F00..=0x3FFF => {
                let mut addr = (self.address & 0x1F) as usize;

                if addr == 0x10 { addr = 0x00; }
                if addr == 0x14 { addr = 0x04; }
                if addr == 0x18 { addr = 0x08; }
                if addr == 0x1C { addr = 0x0C; }

                self.palette_ram[addr] = self.data & 0x3F;
            }

            _ => {}
        }
    }
}
