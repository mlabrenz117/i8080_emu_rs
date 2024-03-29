pub mod basic_io;

pub trait IO {
    fn read_port(&self, port: u8) -> u8;
    fn write_port(&mut self, port: u8, value: u8);
}
