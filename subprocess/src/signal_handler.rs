use signal_hook::iterator::Signals;
use std::error::Error;
use std::thread::{self, sleep};
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>>{
    let signals = Signals::new(&[
        signal_hook::SIGINT,
        signal_hook::SIGTERM,
    ])?;
    thread::spawn(|| {
        loop {
            println!("child thread will sleep for 1 second");
            sleep(Duration::from_secs(1));
        }
    });
    'signal_loop: loop {
        for signal in signals.pending() {
            match signal {
                signal_hook::SIGINT => {
                    println!("received SIGINT, but nothing special will happen.")
                },
                signal_hook::SIGTERM => {
                    println!("received SIGTERM, the program will be terminated.");
                    break 'signal_loop;
                },
                _ => unreachable!(), 
            }
        }
    }
    Ok(())
}
