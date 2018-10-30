pub mod i8080;
pub mod instruction;
pub mod interconnect;
pub mod io;
pub mod mmu;
pub mod pic;

use log::error;

use self::{
    i8080::I8080,
    instruction::{Instruction, Opcode},
    interconnect::Interconnect,
    io::{basic_io::BasicIO, IO},
    mmu::{basic_mmu::BasicMMU, Mmu, Rom},
};

use failure::Error;

pub struct Emulator<T: Mmu, U: IO> {
    cpu: I8080,
    pub interconnect: Interconnect<T, U>,
}

impl Emulator<BasicMMU, BasicIO> {
    pub fn new<U: Into<Rom>>(rom: U) -> Emulator<BasicMMU, BasicIO> {
        Emulator {
            cpu: I8080::new(),
            interconnect: Interconnect::new(rom),
        }
    }
}

impl<T: Mmu, U: IO> Emulator<T, U> {
    pub fn step(&mut self) {
        if let Err(e) = self.try_step() {
            error!("{}", e);
        }
    }

    pub fn try_step(&mut self) -> Result<(), Error> {
        if self.cpu.interrupts_enabled() {
            if let Some(instruction) = self.interconnect.interrupt_controller.consume_interrupt() {
                self.cpu
                    .emulate_instruction(instruction, &mut self.interconnect, true)?;
            }
        }
        if let Some(instruction) = self.next_instruction() {
            self.cpu
                .emulate_instruction(instruction, &mut self.interconnect, false)?;
        }
        Ok(())
    }

    pub fn run(&mut self) {
        if let Err(e) = self.try_run() {
            error!("{}", e);
        }
    }

    pub fn try_run(&mut self) -> Result<(), Error> {
        if self.cpu.interrupts_enabled() {
            if let Some(instruction) = self.interconnect.interrupt_controller.consume_interrupt() {
                self.cpu
                    .emulate_instruction(instruction, &mut self.interconnect, true)?;
            }
        }
        while let Some(instruction) = self.next_instruction() {
            self.cpu
                .emulate_instruction(instruction, &mut self.interconnect, false)?
        }
        Ok(())
    }

    fn next_instruction(&self) -> Option<Instruction> {
        use self::instruction::opcode::OpcodeSize;
        if (self.cpu.pc() as usize) >= self.mmu().rom_len() {
            None
        } else {
            let opcode = Opcode::from(self.mmu().read_byte(self.cpu.pc()));
            let instruction = match opcode.size() {
                OpcodeSize::Binary => {
                    let data = self.mmu().read_byte(self.cpu.pc() + 1);
                    Instruction::new_binary(opcode, data).unwrap()
                }
                OpcodeSize::Trinary => {
                    let data_low = self.mmu().read_byte(self.cpu.pc() + 1) as u16;
                    let data_high = self.mmu().read_byte(self.cpu.pc() + 2) as u16;
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

    pub fn mmu(&self) -> &impl Mmu {
        &self.interconnect.mmu
    }

    pub fn mmu_mut(&mut self) -> &mut impl Mmu {
        &mut self.interconnect.mmu
    }
}
