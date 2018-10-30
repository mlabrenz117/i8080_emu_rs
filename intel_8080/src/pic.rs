use crate::instruction::Instruction;

#[derive(Default)]
pub struct InterruptController {
    interrupt: Option<Instruction>,
}

impl InterruptController {
    pub fn generate_interrupt(&mut self, instruction: Instruction) {
        self.interrupt = Some(instruction);
    }

    pub fn interrupt(&self) -> Option<Instruction> {
        self.interrupt
    }
}
