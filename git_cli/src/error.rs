use std::{fmt::Debug, io};
use std::fmt;

pub enum GitError {
    IoError(io::Error),
    NoDirectory,
    InvalidCommit,
    InvalidIndex,
}

impl fmt::Display for GitError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &GitError::IoError(ref e) => e.fmt(formatter),
            &GitError::NoDirectory => formatter.write_str("No directory found")
        }
    }
}