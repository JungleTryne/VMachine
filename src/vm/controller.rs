use crate::vm::instruction::Instruction;
use crate::vm::decoder;
use crate::vm::display::{Display, SystemDisplay};
use crate::vm::state::{Register, State};

pub struct Controller {
    state: State,
    display: Box<dyn Display>,
}

impl Controller {
    pub fn new(state: State) -> Self {
        Controller {
            state,
            display: Box::new(SystemDisplay::new()),
        }
    }

    pub fn execute(&mut self) {
        while !self.is_finished() {
            self.step()
        }
    }

    pub fn get_mut_state(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn get_mut_display(&mut self) -> &mut dyn Display {
        self.display.as_mut()
    }

    fn step(&mut self) {
        let command = self.fetch();
        command.execute(self);
        self.next();
    }

    fn next(&mut self) {
        let ip_value = self.state.get_register(Register::IP);
        self.state.set_register(Register::IP, ip_value + 4);
    }

    fn fetch(&mut self) -> Box<dyn Instruction> {
        let ip_value = self.state.get_register(Register::IP);
        let instruction = self.state.get_memory_handler().read_code(ip_value);
        decoder::decode(instruction)
    }

    fn is_finished(&self) -> bool {
        self.state.get_register(Register::END) != 0
    }
}
