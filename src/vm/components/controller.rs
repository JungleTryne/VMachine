use crate::vm::components::display::{Display, SystemDisplay};
use crate::vm::components::state::{Register, State};
use crate::vm::arch::ARCH_BYTES;
use crate::vm::arch::instruction::decode;

/// # Controller
/// Simulates controller component of the virtual machine
/// Stores the state of the machine and changes it
/// by executing given instructions.
///
/// Controller pipeline:
/// - fetch
/// - decode
/// - execute
///
pub struct Controller {
    state: State,
    display: Box<dyn Display>,
    initial_ip_value: u32,
}

impl Controller {
    pub fn new(state: State) -> Self {
        let initial_ip_value = state.register(Register::IP);
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

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn mut_state(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn display(&self) -> &dyn Display {
        self.display.as_ref()
    }

    pub fn mut_display(&mut self) -> &mut dyn Display {
        self.display.as_mut()
    }

    fn reset_machine(&mut self) {
        self.state.set_register(Register::IP, self.initial_ip_value);
        self.state.set_register(Register::END, 0);
    }

    fn step(&mut self) {
        let instruction = self.fetch();
        let command = decode(instruction);
        command.execute(self);
        if command.move_ip() {
            self.next();
        }
    }

    fn next(&mut self) {
        let ip_value = self.state.register(Register::IP);
        self.state.set_register(Register::IP, ip_value + ARCH_BYTES);
    }

    fn fetch(&mut self) -> &[u8] {
        let ip_value = self.state.register(Register::IP);
        self.state.get_memory_handler().read_word(ip_value)
    }

    fn is_finished(&self) -> bool {
        self.state.register(Register::END) != 0
    }
}
