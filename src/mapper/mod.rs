mod nrom;
pub use nrom::NROM;

use crate::rom::Rom;

pub trait Mapper {
    fn cpu_read(&mut self, addr: u16) -> Option<u8>;
    fn cpu_write(&mut self, addr: u16, val: u8);
    fn ppu_read(&mut self, addr: u16) -> Option<u8>;
    fn ppu_write(&mut self, addr: u16, val: u8);
    fn swap_prg_rom(&mut self, rom: Rom);
    fn swap_chr_rom(&mut self, rom: Rom);
}
