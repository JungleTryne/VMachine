use crate::vm::components::controller::Controller;
use crate::vm::components::memory::VirtualMemory;
use vm::components::state::State;

pub mod vm;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    image_path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let memory = VirtualMemory::new(&args.image_path);
    let state = State::new(memory);
    let mut controller = Controller::new(state);
    controller.execute();
}
