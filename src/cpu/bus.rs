use std::{cell::RefCell, rc::Rc};

use crate::{card::Card, cpu::Ram, js::consoleLog, ppu::{Ppu, PpuCtrl, PpuMask}};

pub struct Bus {
    pub address: u16,
    pub data: u8,
    pub ram: Ram,
    pub card: Rc<RefCell<Card>>,
    ppu: Rc<RefCell<Ppu>>
}

impl Bus {
    pub fn new(card: Rc<RefCell<Card>>, ppu: Rc<RefCell<Ppu>>) -> Self {
        Self {
            address: 0,
            data: 0,
            ram: Ram::default(),
            card,
            ppu
        }
    }

    pub fn read(&mut self) {
        match self.address {
            0x0000..=0x1FFF => {
                if let Some(byte) = self.ram.read(self.address) {
                    self.data = byte;
                }
            }

            0x2000 => {
                let ppu = self.ppu.borrow();
                self.data = ppu.ctrl_flags.to_byte();
            }

            0x2001 => {
                let ppu = self.ppu.borrow();
                self.data = ppu.mask_flags.to_byte();
            }

            0x2002 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.status_flags.v_blank = true;
                self.data = ppu.status_flags.read_and_clear_vblank();
            }

            0x2007 => {
                let mut ppu = self.ppu.borrow_mut();
                self.data = ppu.ppu_data_read();
            }

            0x8000..=0xFFFF => {
                let mut card = self.card.borrow_mut();

                if let Some(byte) = card.cpu_read(self.address - 0x8000) {
                    self.data = byte;
                }
            }

            _ => {}
        };
    }

    pub fn write(&mut self) {
        match self.address {
            0x0000..=0x1FFF => {
                self.ram.write(self.address, self.data);
            }

            0x2000 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.ctrl_flags = PpuCtrl::from_byte(self.data);
            }

            0x2001 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.mask_flags = PpuMask::from_byte(self.data);
            }

            0x2006 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.ppu_addr(self.data);
            }

            0x2007 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.ppu_data_write(self.data);
            }

            0x8000..=0xFFFF => {
                let mut card = self.card.borrow_mut();
                card.cpu_write(self.address, self.data);
            }

            _ => {}
        }
    }
}
