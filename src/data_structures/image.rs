use crate::prelude::*;
use crc32fast::Hasher;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::ops::Index;
use std::path::Path;
use FileError as fe;
use ParsingError as pe;

#[derive(Debug, PartialEq, Eq)]
pub struct Image {
    chunks: Box<[Chunk]>,
}

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
                chunks: Image::parse(&buffer[8..])?,
            }),
            false => Err(fe::NotAPng.into()),
        }
    }

    fn parse(bytes: &[u8]) -> Result<Box<[Chunk]>, ParsingError> {
        if bytes.len() < 16 {
            return Err(pe::SizeTooSmall);
        }

        let mut offset = 0;
        let mut chunks = vec![];
        let hasher = Hasher::new();

        while offset < bytes.len() {
            let chunk = Chunk::parse(&bytes[offset..], hasher.clone())?;
            offset += CRC_SIZE_OFFSET + chunk.get_length();
            chunks.push(chunk);
        }
        Ok(chunks.into_boxed_slice())
    }

    pub fn is_png(data: &[u8]) -> bool {
        data.starts_with(&PNG_SIGNATURE)
    }

    pub fn get_ihdr(&self) -> Result<IHDRChunk, ParsingError> {
        let ihdr = &self[0];
        let data = ihdr.get_data();
        Ok(IHDRChunk::new(
            (
                u32::from_be_bytes(data[..4].try_into().map_err(|_| pe::InvalidData)?),
                u32::from_be_bytes(data[4..8].try_into().map_err(|_| pe::InvalidData)?),
            ),
            ColorLayout::from_bit_depth_and_color_type(data[8], data[9]),
            data[10],
            data[11],
            data[12],
        ))
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

impl IntoIterator for Image {
    type Item = Chunk;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.chunks.into_vec().into_iter()
    }
}
