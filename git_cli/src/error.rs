use std::io;
use std::fmt;

pub enum RegitError {
    IoError(io::Error),
    NoDirectory,
    InvalidCommit,
    InvalidIndex,
}

impl fmt::Display for RegitError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &RegitError::IoError(ref e) => e.fmt(formatter),
            &RegitError::NoDirectory => formatter.write_str("No directory found"),
            &RegitError::InvalidCommit => formatter.write_str("Invalid commit"),
            &RegitError::InvalidIndex => formatter.write_str("Invalid index (corrupt)"),
        }
    }
}

impl From<io::Error> for RegitError {
    fn from(error: io::Error) -> RegitError {
        RegitError::IoError(error)
    }
}