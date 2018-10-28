use super::IOPort;

#[derive(Default)]
pub struct BasicIO {
    input_0: u8,
    input_1: u8,
    input_2: u8,
    shift_register: ShiftRegister,
}

impl IOPort for BasicIO {
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
        self.value as u8
    }

    fn insert_value(&mut self, value: u8) {
        let v = self.value >> 8;
        self.value = v | (value as u16) << 8;
    }
}

// TODO: Testing
