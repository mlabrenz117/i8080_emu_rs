use std::fmt::{self, Display};
use std::ops::{AddAssign, SubAssign, Deref};

use crate::i8080::TwosComplement;

pub struct Reg<T> {
    value: T,
    has_changed: bool,
}

impl<T: Copy> Reg<T> {
    pub fn new(value: T) -> Self {
        Reg {
            value,
            has_changed: false,
        }
    }
    pub fn set(&mut self, value: T) {
        self.value = value;
        self.has_changed = true;
    }
    
    pub fn reset_changed(&mut self) {
        self.has_changed = false;
    }
}

impl<T: Copy> SubAssign<T> for Reg<T> {
    fn sub_assign(&mut self, other: T) {
        self.set(other);
    }
}

impl<T: Copy> AddAssign<T> for Reg<T> {
    fn add_assign(&mut self, other: T) {
        self.set(other);
    }
}

impl<T> Deref for Reg<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Display + fmt::LowerHex> Display for Reg<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use colored::*;
        let repr = match self.has_changed {
            true => format!("{:02x}", self.value).blue(),
            false => format!("{:02x}", self.value).white(),
        };
        write!(f, "{}", repr)
    }
}

impl<U, T: TwosComplement<U>> TwosComplement<U> for Reg<T> {
    type Output = T::Output;
    
    fn complement_sub(&self, subtrahend: U) -> Self::Output {
        self.value.complement_sub(subtrahend)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M,
    SP,
}

impl Register {
    pub fn get_pair(&self) -> Option<Register> {
        match self {
            Register::B => Some(Register::C),
            Register::D => Some(Register::E),
            Register::H => Some(Register::L),
            _ => None,
        }
    }

    pub fn is_8bit(&self) -> bool {
        match self {
            Register::A => true,
            Register::B => true,
            Register::C => true,
            Register::D => true,
            Register::E => true,
            Register::H => true,
            Register::L => true,
            Register::M => false,
            Register::SP => false,
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Register::A => "A",
            Register::B => "B",
            Register::C => "C",
            Register::D => "D",
            Register::E => "E",
            Register::H => "H",
            Register::L => "L",
            Register::M => "M",
            Register::SP => "S",
        };
        write!(f, "{}", s)
    }
}
