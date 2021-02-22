use std::process::Command;
use std::io::{self, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    println!("Welcome to the dummy command executor v2!");
    loop {
        print!("$ ");
        io::stdout().flush()?;
        let mut inp = String::new();
        io::stdin().read_line(&mut inp)?;
        let cmd_string = inp.trim();
        let cmd_vec = cmd_string.split_whitespace().collect::<Vec<&str>>();
        if cmd_vec.len() == 0 {
            continue;
        }
        
        let mut child_cmd = Command::new(cmd_vec[0]).
            args(&cmd_vec[1..]).
            spawn().
            expect("fail to spawn the child command");

        child_cmd.wait().expect("fail to execute the command");
    }
}
