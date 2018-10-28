pub mod i8080;
pub mod instruction;
pub mod interconnect;

pub(crate) mod mem_map;

use log::error;

use self::{
    i8080::I8080,
    instruction::{Instruction, Opcode},
    interconnect::{Mmu, Interconnect, Rom},
};

use failure::Error;

pub struct Emulator<T: Mmu> {
    cpu: I8080,
    interconnect: T,
}

impl Emulator<Interconnect> {
    pub fn new<U: Into<Rom>>(rom: U) -> Emulator<Interconnect> {
        Emulator {
            cpu: I8080::new(),
            interconnect: Interconnect::new(rom),
        }
    }
}

impl<T: Mmu> Emulator<T> {
    pub fn with_mmu(mmu: T) -> Emulator<T> {
        Emulator {
            cpu: I8080::new(),
            interconnect: mmu,
        }
    }

    pub fn step(&mut self) {
        if let Some(instruction) = self.next_instruction() {
            if let Err(e) = self
                .cpu
                .emulate_instruction(instruction, &mut self.interconnect)
            {
                error!("{}", e);
            }
        }
    }

    pub fn try_step(&mut self) -> Result<(), Error> {
        if let Some(instruction) = self.next_instruction() {
            self.cpu
                .emulate_instruction(instruction, &mut self.interconnect)?;
        }
        Ok(())
    }

    pub fn run(&mut self) {
        while let Some(instruction) = self.next_instruction() {
            if let Err(e) = self
                .cpu
                .emulate_instruction(instruction, &mut self.interconnect)
            {
                error!("{}", e);
                break;
            }
        }
    }

    pub fn try_run(&mut self) -> Result<(), Error> {
        while let Some(instruction) = self.next_instruction() {
            self.cpu
                .emulate_instruction(instruction, &mut self.interconnect)?
        }
        Ok(())
    }

    fn next_instruction(&self) -> Option<Instruction> {
        use self::instruction::opcode::OpcodeSize;
        if (self.cpu.pc() as usize) >= self.interconnect.rom_len() {
            None
        } else {
            let opcode = Opcode::from(self.interconnect.read_byte(self.cpu.pc()));
            let instruction = match opcode.size() {
                OpcodeSize::Binary => {
                    let data = self.interconnect.read_byte(self.cpu.pc() + 1);
                    Instruction::new_binary(opcode, data).unwrap()
                }
                OpcodeSize::Trinary => {
                    let data_low = self.interconnect.read_byte(self.cpu.pc() + 1) as u16;
                    let data_high = self.interconnect.read_byte(self.cpu.pc() + 2) as u16;
                    let addr = (data_high << 8) | data_low;
                    Instruction::new_trinary(opcode, addr).unwrap()
                }
                OpcodeSize::Unary => Instruction::new_unary(opcode).unwrap(),
            };
            Some(instruction)
        }
    }

    pub fn cpu(&self) -> &I8080 {
        &self.cpu
    }

    pub fn cpu_mut(&mut self) -> &mut I8080 {
        &mut self.cpu
    }

    pub fn interconnect(&self) -> &impl Mmu {
        &self.interconnect
    }

    pub fn interconnect_mut(&mut self) -> &mut impl Mmu {
        &mut self.interconnect
    }
}
