use crate::chunk::Chunk;
use crate::errors::{FileError, ImageError, ParsingError};
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
    pub fn new<P>(path: P) -> Result<Image, ImageError>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path).map_err(|_| fe::FileNotFound)?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)
            .map_err(|_| fe::FailedToRead)?;
        match Image::is_png(&buffer) {
            true => Ok(Image {
                chunks: Image::parse(&buffer)?,
            }),
            false => Err(fe::NotAPng.into()),
        }
    }

    fn parse(bytes: &[u8]) -> Result<Vec<Chunk>, ParsingError> {
        if bytes.len() < 16 {
            return Err(pe::SizeTooSmall);
        }

        let mut offset = 0;
        let mut chunks = vec![];

        while offset < bytes.len() {
            let length = u32::from_be_bytes(
                bytes[offset..offset + LENGTH_SIZE_OFFSET]
                    .try_into()
                    .map_err(|_| pe::InvalidLength)?,
            ) as usize;

            let class = u32::from_be_bytes(
                bytes[offset + LENGTH_SIZE_OFFSET..offset + TYPE_SIZE_OFFSET]
                    .try_into()
                    .map_err(|_| pe::InvalidType)?,
            );

            let data =
                bytes[offset + TYPE_SIZE_OFFSET..offset + TYPE_SIZE_OFFSET + length].to_vec();

            let crc = u32::from_be_bytes(
                bytes[offset + TYPE_SIZE_OFFSET + length..offset + CRC_SIZE_OFFSET + length]
                    .try_into()
                    .map_err(|_| pe::InvalidCrc)?,
            );

            let mut crc32 = Hasher::new();
            crc32.update(&class.to_be_bytes());
            crc32.update(&data);
            let finalized_crc = crc32.finalize();
            if crc != finalized_crc {
                return Err(pe::CrcMismatch);
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
