use lazy_static::lazy_static;
use alloc::{vec, vec::Vec, sync::Arc, string::String};
use spin::Mutex;

lazy_static! {
    pub static ref GLOBAL_COMMAND_BUFFER: Command = Command::new(120);
}
pub struct Command {
    command_buffer: Arc<Mutex<Vec<char>>>,
}
impl Command {
    pub fn new(command_limit: u8) -> Command {
        Command { command_buffer: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn push(&self, value: char) {
        self.command_buffer.lock().push(value);
    }

    pub fn print_command(&self) {
        use crate::{print, println};
        println!("{}", self.command_to_string());
    }

    pub fn clear_command(&self) {
        self.command_buffer.lock().clear();
    }

    pub fn execute_command(&self) {
        use crate::{print, println};
        match (self.command_to_string().as_str())
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
        use crate::{print, println};
        let mut s: String = String::from("");
        for c in self.command_buffer.lock().iter() {
            let c_literal = c.clone();
            if c_literal == '\n' { continue; }
            s.push(c_literal);
        }
        s
    }
}