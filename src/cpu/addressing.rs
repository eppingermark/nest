use crate::cpu::Cpu;

impl Cpu {
    pub fn immediate(&mut self) -> (Vec<u8>, u8, String) {
        let operand = self.read_next();
        (vec![operand], operand, format!("#${operand:02X}"))
    }

    pub fn zeropage(&mut self) -> (Vec<u8>, u8, String) {
        let addr = self.read_next();
        let operand = self.read(addr as u16);

        (vec![addr], operand, format!("${operand:02X}"))
    }

    pub fn zeropage_x(&mut self) -> (Vec<u8>, u8, String) {
        let operand = self.read_next();
        let addr = operand.wrapping_add(self.reg_x) as u16;
        self.cycle();
        let operand = self.read(addr);

        (vec![operand], operand, format!("${operand:02X}"))
    }

    pub fn zeropage_y(&mut self) -> (Vec<u8>, u8, String) {
        let operand = self.read_next();
        let addr = operand.wrapping_add(self.reg_y) as u16;
        self.cycle();
        let operand = self.read(addr);

        (vec![operand], operand, format!("${operand:02X}"))
    }

    pub fn absolute(&mut self) -> (Vec<u8>, u8, String) {
        let low = self.read_next();
        let high = self.read_next();
        let addr = (high as u16) << 8 | low as u16;
        let operand = self.read(addr);

        (vec![low, high], operand, format!("${addr:04X}"))
    }

    pub fn absolute_x(&mut self) -> (Vec<u8>, u8, String) {
        let low = self.read_next();
        let high = self.read_next();
        let base_addr = (high as u16) << 8 | low as u16;
        let addr = base_addr.wrapping_add(self.reg_x as u16);

        if base_addr & 0xFF00 != addr & 0xFF00 {
            self.cycle();
        }

        let operand = self.read(addr);
        (vec![low, high], operand, format!("${base_addr:04X},X *"))
    }

    pub fn absolute_y(&mut self) -> (Vec<u8>, u8, String) {
        let low = self.read_next();
        let high = self.read_next();
        let base_addr = (high as u16) << 8 | low as u16;
        let addr = base_addr.wrapping_add(self.reg_y as u16);

        if base_addr & 0xFF00 != addr & 0xFF00 {
            self.cycle();
        }

        let operand = self.read(addr);
        (vec![low, high], operand, format!("${base_addr:04X},Y *"))
    }

    pub fn indirect_x(&mut self) -> (Vec<u8>, u8, String) {
        let zp = self.read_next();
        let ptr = zp.wrapping_add(self.reg_x);
        self.cycle();
        let low = self.read(ptr as u16) as u16;
        let high = self.read(ptr.wrapping_add(1) as u16) as u16;
        let operand = self.read((high as u16) << 8 | low as u16);
        (vec![zp], operand, format!("(${zp:02X},X)"))
    }

    pub fn indirect_y(&mut self) -> (Vec<u8>, u8, String) {
        let zp = self.read_next();
        let low = self.read(zp as u16) as u16;
        let high = self.read(zp.wrapping_add(1) as u16) as u16;
        let base_addr = (high << 8) | low;
        let addr = base_addr.wrapping_add(self.reg_y as u16);

        if base_addr & 0xFF00 != addr & 0xFF00 {
            self.cycle();
        }

        let operand = self.read(addr);
        (vec![zp], operand, format!("(${zp:02X}),Y *"))
    }

    pub fn zeropage_addr(&mut self) -> (Vec<u8>, u16, String) {
        let addr = self.read_next();
        (vec![addr], addr as u16, format!("${addr:02X}"))
    }

    pub fn zeropage_x_addr(&mut self) -> (Vec<u8>, u16, String) {
        let operand = self.read_next();
        let addr = operand.wrapping_add(self.reg_x) as u16;
        self.cycle();
        (vec![operand], addr, format!("${operand:02X},X"))
    }

    pub fn zeropage_y_addr(&mut self) -> (Vec<u8>, u16, String) {
        let operand = self.read_next();
        let addr = operand.wrapping_add(self.reg_y) as u16;
        self.cycle();
        (vec![operand], addr, format!("${operand:02X},Y"))
    }

    pub fn absolute_addr(&mut self) -> (Vec<u8>, u16, String) {
        let low = self.read_next();
        let high = self.read_next();
        let addr = (high as u16) << 8 | low as u16;
        (vec![low, high], addr, format!("${addr:04X}"))
    }

    pub fn absolute_x_addr(&mut self) -> (Vec<u8>, u16, String) {
        let low = self.read_next();
        let high = self.read_next();
        let base_addr = (high as u16) << 8 | low as u16;
        let addr = base_addr.wrapping_add(self.reg_x as u16);

        if base_addr & 0xFF00 != addr & 0xFF00 {
            self.cycle();
        }

        (vec![low, high], addr, format!("${base_addr:04X},X *"))
    }

    pub fn absolute_y_addr(&mut self) -> (Vec<u8>, u16, String) {
        let low = self.read_next();
        let high = self.read_next();
        let base_addr = (high as u16) << 8 | low as u16;
        let addr = base_addr.wrapping_add(self.reg_y as u16);

        if base_addr & 0xFF00 != addr & 0xFF00 {
            self.cycle();
        }

        (vec![low, high], addr, format!("${base_addr:04X},Y *"))
    }

    pub fn indirect_x_addr(&mut self) -> (Vec<u8>, u16, String) {
        let zp = self.read_next();
        let ptr = zp.wrapping_add(self.reg_x);
        self.cycle();
        let low = self.read(ptr as u16) as u16;
        let high = self.read(ptr.wrapping_add(1) as u16) as u16;
        let addr = (high << 8) | low;
        (vec![zp], addr, format!("(${zp:02X},X)"))
    }

    pub fn indirect_y_addr(&mut self) -> (Vec<u8>, u16, String) {
        let zp = self.read_next();
        let low = self.read(zp as u16) as u16;
        let high = self.read(zp.wrapping_add(1) as u16) as u16;
        let base_addr = (high << 8) | low;
        let addr = base_addr.wrapping_add(self.reg_y as u16);

        if base_addr & 0xFF00 != addr & 0xFF00 {
            self.cycle();
        }

        (vec![zp], addr, format!("(${zp:02X}),Y *"))
    }
}
