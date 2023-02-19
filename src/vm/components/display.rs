use std::io;

pub trait Display {
    fn print(&self, c: char);
    fn get(&mut self) -> char;
}

pub struct SystemDisplay {
    buffer: String,
}

#[allow(clippy::new_without_default)]
impl SystemDisplay {
    pub fn new() -> Self {
        SystemDisplay {
            buffer: String::new(),
        }
    }
}

impl Display for SystemDisplay {
    fn print(&self, c: char) {
        print!("{}", c);
    }

    fn get(&mut self) -> char {
        if self.buffer.is_empty() {
            let _ = io::stdin().read_line(&mut self.buffer).unwrap();
            self.buffer = self.buffer.chars().rev().collect();
        }

        self.buffer.pop().unwrap()
    }
}
