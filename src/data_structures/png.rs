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
    ihdr: IHDRChunk,
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
                ChunkType::IHDR => match ihdr {
                    Some(_) => return Err(pe::MultipleIhdr),
                    None => ihdr = Some(IHDRChunk::from_chunk(&chunk)?),
                },
                ChunkType::PLTE => match plte {
                    Some(_) => return Err(pe::MultiplePlte),
                    None => plte = Some(chunk),
                },
                ChunkType::IDAT => idat.push(chunk),
                ChunkType::IEND => match iend {
                    Some(_) => return Err(pe::MultipleIend),
                    None => iend = Some(chunk),
                },
                _ => ancillary_chunks.push(chunk),
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
        // check if the first 8 bytes are the PNG signature, the second chunk is IHDR and the last chunk is IEND
        data.len() >= 16
            && data[..8] == PNG_SIGNATURE
            && data[8..16] == [0, 0, 0, 13, 73, 72, 68, 82]
    }

    pub fn get_ihdr(&self) -> &IHDRChunk {
        &self.ihdr
    }
}

impl Display for Png {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-- PNG information --")?;
        writeln!(f, "File name: {}", "TODO")?;
        writeln!(f, "File size: {}", "TODO")?;
        writeln!(f)?;

        writeln!(f, "-- IHDR chunk information--")?;
        writeln!(f, "{}", self.ihdr)?;

        writeln!(f, "-- PLTE chunk information--")?;
        writeln!(f, "PLTE: {:?}", self.plte)?;
        writeln!(f)?;

        writeln!(f, "-- IDAT chunks information --")?;
        writeln!(f, "Number of chunks: {}", self.idat.len())?;
        writeln!(f, "Average chunk size: {}", "TODO")?;
        writeln!(f)?;

        writeln!(f, "-- IEND  --")?;
        writeln!(f, "IEND: {}", self.iend)?;
        writeln!(f)?;

        writeln!(f, "-- Ancillary chunks --")?;
        writeln!(f, "Number of chunks: {}", self.ancillary_chunks.len())?;
        Ok(())
    }
}
