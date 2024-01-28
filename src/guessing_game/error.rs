use std::{error, fmt, io, num};

#[derive(Debug)]
pub enum GuessingGameError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
    IsNotInRange,
    IsLower,
    IsHigher
}

impl fmt::Display for GuessingGameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         let message = match self {
             GuessingGameError::IoError(err) => format!("I/O Error: {err}"),
             GuessingGameError::ParseError(err) => format!("Parsing Error: {err}"),
             GuessingGameError::IsNotInRange => format!("Guess not in range"),
             GuessingGameError::IsLower => format!("Guess is lower than the hidden value"),
             GuessingGameError::IsHigher => format!("Guess is higher than the hidden value")
         };
         write!(f, "{message}")
    }
}

impl error::Error for GuessingGameError {}

impl From<io::Error> for GuessingGameError {
    fn from(err: io::Error) -> Self {
        GuessingGameError::IoError(err)
    }
}

impl From<num::ParseIntError> for GuessingGameError {
    fn from(err: num::ParseIntError) -> Self {
        GuessingGameError::ParseError(err)
    }
}