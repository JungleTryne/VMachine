use std::io;
use std::io::Write;
use std::str::FromStr;

pub trait Display {
    fn print(&self, c: char);
    fn get(&mut self) -> char;
    fn get_num(&mut self) -> u32;
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

    fn check_fill_buffer(&mut self) {
        if self.buffer.is_empty() {
            let _ = io::stdin().read_line(&mut self.buffer).unwrap();
            self.buffer = self.buffer.trim().parse().unwrap();
        }
    }
}

impl Display for SystemDisplay {
    fn print(&self, c: char) {
        print!("{}", c);
        let _ = io::stdout().flush();
    }

    fn get(&mut self) -> char {
        self.check_fill_buffer();
        self.buffer.pop().unwrap()
    }

    fn get_num(&mut self) -> u32 {
        self.check_fill_buffer();

        let line = self.buffer.clone();
        self.buffer = String::new();

        println!("Got number: {}", line);

        u32::from_str(&line).expect("Couldn't parse a number")
    }
}
