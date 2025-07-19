use crate::{mapper::Mapper, rom::Rom};

pub struct NROM {
    pub prg_rom: Rom,
    pub chr_rom: Rom,
}

impl NROM {
    pub fn new(prg_rom: Rom, chr_rom: Rom) -> Self {
        Self {
            prg_rom,
            chr_rom
        }
    }
}

impl Mapper for NROM {
    fn cpu_read(&mut self, addr: u16) -> Option<u8> {
        return self.prg_rom.read(addr);
    }

    fn cpu_write(&mut self, _addr: u16, _val: u8) {
        // do nothing since the NROM is basic and we cant write to ROM
    }

    fn ppu_read(&mut self, addr: u16) -> Option<u8> {
        return self.chr_rom.read(addr);
    }

    fn ppu_write(&mut self, _addr: u16, _val: u8) {
        // do nothing since the NROM is basic and we cant write to ROM
    }

    fn swap_prg_rom(&mut self, rom: Rom) {
        self.prg_rom = rom;
    }

    fn swap_chr_rom(&mut self, rom: Rom) {
        self.chr_rom = rom
    }
}

