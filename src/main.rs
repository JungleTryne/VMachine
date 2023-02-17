use crate::vm::controller::Controller;
use crate::vm::memory::VirtualMemory;
use crate::vm::state::State;
use std::path::Path;

pub mod vm;

fn main() {
    let memory = VirtualMemory::new(Path::new("./images/hello_world.bin"));
    let state = State::new(memory);
    let mut controller = Controller::new(state);
    controller.execute();
}
