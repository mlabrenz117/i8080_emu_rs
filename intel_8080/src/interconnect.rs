use log::error;

mod game_pad;
pub mod rom;
mod vram;
mod wram;

use self::game_pad::GamePad;
pub use self::rom::Rom;
use self::vram::Vram;
use self::wram::Wram;

use crate::mem_map::*;

pub trait Mmu {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8);
    fn rom_len(&self) -> usize;
}

pub struct Interconnect {
    rom: Rom,
    wram: Wram,
    vram: Vram,
    game_pad: GamePad,
}

impl Interconnect {
    pub fn new<T: Into<Rom>>(rom: T) -> Interconnect {
        let rom = rom.into();
        Interconnect {
            rom,
            wram: Wram::new(),
            vram: Vram::new(),
            game_pad: GamePad::new(),
        }
    }
}

impl Mmu for Interconnect {
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
