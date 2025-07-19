use std::fmt;

use crate::js::updateRam;

pub struct Ram {
    contents: [u8; 0x800],
}

impl Ram {
    pub fn read(&self, addr: u16) -> Option<u8> {
        if let Some(byte) = self.contents.get(addr as usize) {
            Some(byte.clone())
        } else {
            None
        }
    }

    pub fn write(&mut self, addr: u16, byte: u8) {
        self.contents[addr as usize] = byte;
        updateRam(&self.contents);
    }
}

impl Default for Ram {
    fn default() -> Self {
        let contents = [0u8; 0x800];
        updateRam(&contents);

        Self {
            contents,
        }
    }
}

impl fmt::Debug for Ram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "      ")?;
        for col in 0..16 {
            write!(f, "{:02X} ", col)?;
        }
        writeln!(f)?;

        for row in 0..(self.contents.len() / 16) {
            let addr = row * 16;
            write!(f, "0x{:03X} ", addr)?;

            for col in 0..16 {
                let i = addr + col;
                write!(f, "{:02X} ", self.contents[i])?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
