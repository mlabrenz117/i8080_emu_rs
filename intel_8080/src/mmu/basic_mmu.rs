use log::error;

pub mod mem_map;

mod game_pad;
mod vram;
mod wram;

use self::game_pad::GamePad;
use self::vram::Vram;
use self::wram::Wram;

use self::mem_map::*;
use super::{Mmu, Rom};

pub struct BasicMMU {
    rom: Rom,
    wram: Wram,
    vram: Vram,
    game_pad: GamePad,
}

impl BasicMMU {
    pub fn new<T: Into<Rom>>(rom: T) -> BasicMMU {
        let rom = rom.into();
        BasicMMU {
            rom,
            wram: Wram::new(),
            vram: Vram::new(),
            game_pad: GamePad::new(),
        }
    }
}

impl Mmu for BasicMMU {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            ROM_START...ROM_END => self.rom.read_byte(addr - ROM_START),
            WRAM_START...WRAM_END => self.wram.read_byte(addr - WRAM_START),
            VRAM_START...VRAM_END => self.vram.read_byte(addr - VRAM_START),
            _ => panic!("Unrecognized Address: 0x{:04x}", addr),
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            ROM_START...ROM_END => error!("Attempting to write to ROM"),
            WRAM_START...WRAM_END => self.wram.write_byte(addr - WRAM_START, value),
            VRAM_START...VRAM_END => self.vram.write_byte(addr - VRAM_START, value),
            _ => panic!("Unrecognized Address: 0x{:04x}", addr),
        }
    }

    fn rom_len(&self) -> usize {
        self.rom.len()
    }
}
