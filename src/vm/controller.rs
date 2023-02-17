use crate::vm::display::{Display, SystemDisplay};
use crate::vm::state::{Register, State};
use crate::vm::{decoder, ARCH_BYTES};

pub struct Controller {
    state: State,
    display: Box<dyn Display>,
    initial_ip_value: u32,
}

impl Controller {
    pub fn new(state: State) -> Self {
        let initial_ip_value = state.get_register(Register::IP);
        Controller {
            state,
            display: Box::new(SystemDisplay::new()),
            initial_ip_value,
        }
    }

    pub fn execute(&mut self) {
        while !self.is_finished() {
            self.step()
        }
        self.reset_machine();
    }

    pub fn get_mut_state(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn get_mut_display(&mut self) -> &mut dyn Display {
        self.display.as_mut()
    }

    fn reset_machine(&mut self) {
        self.state.set_register(Register::IP, self.initial_ip_value);
        self.state.set_register(Register::END, 0);
    }

    fn step(&mut self) {
        let instruction = self.fetch();
        let command = decoder::decode(instruction);
        command.execute(self);
        self.next();
    }

    fn next(&mut self) {
        let ip_value = self.state.get_register(Register::IP);
        self.state.set_register(Register::IP, ip_value + ARCH_BYTES);
    }

    fn fetch(&mut self) -> &[u8] {
        let ip_value = self.state.get_register(Register::IP);
        self.state.get_memory_handler().read_word(ip_value)
    }

    fn is_finished(&self) -> bool {
        self.state.get_register(Register::END) != 0
    }
}
