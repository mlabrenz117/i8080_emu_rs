pub mod rom;
pub use self::rom::Rom;

pub mod basic_mmu;

pub trait Mmu {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8);
    fn rom_len(&self) -> usize;
}
