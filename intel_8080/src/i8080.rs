use crate::{
    instruction::{Instruction, Opcode},
    interconnect::Interconnect,
    io::IO,
    mmu::Mmu,
};

use log::info;
use std::fmt::{self, Display};

mod flags;
pub use self::flags::ConditionalFlags;

mod register;
use self::register::Reg;
pub use self::register::Register;

mod error;
use self::error::EmulateError;

type Result<T> = std::result::Result<T, EmulateError>;

// Instruction Implementations
mod implementations;

pub struct I8080 {
    a: Reg<u8>,
    b: Reg<u8>,
    c: Reg<u8>,
    d: Reg<u8>,
    e: Reg<u8>,
    h: Reg<u8>,
    l: Reg<u8>,
    sp: Reg<u16>,
    pc: Reg<u16>,
    flags: ConditionalFlags,
    interrupts_enabled: bool,
}

impl I8080 {
    pub fn new() -> I8080 {
        I8080 {
            a: Reg::new(0),
            b: Reg::new(0),
            c: Reg::new(0),
            d: Reg::new(0),
            e: Reg::new(0),
            h: Reg::new(0),
            l: Reg::new(0),
            sp: Reg::new(0),
            pc: Reg::new(0),
            flags: ConditionalFlags::new(),
            interrupts_enabled: true,
        }
    }

    pub fn emulate_instruction<T: Mmu, U: IO>(
        &mut self,
        instruction: Instruction,
        interconnect: &mut Interconnect<T, U>,
        is_interrupt: bool,
    ) -> Result<()> {
        use self::Opcode::*;

        let mmu = &mut interconnect.mmu;
        let io = &mut interconnect.io;

        let old_pc: u16 = self.pc.clone();
        match is_interrupt {
            true => self.interrupts_enabled = false,
            false => self.pc.set(old_pc + instruction.len()),
        }

        self.reset_rc();
        let r = match instruction.opcode() {
            NOP => Ok(()),
            // Data transfer Instructions
            LXI(r) => self.lxi(r, instruction.data()),
            LDAX(r) => self.ldax(r, mmu),
            LDA => self.lda(instruction.data(), mmu),
            STA => self.sta(instruction.data(), mmu),
            MOV(d, s) => self.mov(d, s, mmu),
            MVI(r) => self.mvi(r, instruction.data(), mmu),
            XCHG => self.xchg(),
            PUSH(r) => self.push(r, mmu),
            POP(r) => self.pop(r, mmu),
            // Arithmetic Instructions
            INX(r) => self.inx(r),
            DCX(r) => self.dcx(r),
            INR(r) => self.inr(r, mmu),
            DCR(r) => self.dcr(r, mmu),
            ADD(r) => self.add(r, mmu),
            ADI => self.adi(instruction.data()),
            DAD(r) => self.dad(r),
            SUB(r) => self.sub(r, mmu),
            SUI => self.sui(instruction.data()),
            RRC => self.rrc(),
            // Logical Instructions
            CPI => self.cpi(instruction.data()),
            ANI => self.ani(instruction.data()),
            ANA(r) => self.ana(r, mmu),
            XRA(r) => self.xra(r, mmu),
            // IO Instructions
            OUT => self.out(instruction.data(), io),
            IN => self.input(instruction.data(), io),
            // Branch Instructions
            JMP => self.jmp(instruction.data()),
            JZ => self.jz(instruction.data()),
            JNZ => self.jnz(instruction.data()),
            JNC => self.jnc(instruction.data()),
            CALL => self.call(instruction.data(), mmu),
            RET => self.ret(mmu),
            // Special Instructions
            EI => self.ei(),
            _op => return Err(EmulateError::UnimplementedInstruction { instruction }),
        };

        if let Ok(()) = r {
            info!("{}: {}; {}", old_pc, instruction, self);
        }
        r
    }

    fn set_8bit_register(&mut self, register: Register, value: u8) {
        match register {
            Register::A => self.a.set(value),
            Register::B => self.b.set(value),
            Register::C => self.c.set(value),
            Register::D => self.d.set(value),
            Register::E => self.e.set(value),
            Register::H => self.h.set(value),
            Register::L => self.l.set(value),
            Register::M => {
                self.l.set(value);
                self.h.set(0);
            }
            Register::SP => self.sp.set(value as u16),
        };
    }

