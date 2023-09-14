use crate::chunk::Chunk;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::ops::Index;
use std::path::Path;

#[derive(Debug)]
pub struct Image {
    chunks: Vec<Chunk>,
}

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const LENGTH_SIZE_OFFSET: usize = 4;
const TYPE_SIZE_OFFSET: usize = 8;
const CRC_SIZE_OFFSET: usize = 12;

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

use FileError as e;
impl Image {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Image, FileError> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(e::FileNotFound),
        };

        let mut buffer = vec![];

        match file.read_to_end(&mut buffer) {
            Ok(_) => (),
            Err(_) => return Err(e::FailedToRead),
        };

        match &Self::is_png(&buffer) {
            true => Ok(Image {
                chunks: Self::parse(&buffer[PNG_SIGNATURE.len()..]),
            }),
            false => Err(e::NotAPng),
        }
    }

    fn parse(bytes: &[u8]) -> Vec<Chunk> {
        if bytes.len() < 16 {
            panic!("Bytes length is less than 16");
        }

        let mut offset = 0;
        let mut chunks = vec![];

        while offset < bytes.len() {
            let chunk_length = u32::from_be_bytes(
                bytes[offset..offset + LENGTH_SIZE_OFFSET]
                    .try_into()
                    .unwrap_or_else(|error| {
                        panic!("Failed to convert chunk length bytes to u32: {}", error)
                    }),
            ) as usize;
            chunks.push(Chunk {
                length: chunk_length,
                class: u32::from_be_bytes(
                    bytes[offset + LENGTH_SIZE_OFFSET..offset + TYPE_SIZE_OFFSET]
                        .try_into()
                        .unwrap_or_else(|error| {
                            panic!("Failed to convert chunk type bytes to u32: {}", error)
                        }),
                ),
                data: bytes[offset + TYPE_SIZE_OFFSET..offset + TYPE_SIZE_OFFSET + chunk_length]
                    .to_vec(),
                crc: u32::from_be_bytes(
                    bytes[offset + TYPE_SIZE_OFFSET + chunk_length
                        ..offset + CRC_SIZE_OFFSET + chunk_length]
                        .try_into()
                        .unwrap_or_else(|error| {
                            panic!("Failed to convert chunk crc bytes to u32: {}", error)
                        }),
                ),
            });
            offset += CRC_SIZE_OFFSET + chunk_length;
        }

        chunks
    }

    fn is_png(data: &[u8]) -> bool {
        data.starts_with(&PNG_SIGNATURE)
    }
}

impl Index<usize> for Image {
    type Output = Chunk;

    fn index(&self, index: usize) -> &Self::Output {
        &self.chunks[index]
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image: chunks: {:?}", self.chunks)
    }
}
