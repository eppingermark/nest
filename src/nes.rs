use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use crate::cpu::Bus;
use crate::ppu::{Ppu, VBus};
use crate::{card::Card, cpu::Cpu, mapper::NROM, rom::Rom};
use crate::js::{consoleLog, updateCRom, updatePRom};

#[wasm_bindgen]
pub struct Nes {
    cpu: Cpu,
    ppu: Rc<RefCell<Ppu>>,
    card: Rc<RefCell<Card>>,
}


#[wasm_bindgen]
impl Nes {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rom_bytes = include_bytes!("../test_roms/4_TheStack.nes").to_vec();

        let header = &[0x4E, 0x45, 0x53, 0x1A];
        if !rom_bytes.starts_with(header) {
            consoleLog("ROM does not start with an INES header");
            panic!();
        }

        let prg_size = rom_bytes[4] as usize * 16 * 1024;
        let chr_size = rom_bytes[5] as usize * 8 * 1024;
        let vertical_mirror = rom_bytes[6] & 1 != 0;

        consoleLog(format!("Sizes PRG {prg_size} CHR {chr_size} Allocate CHR_RAM? {:?}", chr_size == 0).as_str());

        rom_bytes.drain(0..16);

        let mut prg_rom = Rom::with_capacity(prg_size);
        prg_rom.consume_bytes(&mut rom_bytes);
        let mut chr_rom = Rom::with_capacity(chr_size);
        chr_rom.consume_bytes(&mut rom_bytes);

        updatePRom(&prg_rom.contents);
        updateCRom(&chr_rom.contents);

        let mapper = NROM::new(prg_rom, chr_rom);
        let card = Rc::new(RefCell::new(Card::new(Box::new(mapper))));
        let vbus = VBus::new(card.clone(), vertical_mirror, chr_size == 0);
        let ppu = Rc::new(RefCell::new(Ppu::new(vbus)));
        let bus = Bus::new(card.clone(), ppu.clone());
        let cpu = Cpu::new(bus);

        Self {
            cpu,
            card,
            ppu
        }
    }

    #[wasm_bindgen]
    pub async fn reset(&mut self) {
        self.cpu.reset().await;

        consoleLog(format!(
            "CPU Registers: A = {:#x}; X = {:#x}; Y = {:#x}; PC = {:#x}; Stack = {:#x}",
            self.cpu.reg_a,
            self.cpu.reg_x,
            self.cpu.reg_y,
            self.cpu.counter,
            self.cpu.stack
        ).as_str());
        consoleLog(format!("RAM:\n{:?}", self.cpu.bus.ram).as_str());
        consoleLog(format!("CHR_RAM:\n{:?}", self.ppu.borrow().vbus.chr_ram).as_str());
    }

    #[wasm_bindgen]
    pub fn clock(&mut self) -> usize {
        let cpu_cycles = self.cpu.clock();

        for _ in 0..(cpu_cycles * 3) {
            self.ppu_clock();
        }

        cpu_cycles
    }

    #[wasm_bindgen]
    pub fn cpu_clock(&mut self) -> usize {
        self.cpu.clock()
    }

    #[wasm_bindgen]
    pub fn ppu_clock(&mut self) {
        self.ppu.borrow_mut().clock();
    }

    #[wasm_bindgen]
    pub fn is_running(&self) -> bool {
        self.cpu.running
    }

    #[wasm_bindgen]
    pub async fn swap_rom(&mut self, mut rom_bytes: Vec<u8>) {
        let header = &[0x4E, 0x45, 0x53, 0x1A];
        if !rom_bytes.starts_with(header) {
            consoleLog("ROM does not start with an INES header");
            panic!();
        }

        let prg_size = rom_bytes[4] as usize * 16 * 1024;
        let chr_size = rom_bytes[5] as usize * 8 * 1024;

        consoleLog(format!("Sizes PRG {prg_size} CHR {chr_size}").as_str());

        rom_bytes.drain(0..16);
        updatePRom(&rom_bytes);

        let mut prg_rom = Rom::with_capacity(prg_size);
        prg_rom.consume_bytes(&mut rom_bytes);
        let mut chr_rom = Rom::with_capacity(chr_size);
        chr_rom.consume_bytes(&mut rom_bytes);

        let mut card = self.card.borrow_mut();

        card.mapper.swap_prg_rom(prg_rom);
        card.mapper.swap_chr_rom(chr_rom);

        self.cpu.reset().await;
    }

    #[wasm_bindgen]
    pub fn get_screen_buffer(&mut self) -> Vec<u8> {
        self.ppu.borrow().screen_buffer.as_slice().to_vec()
    }
}
