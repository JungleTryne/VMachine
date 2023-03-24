use std::collections::VecDeque;
use std::io;
use std::io::Write;
use std::str::FromStr;

pub trait Display {
    fn print(&self, c: char);
    fn get(&mut self) -> char;
    fn get_num(&mut self) -> u32;
}

pub struct SystemDisplay {
    buffer: VecDeque<char>,
}

#[allow(clippy::new_without_default)]
impl SystemDisplay {
    pub fn new() -> Self {
        SystemDisplay {
            buffer: VecDeque::new(),
        }
    }

    fn check_fill_buffer(&mut self) {
        if self.buffer.is_empty() {
            let mut line = String::new();
            let _ = io::stdin().read_line(&mut line).unwrap();
            let line = line.trim();
            for c in line.chars() {
                self.buffer.push_back(c);
            }
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
        self.buffer.pop_front().unwrap()
    }

    fn get_num(&mut self) -> u32 {
        self.check_fill_buffer();

        let line: String = self.buffer.clone().into_iter().collect();
        self.buffer = VecDeque::new();

        println!("Got number: {}", line);

        u32::from_str(&line).expect("Couldn't parse a number")
    }
}