    pub fn get_8bit_register(&self, register: Register) -> Result<u8> {
        match register {
            Register::A => Ok(*self.a),
            Register::B => Ok(*self.b),
            Register::C => Ok(*self.c),
            Register::D => Ok(*self.d),
            Register::E => Ok(*self.e),
            Register::H => Ok(*self.h),
            Register::L => Ok(*self.l),
            _r => return Err(EmulateError::RegisterNot8Bit { register }),
        }
    }

    pub fn m(&self) -> u16 {
        let high = self.get_8bit_register(Register::H).unwrap() as u16;
        let low = self.get_8bit_register(Register::L).unwrap() as u16;
        high << 8 | low
    }

    fn set_m(&mut self, addr: u16) {
        let (high, low) = split_bytes(addr);
        self.set_8bit_register(Register::H, high);
        self.set_8bit_register(Register::L, low);
    }

    fn set_sp(&mut self, value: u16) {
        self.sp.set(value);
    }

    pub fn sp(&self) -> u16 {
        *self.sp
    }

    pub fn pc(&self) -> u16 {
        *self.pc
    }

    pub fn flags(&self) -> ConditionalFlags {
        self.flags
    }

    pub fn interrupts_enabled(&self) -> bool {
        self.interrupts_enabled
    }

    fn push_u16<T: Mmu>(&mut self, value: u16, mmu: &mut T) -> Result<()> {
        let (high, low) = split_bytes(value);
        self.push_u8(high, mmu)?;
        self.push_u8(low, mmu)?;
        Ok(())
    }

    fn push_u8<T: Mmu>(&mut self, value: u8, mmu: &mut T) -> Result<()> {
        let loc = *self.sp - 1;
        //if loc < 0x2000 {
        //    return Err(EmulateError::StackOverflow);
        //};
        mmu.write_byte(loc, value);
        self.sp.set(loc);
        Ok(())
    }

    fn pop_u8<T: Mmu>(&mut self, mmu: &T) -> Result<u8> {
        let value = mmu.read_byte(*self.sp);
        self.sp.set(*self.sp + 1);
        Ok(value)
    }

    fn pop_u16<T: Mmu>(&mut self, mmu: &T) -> Result<u16> {
        let low = self.pop_u8(mmu)?;
        let high = self.pop_u8(mmu)?;
        Ok(concat_bytes(high, low))
    }

    fn reset_rc(&mut self) {
        self.a.reset_changed();
        self.b.reset_changed();
        self.c.reset_changed();
        self.d.reset_changed();
        self.e.reset_changed();
        self.h.reset_changed();
        self.l.reset_changed();
        self.sp.reset_changed();
    }
}

pub(crate) fn split_bytes(bytes: u16) -> (u8, u8) {
    let low_byte = (bytes & 0x00ff) as u8;
    let high_byte = (bytes & 0xff00) >> 8;
    let high_byte = high_byte as u8;
    (high_byte, low_byte)
}

pub(crate) fn concat_bytes(high: u8, low: u8) -> u16 {
    (high as u16) << 8 | (low as u16)
}

pub trait TwosComplement<RHS=Self> {
    type Output;
    fn complement_sub(&self, subtrahend: RHS) -> Self::Output;
}

impl TwosComplement for u8 {
    type Output = (u8, bool);
    fn complement_sub(&self, subtrahend: Self) -> Self::Output {
        let complement = !subtrahend + 1;
        let (value, carry) = self.overflowing_add(complement);
        (value, !carry)
    }
}

impl Display for I8080 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CPU: a={}|b={}|c={}|d={}|e={}|h={}|l={}|sp={}",
            self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.sp,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{concat_bytes, split_bytes};
    #[test]
    fn can_split_bytes() {
        let (high, low) = split_bytes(0xea14);
        assert_eq!(high, 0xea);
        assert_eq!(low, 0x14);
    }

    #[test]
    fn can_concat_bytes() {
        let low = 0x14;
        let high = 0xea;
        assert_eq!(concat_bytes(high, low), 0xea14);
    }
}
