use crate::{
    i8080::{error::EmulateError, Result, I8080, Register},
    instruction::{InstructionData, Opcode},
    io::IO,
};

impl I8080 {
    pub(crate) fn out<U: IO>(&mut self, data: InstructionData, io: &mut U) -> Result<()> {
        if let Some(port) = data.first() {
            io.write_port(port, self.a);
        } else {
            return Err(EmulateError::InvalidInstructionData {
                opcode: Opcode::OUT,
                data,
            });
        }
        Ok(())
    }

    pub(crate) fn input<U: IO>(&mut self, data: InstructionData, io: &mut U) -> Result<()> {
        if let Some(port) = data.first() {
            self.set_8bit_register(Register::A, io.read_port(port));
        } else {
            return Err(EmulateError::InvalidInstructionData {
                opcode: Opcode::IN,
                data,
            });
        }
        Ok(())
    }
}
