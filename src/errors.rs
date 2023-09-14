use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum FileError {
    FileNotFound,
    FailedToRead,
    NotAPng,
}

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::FileNotFound => write!(f, "File not found"),
            FileError::FailedToRead => write!(f, "Failed to read file"),
            FileError::NotAPng => write!(f, "File is not a PNG"),
        }
    }
}

impl Error for FileError {}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ParsingError {
    SizeTooSmall,
    InvalidLength,
    InvalidType,
    InvalidCrc,
    CrcMismatch,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::SizeTooSmall => write!(f, "Buffer size too small"),
            ParsingError::InvalidLength => write!(f, "Invalid chunk length"),
            ParsingError::InvalidType => write!(f, "Invalid chunk type"),
            ParsingError::InvalidCrc => write!(f, "Invalid chunk crc"),
            ParsingError::CrcMismatch => write!(f, "Chunk crc mismatch"),
        }
    }
}

impl Error for ParsingError {}
