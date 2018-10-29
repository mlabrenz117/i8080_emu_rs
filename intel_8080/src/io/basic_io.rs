use super::IO;
use log::warn;

#[derive(Default)]
pub struct BasicIO {
    input_0: u8,
    input_1: u8,
    input_2: u8,
    shift_register: ShiftRegister,
}

impl IO for BasicIO {
    fn read_port(&self, port: u8) -> u8 {
        match port {
            0 => self.input_0,
            1 => self.input_1,
            2 => self.input_2,
            3 => self.shift_register.read(),
            _ => unimplemented!("Read for port {} unimplemented!", port),
        }
    }

    fn write_port(&mut self, port: u8, value: u8) {
        match port {
            2 => self.shift_register.offset = value,
            4 => self.shift_register.insert_value(value),
            3 | 5 => warn!("Sound Out Unimplemented: Port {}", port),
            6 => warn!("Watch-dog unimplemented: Port 6"),
            _ => unimplemented!("Write for port {} unimplemented!", port),
        }
    }
}

#[derive(Default)]
struct ShiftRegister {
    value: u16,
    offset: u8,
}

impl ShiftRegister {
    fn read(&self) -> u8 {
        let shift = self.offset % 9;
        let value = self.value << shift;
        let value = value | 0xf0;
        (value >> 8) as u8
    }

    fn insert_value(&mut self, value: u8) {
        let v = self.value >> 8;
        self.value = v | (value as u16) << 8;
    }
}

#[cfg(test)] 
mod test {
    use super::ShiftRegister;

    #[test]
    fn can_read_shift_register() {
        let mut sr = ShiftRegister { value: 0b01101001_11110000, offset: 0 };
        assert_eq!(sr.read(), 0b01101001);
        sr.offset = 1;
        assert_eq!(sr.read(), 0b11010011);
        sr.offset = 2;
        assert_eq!(sr.read(), 0b10100111);
        sr.offset = 3;
        assert_eq!(sr.read(), 0b01001111);
        sr.offset = 4;
        assert_eq!(sr.read(), 0b10011111);
        sr.offset = 5;
        assert_eq!(sr.read(), 0b00111110);
        sr.offset = 6;
        assert_eq!(sr.read(), 0b01111100);
        sr.offset = 7;
        assert_eq!(sr.read(), 0b11111000);
        sr.offset = 8;
        assert_eq!(sr.read(), 0b11110000);
    }

    #[test]
    fn can_insert_shift_value() {
        let mut sr = ShiftRegister { value: 0xaabb, offset: 0 };
        sr.insert_value(0xef);
        assert_eq!(sr.value, 0xefaa);
        sr.insert_value(0x02);
        assert_eq!(sr.value, 0x02ef);
    }
}
