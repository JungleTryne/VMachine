use std::path::Path;
use crate::vm::controller::Controller;
use crate::vm::memory::VirtualMemory;
use crate::vm::state::State;

pub mod vm;

fn main() {
    let memory = VirtualMemory::new(Path::new("./images/image.bin"));
    let state = State::new(memory);
    let mut controller = Controller::new(state);
    controller.execute();
}
