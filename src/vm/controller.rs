use crate::vm::command::Command;
use crate::vm::decoder;
use crate::vm::state::{Register, State};

pub struct Controller {
    state: State,
}

impl Controller {
    pub fn new(state: State) -> Self {
        Controller { state }
    }

    pub fn execute(&mut self) {
        while !self.is_finished() {
            let command = self.fetch();
            command.execute(&mut self.state);
        }
    }

    fn fetch(&mut self) -> Box<dyn Command> {
        let ip_value = self.state.get_register(Register::IP);
        let instruction = self.state.get_memory_handler().read_code(ip_value);
        decoder::decode(instruction)
    }

    fn is_finished(&self) -> bool {
        self.state.get_register(Register::END) != 0
    }
}
