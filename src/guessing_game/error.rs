use std::{io, num};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GuessingGameError {
    #[error("I/O Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Parsing Error: {0}")]
    ParseError(#[from] num::ParseIntError),
    #[error("Guess not in range")]
    IsNotInRange,
    #[error("Guess is lower than the hidden value")]
    IsLower,
    #[error("Guess is higher than the hidden value")]
    IsHigher,
}
