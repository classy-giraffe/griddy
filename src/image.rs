use crate::chunk::Chunk;
use crate::errors::{FileError, ParsingError};
use crc32fast::Hasher;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::ops::{Deref, Index};
use std::path::Path;

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const LENGTH_SIZE_OFFSET: usize = 4;
const TYPE_SIZE_OFFSET: usize = 8;
const CRC_SIZE_OFFSET: usize = 12;

#[derive(Debug)]
pub struct Image {
    chunks: Vec<Chunk>,
}

use FileError as fe;
use ParsingError as pe;
impl Image {
    pub fn new<P>(path: P) -> Result<Image, FileError>
    where
        P: AsRef<Path>,
    {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(fe::FileNotFound),
        };

        let mut buffer = vec![];

        match file.read_to_end(&mut buffer) {
            Ok(_) => (),
            Err(_) => return Err(fe::FailedToRead),
        };

        match &Self::is_png(&buffer) {
            true => Ok(Image {
                chunks: Self::parse(&buffer[8..]).unwrap(),
            }),
            false => Err(fe::NotAPng),
        }
    }

    fn parse(bytes: &[u8]) -> Result<Vec<Chunk>, ParsingError> {
        if bytes.len() < 16 {
            return Err(pe::InvalidLength);
        }

        let mut offset = 0;
        let mut chunks = vec![];

        while offset < bytes.len() {
            let length = match bytes[offset..offset + LENGTH_SIZE_OFFSET].try_into() {
                Ok(length) => u32::from_be_bytes(length),
                Err(_) => return Err(pe::InvalidLength),
            } as usize;

            let class =
                match bytes[offset + LENGTH_SIZE_OFFSET..offset + TYPE_SIZE_OFFSET].try_into() {
                    Ok(class) => u32::from_be_bytes(class),
                    Err(_) => return Err(pe::InvalidType),
                };

            let data =
                bytes[offset + TYPE_SIZE_OFFSET..offset + TYPE_SIZE_OFFSET + length].to_vec();

            let crc = match bytes[offset + TYPE_SIZE_OFFSET + length
                ..offset + CRC_SIZE_OFFSET + length]
                .try_into()
            {
                Ok(crc) => u32::from_be_bytes(crc),
                Err(_) => return Err(pe::InvalidCrc),
            };

            let mut crc32 = Hasher::new();
            crc32.update(&class.to_be_bytes());
            crc32.update(&data);
            let finalized_crc = crc32.finalize();
            match finalized_crc == crc {
                true => (),
                false => return Err(pe::CrcMismatch),
            }

            chunks.push(Chunk::new(length, class, data, crc));
            offset += CRC_SIZE_OFFSET + length;
        }

        Ok(chunks)
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

impl Deref for Image {
    type Target = Vec<Chunk>;

    fn deref(&self) -> &Self::Target {
        &self.chunks
    }
}

impl Iterator for Image {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunks.pop()
    }
}
