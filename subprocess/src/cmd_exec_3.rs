use std::io::{self, Write, Error as ioErr, ErrorKind};
use std::process::Command;
use std::error::Error;

macro_rules! box_io_err {
    ($err_kind:expr, $err_msg:expr) => {
        Err(Box::new(ioErr::new($err_kind, $err_msg)))
    };
}

fn main() -> Result<(), Box<dyn Error>>{
    loop {
        print!("$ ");
        io::stdout().flush()?;
        let mut raw_inp = String::new();
        io::stdin().read_line(&mut raw_inp)?;
        let inp_stat = raw_inp.trim(); 
        let inp_stat_vec = inp_stat.split_whitespace().collect::<Vec<&str>>();
        if inp_stat_vec.len() == 0 {
            return box_io_err!(ErrorKind::InvalidInput, "the input can't be empty");
        }

        let mut child_cmd = match inp_stat_vec[0] {
            "show" if inp_stat_vec.len() > 1 => match inp_stat_vec[1]{
                    "files" | "file" => {
                        Command::new("ls").spawn().
                            expect("fail to spawn the child command `ls`")
                    },
                    "process" => {
                        Command::new("ps").spawn().
                            expect("fail to spawn the child command `ls`")
                    },
                    subcmd @ _ => {
                        return box_io_err!(ErrorKind::InvalidInput, 
                            format!("unsupport subcommand {}, supported subcommands are: file, files and process", subcmd))
                    },
            },
            "show" if inp_stat_vec.len() == 1 => {
                return box_io_err!(ErrorKind::InvalidInput, "subcommand can't be empty");
            },
            action @ _ => {
                return box_io_err!(ErrorKind::InvalidInput, 
                    format!("unsupport action {}, supported actions are: show", action));
            },
        };

        child_cmd.wait().expect(&format!("fail to execute the command {}", inp_stat));
        
    }
}
