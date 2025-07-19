use crate::mapper::Mapper;

pub struct Card {
    pub mapper: Box<dyn Mapper>
}

impl Card {
    pub fn new(mapper: Box<dyn Mapper>) -> Self {
        Self {
            mapper
        }
    }

    pub fn cpu_read(&mut self, addr: u16) -> Option<u8> {
        self.mapper.cpu_read(addr)
    }

    pub fn cpu_write(&mut self, addr: u16, val: u8) {
        self.mapper.cpu_write(addr, val);
    }

    pub fn ppu_read(&mut self, addr: u16) -> Option<u8> {
        self.mapper.ppu_read(addr)
    }

    pub fn ppu_write(&mut self, addr: u16, val: u8) {
        self.mapper.ppu_write(addr, val);
    }
}
