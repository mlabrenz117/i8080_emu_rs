use crate::io::basic_io::BasicIO;
use crate::io::IO;
use crate::mmu::basic_mmu::BasicMMU;
use crate::mmu::Mmu;
use crate::mmu::Rom;
use crate::pic::InterruptController;

pub struct Interconnect<T: Mmu, U: IO> {
    pub mmu: T,
    pub io: U,
    pub interrupt_controller: InterruptController,
}

impl Interconnect<BasicMMU, BasicIO> {
    pub fn new<U: Into<Rom>>(rom: U) -> Interconnect<BasicMMU, BasicIO> {
        Interconnect {
            mmu: BasicMMU::new(rom),
            io: BasicIO::default(),
            interrupt_controller: InterruptController::default(),
        }
    }
}

impl<T: Mmu, U: IO> Interconnect<T, U> {
    pub fn with_mmu(self, mmu: T) -> Interconnect<T, U> {
        Interconnect {
            mmu,
            io: self.io,
            interrupt_controller: self.interrupt_controller,
        }
    }

    pub fn with_io(self, io: U) -> Interconnect<T, U> {
        Interconnect {
            mmu: self.mmu,
            io,
            interrupt_controller: self.interrupt_controller,
        }
    }
}
