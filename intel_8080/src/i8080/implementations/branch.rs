use crate::{
    i8080::{concat_bytes, error::EmulateError, Result, I8080},
    instruction::{InstructionData, Opcode},
    mmu::Mmu,
};

impl I8080 {
    pub(crate) fn jmp(&mut self, data: InstructionData) -> Result<()> {
        if let (Some(hi), Some(lo)) = data.tuple() {
            let addr = concat_bytes(hi, lo);
            self.pc = addr;
        } else {
            return Err(EmulateError::InvalidInstructionData {
                opcode: Opcode::JMP,
                data,
            });
        }
        Ok(())
    }

    pub(crate) fn jnz(&mut self, data: InstructionData) -> Result<()> {
        if !self.flags.z {
            self.jmp(data)?;
        }
        Ok(())
    }

    pub(crate) fn jz(&mut self, data: InstructionData) -> Result<()> {
        if self.flags.z {
            self.jmp(data)?;
        }
        Ok(())
    }

    pub(crate) fn jnc(&mut self, data: InstructionData) -> Result<()> {
        if self.flags.cy {
            self.jmp(data)?;
        }
        Ok(())
    }

    pub(crate) fn call<T: Mmu>(
        &mut self,
        data: InstructionData,
        interconnect: &mut T,
    ) -> Result<()> {
        if let (Some(hi), Some(lo)) = data.tuple() {
            let addr = concat_bytes(hi, lo);
            self.push_u16(self.pc, interconnect)?;
            self.pc = addr;
        } else {
            return Err(EmulateError::InvalidInstructionData {
                opcode: Opcode::CALL,
                data,
            });
        }
        Ok(())
    }

    pub(crate) fn ret<T: Mmu>(&mut self, interconnect: &mut T) -> Result<()> {
        let addr = self.pop_u16(interconnect)?;
        self.pc = addr;
        Ok(())
    }
}
