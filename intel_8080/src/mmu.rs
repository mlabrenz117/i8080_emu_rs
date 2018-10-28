pub mod interconnect;

pub trait Mmu {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8);
    fn rom_len(&self) -> usize;
}
