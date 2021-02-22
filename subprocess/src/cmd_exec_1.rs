use std::process::Command;
use std::io::{self, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    println!("Welcome to dummy command executor v1!");
    loop {
        print!("$ ");
        io::stdout().flush().expect("fail to flush the stdout");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp)?;
        let cmd_to_exec = inp.trim();
        let mut child_cmd = Command::new(cmd_to_exec).
            spawn().
            expect("fail to spawn the command");
        child_cmd.wait().expect("fail to execute the command");
    }
}
