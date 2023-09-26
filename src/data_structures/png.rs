use crate::prelude::*;
use crc32fast::Hasher;
use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::Read,
    path::Path,
};
use FileError as fe;
use ParsingError as pe;
#[derive(Debug)]
pub struct Png {
    ihdr: Chunk,
    plte: Option<Chunk>,
    idat: Vec<Chunk>,
    iend: Chunk,
    ancillary_chunks: Vec<Chunk>,
}

impl Png {
    pub fn new<P>(path: P) -> Result<Png, ImageError>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path).map_err(|_| fe::FileNotFound)?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)
            .map_err(|_| fe::FailedToRead)?;
        match Png::is_valid(&buffer) {
            true => Ok(Png::parse(&buffer[8..])?),
            false => Err(fe::NotAPng.into()),
        }
    }

    fn parse(bytes: &[u8]) -> Result<Png, ParsingError> {
        if bytes.len() < 16 {
            return Err(pe::SizeTooSmall);
        }

        let mut offset = 0;

        let mut ihdr = None;
        let mut plte = None;
        let mut idat = vec![];
        let mut iend = None;
        let mut ancillary_chunks = vec![];

        let hasher = Hasher::new();

        while offset < bytes.len() {
            let chunk = Chunk::parse(&bytes[offset..], hasher.clone())?;
            offset += CRC_SIZE_OFFSET + chunk.get_length();
            match chunk.get_class() {
                ChunkType::IHDR => {
                    if ihdr.is_some() {
                        return Err(pe::MultipleIhdr);
                    }
                    ihdr = Some(chunk);
                }
                ChunkType::PLTE => {
                    if plte.is_some() {
                        return Err(pe::MultiplePlte);
                    }
                    plte = Some(chunk);
                }
                ChunkType::IDAT => {
                    idat.push(chunk);
                }
                ChunkType::IEND => {
                    if iend.is_some() {
                        return Err(pe::MultipleIend);
                    }
                    iend = Some(chunk);
                }
                _ => {
                    ancillary_chunks.push(chunk);
                }
            }
        }

        Ok(Png {
            ihdr: ihdr.ok_or(pe::MissingIHDR)?,
            plte,
            idat,
            iend: iend.ok_or(pe::MissingIEND)?,
            ancillary_chunks,
        })
    }

    pub fn is_valid(data: &[u8]) -> bool {
        data.starts_with(&PNG_SIGNATURE)
    }

    pub fn get_ihdr(&self) -> Result<IHDRChunk, ParsingError> {
        let data = self.ihdr.get_data();
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

impl Display for Png {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "IHDR: {}", self.ihdr)?;
        writeln!(f, "PLTE: {:?}", self.plte)?;
        writeln!(f, "IDAT: {}", self.idat.len())?;
        writeln!(f, "IEND: {}", self.iend)?;
        writeln!(
            f,
            "Ancillary Chunks: {:?} ({})",
            self.ancillary_chunks
                .iter()
                .map(|c| c.get_class().as_str())
                .collect::<Vec<String>>(),
            self.ancillary_chunks.len()
        )?;
        Ok(())
    }
}
