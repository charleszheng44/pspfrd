use std::fmt;
use std::convert::From;
use std::io;

#[derive(Debug)]
pub struct StatsError (String);

impl fmt::Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "{}", self.0)
    }
}

impl From<io::Error> for StatsError {
    fn from(io_err: io::Error) -> Self {
        StatsError(io_err.to_string())
    }
}
