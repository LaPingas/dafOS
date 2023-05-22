use lazy_static::lazy_static;
use alloc::{vec::Vec, sync::Arc, string::String};
use spin::Mutex;
use crate::{print, println};

lazy_static! {
    pub static ref GLOBAL_TERMINAL_COMMAND_BUFFER: Command = Command::new();
}
pub struct Command {
    command_buffer: Arc<Mutex<Vec<char>>>,
}
impl Command {
    pub fn new() -> Command {
        Command { command_buffer: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn push(&self, value: char) {
        self.command_buffer.lock().push(value);
    }

    pub fn print_command(&self) {
        println!("{}", self.command_to_string());
    }

    pub fn clear_command(&self) {
        self.command_buffer.lock().clear();
    }

    pub fn execute_command(&self) {
        match self.command_to_string().as_str()
        {
            "Uriel" => {
                print!("brrrrrrrrrrrrrrrrrrrrr");
            },
            &_ => {
                print!("Command not found")
            }
        }
        println!();
        print!("> ");
    }

    fn command_to_string(&self) -> String {
        let mut s: String = String::from("");
        for c in self.command_buffer.lock().iter() {
            let c_literal = c.clone();
            if c_literal == '\n' { continue; }
            s.push(c_literal);
        }
        s
    }
}