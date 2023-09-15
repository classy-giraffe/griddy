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

use crate::parsed_chunks::IHDRChunk;
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
                chunks: Image::parse(&buffer[8..])?,
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
            let chunk = Image::parse_chunk(&bytes[offset..])?;
            offset += CRC_SIZE_OFFSET + chunk.get_length();
            chunks.push(chunk);
        }
        Ok(chunks)
    }

    fn parse_chunk(bytes: &[u8]) -> Result<Chunk, ParsingError> {
        let length = u32::from_be_bytes(
            bytes[..LENGTH_SIZE_OFFSET]
                .try_into()
                .map_err(|_| pe::InvalidLength)?,
        ) as usize;

        let class = u32::from_be_bytes(
            bytes[LENGTH_SIZE_OFFSET..TYPE_SIZE_OFFSET]
                .try_into()
                .map_err(|_| pe::InvalidType)?,
        );

        let data = bytes[TYPE_SIZE_OFFSET..TYPE_SIZE_OFFSET + length].to_vec();

        let crc = u32::from_be_bytes(
            bytes[TYPE_SIZE_OFFSET + length..CRC_SIZE_OFFSET + length]
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

        Ok(Chunk::new(length, class, data, crc))
    }

    pub fn is_png(data: &[u8]) -> bool {
        data.starts_with(&PNG_SIGNATURE)
    }

    pub fn ihdr_parse(&self) -> IHDRChunk {
        let ihdr = &self[0];
        let data = ihdr.get_data();
        IHDRChunk::new(
            (
                u32::from_be_bytes(data[..4].try_into().unwrap()),
                u32::from_be_bytes(data[4..8].try_into().unwrap()),
            ),
            data[8],
            data[9],
            data[10],
            data[11],
            data[12],
        )
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
