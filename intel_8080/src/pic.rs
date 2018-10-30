use crate::instruction::Instruction;

#[derive(Default)]
pub struct InterruptController {
    interrupt: Option<Instruction>,
}

impl InterruptController {
    pub fn generate_interrupt(&mut self, instruction: Instruction) {
        self.interrupt = Some(instruction);
    }

    pub fn consume_interrupt(&mut self) -> Option<Instruction> {
        match self.interrupt {
            Some(_) => {
                let ret = self.interrupt;
                self.interrupt = None;
                ret
            }
            None => None,
        }
    }
}
