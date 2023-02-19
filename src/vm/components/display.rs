pub trait Display {
    fn print(&self, c: char);
}

pub struct SystemDisplay {}

#[allow(clippy::new_without_default)]
impl SystemDisplay {
    pub fn new() -> Self {
        SystemDisplay {}
    }
}

impl Display for SystemDisplay {
    fn print(&self, c: char) {
        print!("{}", c);
    }
}
